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

extern fn start_interactive_action(view: &WlcView, origin: &Point) -> bool {
    true
}

extern fn start_interactive_move(view: &WlcView, origin: &Point) {
    
}

extern fn start_interactive_resize(view: &WlcView, edges: u32, origin: &Point) {
    
}

extern fn stop_interactive_action() {
    
}

extern fn get_topmost_view(output: &WlcOutput, offset: usize) -> Option<WlcView> {
    None
}

extern fn render_output(output: &WlcOutput) {
    
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

