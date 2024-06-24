use bevy::prelude::Res;
use objc::{msg_send, sel, sel_impl};
use rand::{distributions::Alphanumeric, Rng};

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
pub fn setup_traffic_light_positioner(decorum_settings: Res<crate::decorum::DecorumSettings>) {
    use cocoa::appkit::NSWindow;
    use cocoa::base::{id, BOOL};
    use cocoa::foundation::NSUInteger;
    use objc::runtime::{Object, Sel};
    use std::ffi::c_void;

    if let Some(primary_window_id) = &decorum_settings.primary_window_id {
        unsafe {
            let ns_window: id = std::mem::transmute(*primary_window_id);

            let current_delegate: id = ns_window.delegate();
        }
    } else {
        panic!("Primary window ID is not set.");
    }
}
