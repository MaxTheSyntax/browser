#![allow(non_snake_case)]

pub fn LOWORD(l: u32) -> u16 {
    l as u16
}
pub fn HIWORD(l: u32) -> u16 {
    (l >> 16) as u16
}

use windows::Win32::System::Com::CoTaskMemFree;
use std::ptr;
pub unsafe fn safe_mem_release<T>(ppT: *mut *mut T) {
    unsafe {
        if !(*ppT).is_null() {
            // SAFETY: Caller must ensure that ppT is valid and points to a COM object
            CoTaskMemFree(Some(*ppT as *mut _));
            *ppT = ptr::null_mut();
        }
    }
}
