//! Dummy wlc callbacks for events.
#![allow(missing_docs)]

use super::types::*;
use ::{WlcOutput, WlcView};


pub fn output_created(_callback: extern "C" fn(output: WlcOutput) -> bool) {
    unimplemented!()
}

pub fn output_destroyed(_callback: extern "C" fn(output: WlcOutput)) {
    unimplemented!()
}

pub fn output_focus(_callback: extern "C" fn(output: WlcOutput, focused: bool)) {
    unimplemented!()
}

pub fn output_resolution(_callback: extern "C" fn(output: WlcOutput,
                                                 old_size: &Size,
                                                 new_size: &Size)) {
    unimplemented!()
}

pub fn output_context_destroyed(_cb: extern "C" fn(output: WlcOutput)) {
    unimplemented!()
}

pub fn output_context_created(_cb: extern "C" fn(output: WlcOutput)) {
    unimplemented!()
}

pub fn output_render_pre(_callback: extern "C" fn(output: WlcOutput)) {
    unimplemented!()
}

pub fn output_render_post(_callback: extern "C" fn(output: WlcOutput)) {
    unimplemented!()
}

pub fn view_created(_callback: extern "C" fn(view: WlcView) -> bool) {
    unimplemented!()
}

pub fn view_destroyed(_callback: extern "C" fn(view: WlcView)) {
    unimplemented!()
}

pub fn view_focus(_callback: extern "C" fn(handle: WlcView, focused: bool)) {
    unimplemented!()
}

pub fn view_move_to_output(_callback: extern "C" fn(view: WlcView,
                                                   old_output: WlcOutput,
                                                   new_output: WlcOutput)) {
    unimplemented!()
}

pub fn view_request_geometry(_callback: extern "C" fn(handle: WlcView,
                                                     geometry: &Geometry)) {
    unimplemented!()
}

pub fn view_request_state(_callback: extern "C" fn(current: WlcView,
                                                  state: ViewState,
                                                  handled: bool)) {
    unimplemented!()
}

pub fn view_request_move(_callback: extern "C" fn(handle: WlcView,
                                                 destination: &Point)) {
    unimplemented!()
}

pub fn view_request_resize(_callback: extern "C" fn(handle: WlcView,
                                                   edge: ResizeEdge,
                                                   location: &Point)) {
    unimplemented!()
}

pub fn view_render_pre(_callback: extern "C" fn(view: WlcView)) {
    unimplemented!()
}

pub fn view_render_post(_callback: extern "C" fn(view: WlcView)) {
    unimplemented!()
}

pub fn keyboard_key(_callback: extern "C" fn(view: WlcView, time: u32,
                                            mods: &KeyboardModifiers, key: u32,
                                            state: KeyState) -> bool) {
    unimplemented!()
}

pub fn pointer_button(_callback: extern "C" fn(view: WlcView, time: u32,
                                              mods: &KeyboardModifiers,
                                              button: u32, state: ButtonState,
                                              point: &Point) -> bool) {
    unimplemented!()
}

pub fn pointer_scroll(_callback: extern "C" fn(view: WlcView, time: u32,
                                              mods: &KeyboardModifiers,
                                              axis: ScrollAxis,
                                              amount: [f64; 2]) -> bool) {
    unimplemented!()
}

pub fn pointer_motion(_callback: extern "C" fn(view: WlcView, time: u32,
                                              point: &Point) -> bool) {
    unimplemented!()
}

pub fn touch(_callback: extern "C" fn(handle: WlcView, time: u32,
                                     mods: &KeyboardModifiers, touch: TouchType,
                                     slot: i32, point: &Point) -> bool) {
    unimplemented!()
}

pub fn compositor_ready(_callback: extern "C" fn()) {
    unimplemented!()
}

pub fn compositor_terminate(_callback: extern "C" fn()) {
    unimplemented!()
}

pub fn view_properties_changed(_callback: extern "C" fn(handle: WlcView, mask: ViewPropertyType)) {
    unimplemented!()
}
