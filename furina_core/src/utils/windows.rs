use std::ffi::{OsStr, OsString};
use std::iter::once;
use std::marker::PhantomPinned;
use std::mem::transmute;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::pin::{pin, Pin};
use std::ptr::{null, null_mut, slice_from_raw_parts_mut};
use std::sync::atomic::{AtomicPtr, Ordering};

use anyhow::{anyhow, Result};
use log::{info, warn};
use once_cell::sync::Lazy;
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::Graphics::Gdi::ClientToScreen;
use windows_sys::Win32::Security::*;
use windows_sys::Win32::System::LibraryLoader::*;
use windows_sys::Win32::System::SystemServices::*;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;
use windows_sys::Win32::UI::WindowsAndMessaging::*;

use crate::positioning::Rect;

pub fn encode_lpcstr(s: &str) -> Vec<u8> {
    let mut arr: Vec<u8> = s.bytes().collect();
    arr.push(0);
    arr
}

fn encode_wide_with_null(s: impl AsRef<str>) -> Vec<u16> {
    let wide: Vec<u16> = OsStr::new(s.as_ref()).encode_wide().chain(once(0)).collect();
    wide
}

pub fn find_window_local(title: impl AsRef<str>) -> Result<HWND> {
    let title = encode_wide_with_null(title);
    let class = encode_wide_with_null("UnityWndClass");
    let result: HWND = unsafe { FindWindowW(class.as_ptr(), title.as_ptr()) };
    if result.is_null() {
        Err(anyhow!("cannot find window"))
    } else {
        Ok(result)
    }
}

pub fn find_window_cloud() -> Result<HWND> {
    let title = encode_wide_with_null(String::from("云·原神"));
    //let class = encode_wide(String::from("Qt5152QWindowIcon"));
    unsafe {
        let mut result: HWND = null_mut();
        for _ in 0..3 {
            result = FindWindowExW(null_mut(), result, null_mut(), title.as_ptr());
            let exstyle = GetWindowLongPtrW(result, GWL_EXSTYLE);
            let style = GetWindowLongPtrW(result, GWL_STYLE);
            if exstyle == 0x0 && style == 0x96080000 {
                return Ok(result); //全屏
            } else if exstyle == 0x100 && style == 0x96CE0000 {
                return Ok(result); //窗口
            }
        }
    }
    Err(anyhow!("cannot find window"))
}

unsafe fn get_client_rect_unsafe(hwnd: HWND) -> Result<Rect<i32>> {
    let mut rect: RECT = RECT { left: 0, top: 0, right: 0, bottom: 0 };
    GetClientRect(hwnd, &mut rect);
    let width: i32 = rect.right;
    let height: i32 = rect.bottom;

    let mut point: POINT = POINT { x: 0, y: 0 };
    ClientToScreen(hwnd, &mut point as *mut POINT);
    let left: i32 = point.x;
    let top: i32 = point.y;

    Ok(Rect { left, top, width, height })
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn get_client_rect(hwnd: HWND) -> Result<Rect<i32>> {
    unsafe { get_client_rect_unsafe(hwnd) }
}

unsafe fn is_admin_unsafe() -> bool {
    let mut authority: SID_IDENTIFIER_AUTHORITY =
        SID_IDENTIFIER_AUTHORITY { Value: [0, 0, 0, 0, 0, 5] };
    let mut group: PSID = null_mut();
    let mut b = AllocateAndInitializeSid(
        &mut authority as *mut SID_IDENTIFIER_AUTHORITY,
        2,
        SECURITY_BUILTIN_DOMAIN_RID as u32,
        DOMAIN_ALIAS_RID_ADMINS as u32,
        0,
        0,
        0,
        0,
        0,
        0,
        &mut group as *mut PSID,
    );
    if b != 0 {
        let r = CheckTokenMembership(null_mut(), group, &mut b as *mut BOOL);
        if r == 0 {
            b = 0;
        }
        FreeSid(group);
    }

    b != 0
}

pub fn is_admin() -> bool {
    unsafe { is_admin_unsafe() }
}

/// 检测鼠标右键当前是否处于按下状态
///
/// 使用 Windows API GetAsyncKeyState 检测右键的实时状态。
///
/// # 返回值
/// - `true`: 右键当前正在被按下
/// - `false`: 右键当前未被按下
///
/// # GetAsyncKeyState 返回值说明
/// - 位0（最低位）: 表示按键在上次调用后是否被按下过（历史状态）
/// - 位15（最高位，0x8000）: 表示按键当前是否处于按下状态（实时状态）
///
/// # 安全性
/// 此函数使用 unsafe 块调用 Windows API，但在正常使用情况下是安全的。
pub fn is_rmb_down() -> bool {
    unsafe {
        let state = GetAsyncKeyState(VK_RBUTTON as i32);
        // 检测当前按键状态（位15）而非历史状态（位0）
        // GetAsyncKeyState 返回 i16，最高位为1时表示按键被按下
        // 使用负数检测更简洁，因为 i16 的最高位为符号位
        state < 0
    }
}

pub fn set_dpi_awareness() {
    let h_lib = unsafe {
        let utf16 = encode_lpcstr("Shcore.dll");
        LoadLibraryA(utf16.as_ptr())
    };
    if h_lib.is_null() {
        unsafe {
            SetProcessDPIAware();
        }
    } else {
        unsafe {
            let addr = GetProcAddress(h_lib, encode_lpcstr("SetProcessDpiAwareness").as_ptr());
            if let Some(proc) = addr {
                let func = transmute::<
                    unsafe extern "system" fn() -> isize,
                    unsafe extern "system" fn(usize) -> isize,
                >(proc);
                func(2);
            } else {
                warn!("cannot find process `SetProcessDpiAwareness`, but `Shcore.dll` exists");
                SetProcessDPIAware();
            }

            FreeLibrary(h_lib);
        }
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn show_window_and_set_foreground(hwnd: HWND) {
    unsafe {
        ShowWindow(hwnd, SW_RESTORE);
        SetForegroundWindow(hwnd);
    }
}

unsafe fn iterate_window_unsafe() -> Vec<HWND> {
    static ALL_HANDLES: Lazy<AtomicPtr<Vec<HWND>>> =
        Lazy::new(|| AtomicPtr::new(Box::into_raw(Box::new(Vec::new()))));

    extern "system" fn callback(hwnd: HWND, _vec_ptr: LPARAM) -> BOOL {
        unsafe {
            let handles = ALL_HANDLES.load(Ordering::Acquire);
            (*handles).push(hwnd);
        }
        1
    }

    unsafe {
        let handles = ALL_HANDLES.load(Ordering::Acquire);
        (*handles).clear();
        EnumWindows(Some(callback), 0);
        (*handles).clone()
    }
}

pub fn iterate_window() -> Vec<HWND> {
    unsafe { iterate_window_unsafe() }
}

unsafe fn get_window_title_unsafe(hwnd: HWND) -> Option<String> {
    let mut buffer: Vec<u16> = vec![0; 100];
    GetWindowTextW(hwnd, buffer.as_mut_ptr(), 100);

    let s = OsString::from_wide(&buffer);

    if let Ok(ss) = s.into_string() {
        let ss = ss.trim_matches(char::from(0));
        Some(String::from(ss))
    } else {
        None
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn get_window_title(hwnd: HWND) -> Option<String> {
    unsafe { get_window_title_unsafe(hwnd) }
}
