use std::sync::RwLock;

#[macro_use]
extern crate lazy_static;

extern crate rustwlc;

use rustwlc::interface::*;
use rustwlc::handle::{WlcView, WlcOutput};
use rustwlc::types::*;

struct Compositor {
    pub view: Option<WlcView>,
    pub grab: Point,
    pub edges: ResizeEdge
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

    println!("{} is preparing to render {} views", output.get_name(), views.len());

    for view in views {
        println!("\tView \"{}\"", view.get_title());
        println!("\tGeometry: {:?}", view.get_geometry());
        println!("\tIt's state: {}", view.get_state());
        view.set_geometry(ResizeEdge::empty(), &Geometry {
            size: Size { w: 0u32, h: 0u32 },
            origin: Point { x: 0, y: 0 }
        });
        println!("\tAtempted geometry, got {:?}", view.get_geometry());
    }

    //println!("Rendered {} views for output {}", views.len(), output.get_name());
}

// Handles

extern fn on_output_resolution(output: WlcOutput, from: &Size, to: &Size) {
    render_output(&output);
}

extern fn on_view_created(view: WlcView) -> bool {
    println!("View created: {:?}: {}", &view, view.get_class());
    view.set_mask(view.get_output().get_mask());
    view.bring_to_front();
    view.focus();
    render_output(&(view).get_output());
    true
}

extern fn on_view_destroyed(view: WlcView) {
    println!("View destroyed: {:?}: {}", &view, view.get_class());
    render_output(&(view).get_output());
}

extern fn on_view_focus(view: WlcView, focused: bool) {
    println!("View {} focused: {}. Setting state...", view.get_title(), focused);
    view.set_state(VIEW_ACTIVATED, focused);
    println!("View state: {}", view.get_state());
}

extern fn on_view_request_move(view: WlcView, origin: &Point) {
    start_interactive_move(&view, origin);
}

extern fn on_view_request_resize(view: WlcView, edges: ResizeEdge, origin: &Point) {
    start_interactive_resize(&view, edges, origin);
}

extern fn on_keyboard_key(view: WlcView, time: u32, mods: &KeyboardModifiers, key: u32, state: KeyState) -> bool {
    use std::process::Command;
    println!("Keyboard press on {:?}, with mods {:?} and key {} {:?}", view, mods, key, state);
    if state == KeyState::Pressed {
        if mods.mods == MOD_CTRL {
            println!("Checking for keys...");
            if key == 67 {
                println!("Handling kill window");
                if view.is_some() {
                    view.close();
                }
                return true;
            }
            else if key == 66 { // Execute order 66
                // TODO I will make a dezombifying thread
                let child = Command::new("/bin/weston-terminal").spawn()
                    .unwrap_or_else(|e| { println!("Error spawning child: {}", e); panic!("spawning child")});
                return true;
            }
        }
    }
    return false;
}

extern fn on_pointer_button(view: WlcView, time: u32, mods: &KeyboardModifiers,
                            button: u32, state: ButtonState, point: &Point) -> bool {
    println!("pointer_button: pressed {} at {} with view {:?}", button, point, view);
    if state == ButtonState::Pressed {
        if view.is_some() && mods.mods.contains(MOD_CTRL) { //view.0 != 0 {
            view.focus(); // Again may cause problems with no Some<View>
            if mods.mods.contains(MOD_CTRL) {
                // Button left, we need to include linux/input.h somehow
                if button == 0x110 {
                    println!("Preparing interative move...");
                    start_interactive_move(&view, point);
                }
                if button == 0x111 {
                    println!("Preparing interactive resize...");
                    start_interactive_resize(&view, ResizeEdge::empty(), point);
                }
            }
        }
        else {
            stop_interactive_action();
        }
    }
    else { return true; }

    {
        let comp = COMPOSITOR.read().unwrap();
        return comp.view.is_some();
    }
}

extern fn on_pointer_motion(in_view: WlcView, time: u32, point: &Point) -> bool {
    rustwlc::input::pointer::set_position(point);
    {
        let comp = COMPOSITOR.read().unwrap();
        if let Some(ref view) = comp.view {
                println!("Pointer motion for moving view!");
                let dx = point.x - comp.grab.x;
                let dy = point.y - comp.grab.y;
                println!("Moving by (x,y): ({}, {})", dx, dy);
                let mut geo = view.get_geometry().clone();
                println!("\tView's current geometry: {:?}", geo);
                if comp.edges.bits() != 0 {
                    let min = Size { w: 80u32, h: 40u32};
                    let mut new_geo = geo.clone();
                    println!("\tCloned geometry: {:?}", new_geo);

                    if comp.edges.contains(RESIZE_LEFT) {
                        println!("ResizeEdge::Left detected");
                        new_geo.size.w -= dx.abs() as u32;
                        new_geo.origin.x += dx;
                    }
                    else if comp.edges.contains(RESIZE_RIGHT) {
                        println!("ResizeEdge::Right detected");
                        new_geo.size.w += dx.abs() as u32;
                    }

                    if comp.edges.contains(RESIZE_TOP) {
                        new_geo.size.h -= dy.abs() as u32;
                        new_geo.origin.y += dy;
                    }
                    else if comp.edges.contains(RESIZE_BOTTOM) {
                        new_geo.size.h += dy.abs() as u32; // Here
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

    // We need a fucking builder
    let interface = WlcInterface {
        output: OutputInterface {
            created: None, destroyed: None, focus: None,

            resolution: Some(on_output_resolution),

            render: OutputRenderInterface { pre: None, post: None }
        },
        view: ViewInterface {
            created: Some(on_view_created),
            destroyed: Some(on_view_destroyed),
            focus: Some(on_view_focus),

            move_to_output: None,

            request: RequestInterface {
                geometry: None, state: None,

                move_: Some(on_view_request_move),
                resize: Some(on_view_request_resize),

                render: ViewRenderInterface { pre: None, post: None }
            }
        },

        keyboard: KeyboardInterface { key: Some(on_keyboard_key) },

        pointer: PointerInterface {
            button: Some(on_pointer_button),
            scroll: None,
            motion: Some(on_pointer_motion)
        },

        touch: TouchInterface { touch: None },

        compositor: CompositorInterface { ready: None },

        input: InputInterface { created: None, destroyed: None }
    };

    if !(rustwlc::init(interface)) {
        panic!("Unable to initialize!");
    }

    rustwlc::run_wlc();
}

