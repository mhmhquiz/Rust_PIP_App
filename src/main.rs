extern crate winapi;

use std::ptr::null_mut;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;

use winapi::um::winuser::{
    FindWindowW, SetWindowPos, HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE
};
use winapi::shared::minwindef::BOOL;

fn main() {
    let app_name = ""; // 最前面に固定したいアプリケーション名を指定

    let wide_app_name: Vec<u16> = OsStr::new(app_name)
        .encode_wide()
        .chain(once(0))
        .collect();

    unsafe {
        let hwnd = FindWindowW(null_mut(), wide_app_name.as_ptr());
        if hwnd.is_null() {
            println!("アプリケーションが見つかりませんでした");
        } else {
            set_top_most(hwnd, true);
        }
    }
}

unsafe fn set_top_most(hwnd: winapi::shared::windef::HWND, top_most: bool) -> BOOL {
    let flags = SWP_NOMOVE | SWP_NOSIZE;
    let z_order = if top_most { HWND_TOPMOST } else { winapi::um::winuser::HWND_NOTOPMOST };

    SetWindowPos(hwnd, z_order, 0, 0, 0, 0, flags)
}
