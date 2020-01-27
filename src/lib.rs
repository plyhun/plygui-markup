extern crate plygui_api;

pub use plygui_api::controls::*;
pub use plygui_api::ids::*;
pub use plygui_api::types::*;
pub use plygui_api::callbacks;
pub use plygui_api::layout;
pub use plygui_api::utils;
pub use plygui_api::markup;

#[cfg(feature = "gtk3")]
extern crate plygui_gtk;
#[cfg(feature = "gtk3")]
pub use plygui_gtk::imp;
#[cfg(feature = "gtk3")]
pub use plygui_gtk::register_members;

#[cfg(feature = "qt5")]
extern crate plygui_qt;
#[cfg(feature = "qt5")]
pub use plygui_qt::imp;
#[cfg(feature = "qt5")]
pub use plygui_qt::register_members;

#[cfg(all(target_os = "macos", feature = "cocoa"))]
extern crate plygui_cocoa;
#[cfg(all(target_os = "macos", feature = "cocoa"))]
pub use plygui_cocoa::imp;
#[cfg(all(target_os = "macos", feature = "cocoa"))]
pub use plygui_cocoa::register_members;

#[cfg(all(target_os = "windows", feature = "win32"))]
extern crate plygui_win32;
#[cfg(all(target_os = "windows", feature = "win32"))]
pub use plygui_win32::imp;
#[cfg(all(target_os = "windows", feature = "win32"))]
pub use plygui_win32::register_members;

#[cfg(not(any(feature = "gtk3", feature = "cocoa", feature = "win32")))]
extern crate plygui_testable;
#[cfg(not(any(feature = "gtk3", feature = "cocoa", feature = "win32")))]
pub use plygui_testable::imp;
#[cfg(not(any(feature = "gtk3", feature = "cocoa", feature = "win32")))]
pub use plygui_testable::register_members;

pub fn register_markup_members(registry: &mut plygui_api::markup::MarkupRegistry) {
    register_members(registry);
}
