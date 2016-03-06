use std::sync::RwLock;

#[macro_use]
extern crate lazy_static;

extern crate rustwlc;

use rustwlc::*;
use rustwlc::interface::*;
use rustwlc::handle::{WlcView, WlcOutput};
use rustwlc::types::*;
use rustwlc::input::keyboard;

use std::cmp;

struct Compositor {
    pub view: Option<WlcView>,
    pub grab: Point,
    pub edges: ResizeEdge
}

enum KeySym {
    KeyQ = 0x0071,
    KeyDown = 0xff54,
    KeyEsc = 0xff1b,
    KeyReturn = 0xff0d
}

lazy_static! {
    static ref COMPOSITOR: RwLock<Compositor> =
        RwLock::new(Compositor { view: None,
                                 grab: Point { x: 0, y: 0 },
                                 edges: ResizeEdge::empty() });
}

fn start_interactive_action(view: &WlcView, origin: &Point) -> bool {
    {
        let mut comp = COMPOSITOR.write().unwrap();
        if comp.view != None {
            return false;
        }
        comp.grab = origin.clone();
        comp.view = Some(view.clone());
    }

    view.bring_to_front();
    return true;
}

fn start_interactive_move(view: &WlcView, origin: &Point) {
    start_interactive_action(view, origin);
}

fn start_interactive_resize(view: &WlcView, edges: ResizeEdge, origin: &Point) {
    let geometry = view.get_geometry();

    if !start_interactive_action(view, origin) {
        return;
    }
    let halfw = geometry.origin.x + geometry.size.w as i32 / 2;
    let halfh = geometry.origin.y + geometry.size.h as i32 / 2;

    {
        let mut comp = COMPOSITOR.write().unwrap();
        comp.edges = edges.clone();
        if comp.edges.bits() == 0 {
            let flag_x = if origin.x < halfw {
                RESIZE_LEFT
            } else if origin.x > halfw {
                RESIZE_RIGHT
            } else {
                ResizeEdge::empty()
            };

            let flag_y = if origin.y < halfh {
                RESIZE_TOP
            } else if origin.y > halfh {
                RESIZE_BOTTOM
            } else {
                ResizeEdge::empty()
            };

            comp.edges = flag_x | flag_y;
        }
    }
    view.set_state(VIEW_RESIZING, true);
}

fn stop_interactive_action() {
    let mut comp = COMPOSITOR.write().unwrap();

    match comp.view {
        None => return,
        Some(ref view) =>
            view.set_state(VIEW_RESIZING, false)
    }

    (*comp).view = None;
    comp.edges = ResizeEdge::empty();
}

fn get_topmost_view(output: &WlcOutput, offset: usize) -> Option<WlcView> {
    let views = output.get_views();
    if views.is_empty() { None }
    else {
        Some(views[(views.len() - 1 + offset) % views.len()].clone())
    }
}

fn render_output(output: &WlcOutput) {
    let resolution = output.get_resolution();
    let views = output.get_views();
    if views.is_empty() { return; }

    let mut toggle = false;
    let mut y = 0;
    let w = resolution.w / 2;
    let h = resolution.h / cmp::max((views.len() + 1) / 2, 1) as u32;
    for (i, view) in views.iter().enumerate() {
        view.set_geometry(ResizeEdge::empty(), &Geometry {
            origin: Point { x: if toggle { w as i32 } else { 0 }, y: y },
            size: Size { w: if !toggle && i == views.len() - 1 { resolution.w } else { w }, h: h }
        });
        toggle = ! toggle;
        y = if y > 0 || !toggle { h as i32 } else { 0 };
    }
}

// Handles

extern fn on_output_resolution(output: WlcOutput, _from: &Size, _to: &Size) {
    render_output(&output);
}

extern fn on_view_created(view: WlcView) -> bool {
    view.set_mask(view.get_output().get_mask());
    view.bring_to_front();
    view.focus();
    render_output(&(view).get_output());
    true
}

extern fn on_view_destroyed(view: WlcView) {
    if let Some(top_view) = get_topmost_view(&view.get_output(), 0) {
        top_view.focus();
    }
    render_output(&view.get_output());
}

extern fn on_view_focus(view: WlcView, focused: bool) {
    view.set_state(VIEW_ACTIVATED, focused);
}

extern fn on_view_request_move(view: WlcView, origin: &Point) {
    start_interactive_move(&view, origin);
}

extern fn on_view_request_resize(view: WlcView, edges: ResizeEdge, origin: &Point) {
    start_interactive_resize(&view, edges, origin);
}

extern fn on_keyboard_key(view: WlcView, _time: u32, mods: &KeyboardModifiers, key: u32, state: KeyState) -> bool {
    use std::process::Command;
    let sym = keyboard::get_keysym_for_key(key, &MOD_NONE);
    if state == KeyState::Pressed {
        if mods.mods == MOD_CTRL {
            // Key Q
            if sym == KeySym::KeyQ as u32 {
                if !view.is_root() {
                    view.close();
                }
                return true;
            // Down key
            } else if sym == KeySym::KeyDown as u32 {
                view.send_to_back();
                get_topmost_view(&view.get_output(), 0).unwrap().focus();
                return true;
            // Esc Key
            } else if sym == KeySym::KeyEsc as u32 {
                terminate();
                return true;
            // Return key
            } else if sym == KeySym::KeyReturn as u32 {
                let _ = Command::new("sh")
                                .arg("-c")
                                .arg("/usr/bin/weston-terminal || echo a").spawn()
                    .unwrap_or_else(|e| {
                        println!("Error spawning child: {}", e); panic!("spawning child")});
                return true;
            }
        }
    }
    return false;
}

extern fn on_pointer_button(view: WlcView, _time: u32, mods: &KeyboardModifiers,
                            button: u32, state: ButtonState, point: &Point) -> bool {
    if state == ButtonState::Pressed {
        if !view.is_root() && mods.mods.contains(MOD_CTRL) {
            view.focus();
            if mods.mods.contains(MOD_CTRL) {
                // Button left, we need to include linux/input.h somehow
                if button == 0x110 {
                    start_interactive_move(&view, point);
                }
                if button == 0x111 {
                    start_interactive_resize(&view, ResizeEdge::empty(), point);
                }
            }
        }
    }
    else {
        stop_interactive_action();
    }

    {
        let comp = COMPOSITOR.read().unwrap();
        return comp.view.is_some();
    }
}
extern fn on_pointer_motion(_in_view: WlcView, _time: u32, point: &Point) -> bool {
    rustwlc::input::pointer::set_position(point);
    {
        let comp = COMPOSITOR.read().unwrap();
        if let Some(ref view) = comp.view {
                let dx = point.x - comp.grab.x;
                let dy = point.y - comp.grab.y;
                let mut geo = view.get_geometry().clone();
                if comp.edges.bits() != 0 {
                    let min = Size { w: 80u32, h: 40u32};
                    let mut new_geo = geo.clone();

                    if comp.edges.contains(RESIZE_LEFT) {
                        if dx < 0 {
                            new_geo.size.w += dx.abs() as u32;
                        } else {
                            new_geo.size.w -= dx.abs() as u32;
                        }
                        new_geo.origin.x += dx;
                    }
                    else if comp.edges.contains(RESIZE_RIGHT) {
                        if dx < 0 {
                            new_geo.size.w -= dx.abs() as u32;
                        } else {
                            new_geo.size.w += dx.abs() as u32;
                        }
                    }

                    if comp.edges.contains(RESIZE_TOP) {
                        if dy < 0 {
                            new_geo.size.h += dy.abs() as u32;
                        } else {
                            new_geo.size.h -= dy.abs() as u32;
                        }
                        new_geo.origin.y += dy;
                    }
                    else if comp.edges.contains(RESIZE_BOTTOM) {
                        if dy < 0 {
                            new_geo.size.h -= dy.abs() as u32;
                        } else {
                            new_geo.size.h += dy.abs() as u32;
                        }
                    }

                    if new_geo.size.w >= min.w {
                        geo.origin.x = new_geo.origin.x;
                        geo.size.w = new_geo.size.w;
                    }

                    if new_geo.size.h >= min.h {
                        geo.origin.y = new_geo.origin.y;
                        geo.size.h = new_geo.size.h;
                    }

                    view.set_geometry(comp.edges, &geo);
                }
                else {
                    geo.origin.x += dx;
                    geo.origin.y += dy;
                    view.set_geometry(ResizeEdge::empty(), &geo);
                }
        }
    }

    {
        let mut comp = COMPOSITOR.write().unwrap();
        comp.grab = point.clone();
        return comp.view.is_some();
    }
}

fn main() {
    let interface = WlcInterface::new()
        .output_resolution(on_output_resolution)
        .view_created(on_view_created)
        .view_destroyed(on_view_destroyed)
        .view_focus(on_view_focus)
        .view_request_move(on_view_request_move)
        .view_request_resize(on_view_request_resize)
        .keyboard_key(on_keyboard_key)
        .pointer_button(on_pointer_button)
        .pointer_motion(on_pointer_motion);

    rustwlc::log_set_default_handler();
    let run_fn = rustwlc::init(interface).expect("Unable to initialize!");
    run_fn();
}

