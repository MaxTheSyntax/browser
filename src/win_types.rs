#![allow(non_snake_case)]

pub fn LOWORD(l: u32) -> u16 {
    l as u16
}
pub fn HIWORD(l: u32) -> u16 {
    (l >> 16) as u16
}
