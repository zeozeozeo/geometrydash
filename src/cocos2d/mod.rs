mod ccapplication;
mod ccdirector;

pub use ccapplication::*;
pub use ccdirector::*;

use winapi::shared::minwindef::HMODULE;

/// Get the `libcocos2d.dll` module handle.
pub fn get_hmod() -> HMODULE {
    unsafe { winapi::um::libloaderapi::GetModuleHandleA(crate::cstr!("libcocos2d.dll")) }
}

/*
#[macro_export]
macro_rules! cocos2dx_procname {
    ($procname:literal($($($t:ty),+)?) $(-> $ret:ty)?) => {
        /* */
        unsafe {
            (::std::mem::transmute::<
                $crate::winapi::shared::minwindef::FARPROC,
                extern "cdecl" fn($($($t),+)?) $(-> $ret)?,
            >(winapi::um::libloaderapi::GetProcAddress(
                $crate::cocos2d::get_hmod(),
                $crate::cstr!($procname),
            )))
        }
    };
}
*/

/// This is ugly but who cares
#[macro_export]
macro_rules! cocos2dx_callproc (
    ($procname:literal($($($arg:expr => $t:ty),+)?) $(-> $ret:ty)?) => {
        unsafe {
            (::std::mem::transmute::<
                $crate::winapi::shared::minwindef::FARPROC,
                extern "cdecl" fn($($($t),+)?) $(-> $ret)?,
            >(winapi::um::libloaderapi::GetProcAddress(
                $crate::cocos2d::get_hmod(),
                $crate::cstr!($procname),
            )))($($($arg),+)?)
        }
    }
);
