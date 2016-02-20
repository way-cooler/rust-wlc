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
    pub edges: u32
}

lazy_static! {
    static ref COMPOSITOR: RwLock<Compositor> =
        RwLock::new(Compositor { view: None, grab: Point { x: 0, y: 0 }, edges: 0 });
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

fn start_interactive_resize(view: &WlcView, edges: u32, origin: &Point) {
    let geometry = view.get_geometry().unwrap();

    if !start_interactive_action(view, origin) {
        return;
    }
    let halfw = geometry.origin.x + geometry.size.w as i32 / 2;
    let halfh = geometry.origin.y + geometry.size.h as i32 / 2;

    {
        let mut comp = COMPOSITOR.write().unwrap();
        comp.edges = edges.clone();
        if comp.edges == 0 {
            let x = if origin.x < halfw {
                ResizeEdge::Left as u32
            } else if origin.x > halfw {
                ResizeEdge::Right as u32
            } else {
                ResizeEdge::None as u32
            };

            let y = if origin.y < halfh {
                ResizeEdge::Top as u32
            } else if origin.y > halfh {
                ResizeEdge::Bottom as u32
            } else {
                ResizeEdge::None as u32
            };

            comp.edges = x | y;
        }
    }
    view.set_state(ViewState::Resizing, true);
}

fn stop_interactive_action() {
    let mut comp = COMPOSITOR.write().unwrap();

    match comp.view {
        None => return,
        Some(ref view) =>
            view.set_state(ViewState::Resizing, false)
    }

    (*comp).view = None;
}

fn get_topmost_view(output: &WlcOutput, offset: usize) -> Option<WlcView> {
    let views = output.get_views();
    if views.is_empty() { None }
    else {
        Some(views[(views.len() - 1 + offset) % views.len()].clone())
    }
}

fn render_output(output: &WlcOutput) {
    let views = output.get_views();
    if views.is_empty() { return; }


}

// Handles

extern fn on_output_resolution(output: WlcOutput, from: &Size, to: &Size) {
    
}

extern fn on_view_created(view: WlcView) -> bool {
    true
}

extern fn on_view_destroyed(view: WlcView) {
    
}

extern fn on_view_focus(view: WlcView, focused: bool) {
    
}

extern fn on_view_request_move(view: WlcView, origin: &Point) {
    
}

extern fn on_view_request_resize(view: WlcView, edges: ResizeEdge, origin: &Point) {
    
}

extern fn on_keyboard_key(view: WlcView, time: u32, mods: &KeyboardModifiers, key: u32, state: KeyState) -> bool {
    true
}

extern fn on_pointer_motion(view: WlcView, time: u32, point: &Point) {
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
            button: None, scroll: None,
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

