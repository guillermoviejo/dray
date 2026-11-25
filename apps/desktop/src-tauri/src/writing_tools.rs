//! Keeps macOS's "Write with Siri" pill away from the app's text fields.
//!
//! macOS 27 raises that pill beside the caret of an empty multi-line field
//! whose view answers `allowsWritingToolsAffordance` with yes, and `WKWebView`
//! says yes for every textarea and contenteditable. Nothing in the page turns
//! it off: `writingsuggestions="false"` is inline predictions, and
//! `WKWebViewConfiguration.writingToolsBehavior = .none` leaves the pill up,
//! both measured on 27.0.1. Writing Tools' own behaviour is left as it was.

use objc2::runtime::{AnyClass, AnyObject, Bool, Sel};
use objc2::sel;
use tauri::WebviewWindow;

extern "C" fn no_affordance(_this: &AnyObject, _sel: Sel) -> Bool {
    Bool::NO
}

/// Makes the window's webview decline the Writing Tools affordance.
pub fn decline_affordance(window: &WebviewWindow) {
    let _ = window.with_webview(|webview| {
        let view = webview.inner() as *const AnyObject;
        // SAFETY: `inner` is the live `WKWebView`, and this runs on the main thread.
        let Some(view) = (unsafe { view.as_ref() }) else { return };
        let class: *const AnyClass = view.class();
        // wry's own subclass, so this is an override on a class nobody else
        // uses. `class_addMethod` refuses a class that already implements the
        // selector, which is `WKWebView` itself should wry stop subclassing.
        unsafe {
            let imp: objc2::runtime::Imp = std::mem::transmute(no_affordance as *const ());
            objc2::ffi::class_addMethod(
                class as *mut AnyClass,
                sel!(allowsWritingToolsAffordance),
                imp,
                c"B@:".as_ptr(),
            );
        }
    });
}
