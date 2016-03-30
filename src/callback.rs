
#[link(name = "wlc")]
extern "C" {
    // Output was created. Return false if you want to destroy the output.
    // (e.g. failed to allocate data related to view)
    fn wlc_set_output_created_cb(extern "C" fn(WlcOutput) -> bool);

    // Output was destroyed.
    fn wlc_set_output_destroyed_cb(extern "C" fn(WlcOutput));

    // Output got or lost focus.
    fn wlc_set_output_focus_cb(extern "C" fn(WlcOutput, bool));

    // Output resolution changed.
    fn wlc_set_output_resolution_cb(extern "C" fn(WlcOutput, &Size, &Size));

    // Output pre render hook.
    fn wlc_set_output_render_pre_cb(extern "C" fn(WlcOutput));

    // Output post render hook.
    fn wlc_set_output_render_post_cb(extern "C" fn(WlcOutput));

    // View was created. Return false if you want to destroy the view.
    // (e.g. failed to allocate data related to view)
    fn wlc_set_view_created_cb(extern "C" fn(WlcView) -> bool);

    // View was destroyed.
    fn wlc_set_view_destroyed_cb(extern "C" fn(handle: WlcView));

    // View got or lost focus.
    fn wlc_set_view_focus_cb(extern "C" fn(WlcView, bool));

    // View was moved to output.
    fn wlc_set_view_move_to_output_cb(extern "C" fn(WlcView, WlcOutput,
                                                    WlcOutput));

    // Request to set given geometry for view.
    // Apply using wlc_view_set_geometry to agree.
    fn wlc_set_view_request_geometry_cb(extern "C" fn(WlcView, &Geometry));

    // Request to disable or enable the given state for view.
    // Apply using wlc_view_set_state to agree.
    fn wlc_set_view_request_state_cb(extern "C" fn(WlcView, ViewState, bool));

    // Request to move itself. Start a interactive move to agree.
    fn wlc_set_view_request_move_cb(extern "C" fn(WlcView, &Point));

    // Request to resize itself with the given edges.
    // Start a interactive resize to agree.
    fn wlc_set_view_request_resize_cb(extern "C" fn(WlcView, ResizeEdge,
                                                    &Point));

    // View pre render hook.
    fn wlc_set_view_render_pre_cb(extern "C" fn(WlcView));

    // View post render hook.
    fn wlc_set_view_render_post_cb(extern "C" fn(WlcView));

    // Key event was triggered, view handle will be zero if there was no focus.
    // Return true to prevent sending the event to clients.
    fn wlc_set_keyboard_key_cb(extern "C" fn(WlcView, u32, &KeyboardModifiers,
                                             u32, KeyState) -> bool);

    // Button event was triggered, view handle will be zero if there
    // was no focus. Return true to prevent sending the event to clients.
    fn wlc_set_pointer_button_cb(extern "C" fn(WlcView, u32, &KeyboardModifiers,
                                             u32, ButtonState, &Point) -> bool);

    // Scroll event was triggered, view handle will be zero if there was no
    // focus. Return true to prevent sending the event to clients.
    fn wlc_set_pointer_scroll_cb(extern "C" fn(WlcView, u32, &KeyboardModifiers,
                                               ScrollAxis, [u64; 2]) -> bool);

    // Motion event was triggered, view handle will be zero if there was no
    // focus. Apply with wlc_pointer_set_position to agree. Return true to
    // prevent sending the event to clients.
    fn wlc_set_pointer_motion_cb(extern "C" fn(WlcView, u32, &Point) -> bool);

    // Touch event was triggered, view handle will be zero if there was no
    // focus. Return true to prevent sending the event to clients.
    fn wlc_set_touch_cb(extern "C" fn(WlcView, u32, &KeyboardModifiers,
                                    TouchType, i32, &Point) -> bool);

    // Compositor is ready to accept clients.
    fn wlc_set_compositor_ready_cb(extern "C" fn());

    // Compositor is about to terminate.
    fn wlc_set_compositor_terminate_cb(extern "C" fn());

    // Input device was created. Return value does nothing. (Experimental)
    fn wlc_set_input_created_cb(extern "C" fn(&LibinputDevice) -> bool);

    // Input device was destroyed. (Experimental)
    fn wlc_set_input_destroyed_cb(extern "C" fn(&LibinputDevice));
}
