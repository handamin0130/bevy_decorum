use bevy::prelude::Res;
use objc::{msg_send, sel, sel_impl};
use rand::{distributions::Alphanumeric, Rng};
use winit::window::WindowId;

const WINDOW_CONTROL_PAD_X: f64 = 8.0;
const WINDOW_CONTROL_PAD_Y: f64 = 12.0;

pub struct UnsafeWindowHandle(pub *mut std::ffi::c_void);
unsafe impl Send for UnsafeWindowHandle {}
unsafe impl Sync for UnsafeWindowHandle {}

#[cfg(target_os = "macos")]
pub fn positioner_traffic_light(unsafe_window_handle: UnsafeWindowHandle, x: f64, y: f64) {
    use cocoa::appkit::{NSView, NSWindow, NSWindowButton};
    use cocoa::foundation::NSRect;

    let unsafe_window = unsafe_window_handle.0 as cocoa::base::id;

    unsafe {
        let close = unsafe_window.standardWindowButton_(NSWindowButton::NSWindowCloseButton);
        let miniaturize =
            unsafe_window.standardWindowButton_(NSWindowButton::NSWindowMiniaturizeButton);
        let zoom = unsafe_window.standardWindowButton_(NSWindowButton::NSWindowZoomButton);

        let title_bar_container_view = close.superview().superview();

        let close_rect: NSRect = msg_send![close, frame];
        let button_height = close_rect.size.height;

        let title_bar_frame_height = button_height + y;
        let mut title_bar_rect = NSView::frame(title_bar_container_view);
        title_bar_rect.size.height = title_bar_frame_height;
        title_bar_rect.origin.y = NSView::frame(unsafe_window).size.height - title_bar_frame_height;
        let _: () = msg_send![title_bar_container_view, setFrame: title_bar_rect];

        let window_buttons = vec![close, miniaturize, zoom];
        let space_between = NSView::frame(miniaturize).origin.x - NSView::frame(close).origin.x;

        for (i, button) in window_buttons.into_iter().enumerate() {
            let mut rect: NSRect = NSView::frame(button);
            rect.origin.x = x + (i as f64 * space_between);
            button.setFrameOrigin(rect.origin);
        }
    }
}

#[cfg(target_os = "macos")]
#[derive(Debug)]
struct WindowState {
    primary_window_id: WindowId,
}

#[cfg(target_os = "macos")]
pub fn setup_traffic_light_positioner(decorum_settings: Res<crate::decorum::DecorumSettings>) {
    use cocoa::appkit::NSWindow;
    use cocoa::base::{id, BOOL};
    use cocoa::foundation::NSUInteger;
    use objc::runtime::{Object, Sel};
    use std::ffi::c_void;

    if let Some(primary_window_id) = &decorum_settings.primary_window_id {
        unsafe {
            let ns_window: id = std::mem::transmute(*primary_window_id);
            let ns_window_ptr: *mut c_void = ns_window as *mut c_void;

            positioner_traffic_light(
                UnsafeWindowHandle(ns_window_ptr),
                WINDOW_CONTROL_PAD_X,
                WINDOW_CONTROL_PAD_Y,
            );

            fn with_window_state<F: FnOnce(&mut WindowState) -> T, T>(this: &Object, func: F) {
                let ptr = unsafe {
                    let x: *mut c_void = *this.get_ivar("app_box");
                    &mut *(x as *mut WindowState)
                };
                func(ptr);
            }

            let current_delegate: id = ns_window.delegate();

            extern "C" fn on_window_should_close(this: &Object, _cmd: Sel, sender: id) -> BOOL {
                unsafe {
                    let super_del: id = *this.get_ivar("super_delegate");
                    msg_send![super_del, windowShouldClose: sender]
                }
            }
            extern "C" fn on_window_will_close(this: &Object, _cmd: Sel, notification: id) {
                unsafe {
                    let super_del: id = *this.get_ivar("super_delegate");
                    let _: () = msg_send![super_del, windowWillClose: notification];
                }
            }
            extern "C" fn on_window_did_resize(this: &Object, _cmd: Sel, notification: id) {
                unsafe {
                    with_window_state(this, |state: &mut WindowState| {
                        let ns_window: id = std::mem::transmute(state.primary_window_id);
                        let ns_window_ptr: *mut c_void = ns_window as *mut c_void;

                        #[cfg(target_os = "macos")]
                        positioner_traffic_light(
                            UnsafeWindowHandle(ns_window_ptr),
                            WINDOW_CONTROL_PAD_X,
                            WINDOW_CONTROL_PAD_Y,
                        );
                    });

                    let super_del: id = *this.get_ivar("super_delegate");
                    let _: () = msg_send![super_del, windowDidResize: notification];
                }
            }
            extern "C" fn on_window_did_move(this: &Object, _cmd: Sel, notification: id) {
                unsafe {
                    let super_del: id = *this.get_ivar("super_delegate");
                    let _: () = msg_send![super_del, windowDidMove: notification];
                }
            }
            extern "C" fn on_window_did_change_backing_properties(
                this: &Object,
                _cmd: Sel,
                notification: id,
            ) {
                unsafe {
                    let super_del: id = *this.get_ivar("super_delegate");
                    let _: () =
                        msg_send![super_del, windowDidChangeBackingProperties: notification];
                }
            }
            extern "C" fn on_window_did_become_key(this: &Object, _cmd: Sel, notification: id) {
                unsafe {
                    let super_del: id = *this.get_ivar("super_delegate");
                    let _: () = msg_send![super_del, windowDidBecomeKey: notification];
                }
            }
            extern "C" fn on_window_did_resign_key(this: &Object, _cmd: Sel, notification: id) {
                unsafe {
                    let super_del: id = *this.get_ivar("super_delegate");
                    let _: () = msg_send![super_del, windowDidResignKey: notification];
                }
            }
            extern "C" fn on_dragging_entered(this: &Object, _cmd: Sel, notification: id) -> BOOL {
                unsafe {
                    let super_del: id = *this.get_ivar("super_delegate");
                    msg_send![super_del, draggingEntered: notification]
                }
            }
            extern "C" fn on_prepare_for_drag_operation(
                this: &Object,
                _cmd: Sel,
                notification: id,
            ) -> BOOL {
                unsafe {
                    let super_del: id = *this.get_ivar("super_delegate");
                    msg_send![super_del, prepareForDragOperation: notification]
                }
            }
            extern "C" fn on_perform_drag_operation(this: &Object, _cmd: Sel, sender: id) -> BOOL {
                unsafe {
                    let super_del: id = *this.get_ivar("super_delegate");
                    msg_send![super_del, performDragOperation: sender]
                }
            }
            extern "C" fn on_conclude_drag_operation(this: &Object, _cmd: Sel, notification: id) {
                unsafe {
                    let super_del: id = *this.get_ivar("super_delegate");
                    let _: () = msg_send![super_del, concludeDragOperation: notification];
                }
            }
            extern "C" fn on_dragging_exited(this: &Object, _cmd: Sel, notification: id) {
                unsafe {
                    let super_del: id = *this.get_ivar("super_delegate");
                    let _: () = msg_send![super_del, draggingExited: notification];
                }
            }
            extern "C" fn on_window_will_use_full_screen_presentation_options(
                this: &Object,
                _cmd: Sel,
                window: id,
                proposed_options: NSUInteger,
            ) -> NSUInteger {
                unsafe {
                    let super_del: id = *this.get_ivar("super_delegate");
                    msg_send![super_del, window: window willUseFullScreenPresentationOptions: proposed_options]
                }
            }
            // TODO
            extern "C" fn on_window_did_enter_full_screen(
                this: &Object,
                _cmd: Sel,
                notification: id,
            ) {
                unsafe {
                    with_window_state(this, |_state: &mut WindowState| {});

                    let super_del: id = *this.get_ivar("super_delegate");
                    let _: () = msg_send![super_del, windowDidEnterFullScreen: notification];
                }
            }
            // TODO
            extern "C" fn on_window_will_enter_full_screen(
                this: &Object,
                _cmd: Sel,
                notification: id,
            ) {
                unsafe {
                    with_window_state(this, |_state: &mut WindowState| {});

                    let super_del: id = *this.get_ivar("super_delegate");
                    let _: () = msg_send![super_del, windowWillEnterFullScreen: notification];
                }
            }
            extern "C" fn on_window_did_exit_full_screen(
                this: &Object,
                _cmd: Sel,
                notification: id,
            ) {
                unsafe {
                    with_window_state(this, |_state: &mut WindowState| {});

                    let super_del: id = *this.get_ivar("super_delegate");
                    let _: () = msg_send![super_del, windowDidExitFullScreen: notification];
                }
            }
            // TODO
            extern "C" fn on_window_will_exit_full_screen(
                this: &Object,
                _cmd: Sel,
                notification: id,
            ) {
                unsafe {
                    with_window_state(this, |_state: &mut WindowState| {});

                    let super_del: id = *this.get_ivar("super_delegate");
                    let _: () = msg_send![super_del, windowWillExitFullScreen: notification];
                }
            }
            extern "C" fn on_window_did_fail_to_enter_full_screen(
                this: &Object,
                _cmd: Sel,
                window: id,
            ) {
                unsafe {
                    let super_del: id = *this.get_ivar("super_delegate");
                    let _: () = msg_send![super_del, windowDidFailToEnterFullScreen: window];
                }
            }
            extern "C" fn on_effective_appearance_did_change(
                this: &Object,
                _cmd: Sel,
                notification: id,
            ) {
                unsafe {
                    let super_del: id = *this.get_ivar("super_delegate");
                    let _: () = msg_send![super_del, effectiveAppearanceDidChange: notification];
                }
            }
            extern "C" fn on_effective_appearance_did_changed_on_main_thread(
                this: &Object,
                _cmd: Sel,
                notification: id,
            ) {
                unsafe {
                    let super_del: id = *this.get_ivar("super_delegate");
                    let _: () = msg_send![
                        super_del,
                        effectiveAppearanceDidChangedOnMainThread: notification
                    ];
                }
            }

            let app_state = WindowState {
                primary_window_id: *primary_window_id,
            };
            let app_box = Box::into_raw(Box::new(app_state)) as *mut c_void;
            let random_str: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(20)
                .map(char::from)
                .collect();

            let delegate_name = format!("windowDelegate_{}", random_str);

            ns_window.setDelegate_(cocoa::delegate!(&delegate_name, {
                window: id = ns_window,
                app_box: *mut c_void = app_box,
                toolbar: id = cocoa::base::nil,
                super_delegate: id = current_delegate,
                (windowShouldClose:) => on_window_should_close as extern fn(&Object, Sel, id) -> BOOL,
                (windowWillClose:) => on_window_will_close as extern fn(&Object, Sel, id),
                (windowDidResize:) => on_window_did_resize as extern fn(&Object, Sel, id),
                (windowDidMove:) => on_window_did_move as extern fn(&Object, Sel, id),
                (windowDidChangeBackingProperties:) => on_window_did_change_backing_properties as extern fn(&Object, Sel, id),
                (windowDidBecomeKey:) => on_window_did_become_key as extern fn(&Object, Sel, id),
                (windowDidResignKey:) => on_window_did_resign_key as extern fn(&Object, Sel, id),
                (draggingEntered:) => on_dragging_entered as extern fn(&Object, Sel, id) -> BOOL,
                (prepareForDragOperation:) => on_prepare_for_drag_operation as extern fn(&Object, Sel, id) -> BOOL,
                (performDragOperation:) => on_perform_drag_operation as extern fn(&Object, Sel, id) -> BOOL,
                (concludeDragOperation:) => on_conclude_drag_operation as extern fn(&Object, Sel, id),
                (draggingExited:) => on_dragging_exited as extern fn(&Object, Sel, id),
                (window:willUseFullScreenPresentationOptions:) => on_window_will_use_full_screen_presentation_options as extern fn(&Object, Sel, id, NSUInteger) -> NSUInteger,
                (windowDidEnterFullScreen:) => on_window_did_enter_full_screen as extern fn(&Object, Sel, id),
                (windowWillEnterFullScreen:) => on_window_will_enter_full_screen as extern fn(&Object, Sel, id),
                (windowDidExitFullScreen:) => on_window_did_exit_full_screen as extern fn(&Object, Sel, id),
                (windowWillExitFullScreen:) => on_window_will_exit_full_screen as extern fn(&Object, Sel, id),
                (windowDidFailToEnterFullScreen:) => on_window_did_fail_to_enter_full_screen as extern fn(&Object, Sel, id),
                (effectiveAppearanceDidChange:) => on_effective_appearance_did_change as extern fn(&Object, Sel, id),
                (effectiveAppearanceDidChangedOnMainThread:) => on_effective_appearance_did_changed_on_main_thread as extern fn(&Object, Sel, id)
            }))
        }
    } else {
        panic!("Primary window ID is not set.");
    }
}
