mod ccapplication;
mod ccdirector;

pub use ccapplication::*;
pub use ccdirector::*;
use windows::Win32::Foundation::HMODULE;

/// Get the `libcocos2d.dll` module handle.
pub fn get_hmod() -> HMODULE {
    use windows::Win32::System::LibraryLoader::GetModuleHandleA;
    unsafe { GetModuleHandleA(windows::core::s!("libcocos2d.dll")).unwrap() }
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
            >(windows::Win32::System::LibraryLoader::GetProcAddress(
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
            >(windows::Win32::System::LibraryLoader::GetProcAddress(
                $crate::cocos2d::get_hmod(),
                $crate::cstr!($procname),
            )))($($($arg),+)?)
        }
    }
);
