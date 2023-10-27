pub mod cocos2d;
mod game_manager;
mod game_object;
pub mod hook;
mod level_settings;
mod play_layer;
mod player_object;

#[cfg(feature = "fmod")]
pub mod fmod;

pub use game_manager::*;
pub use game_object::*;
pub use hook::*;
pub use level_settings::*;
pub use play_layer::*;
pub use player_object::*;
pub use winapi;

pub type Ptr = usize;

/// GetModuleHandle(NULL)
#[inline]
pub fn get_base() -> Ptr {
    unsafe { winapi::um::libloaderapi::GetModuleHandleA(std::ptr::null()) as Ptr }
}

/// Returns the pointer for a given address
pub const fn read_mem<T>(address: Ptr) -> *mut T {
    address as _
}

/// Reads a pointer at a given address
#[inline]
pub unsafe fn read_ptr(address: Ptr) -> Ptr {
    *read_mem(address)
}

/// Copies the given data to the given address in memory.
#[inline]
pub fn patch_mem(address: Ptr, data: &[u8]) {
    unsafe {
        let mut old_prot = 0u32;
        winapi::um::memoryapi::VirtualProtect(
            address as _,
            data.len(),
            winapi::um::winnt::PAGE_EXECUTE_READWRITE,
            &mut old_prot, // old protection
        );
        winapi::um::winnt::RtlCopyMemory(address as _, data.as_ptr() as _, data.len());
        winapi::um::memoryapi::VirtualProtect(address as _, data.len(), old_prot, 0 as _);
    }
}

pub trait AddressUtils {
    /// Returns the underlying pointer.
    fn ptr(&self) -> Ptr;

    /// Returns whether the underlying pointer is true or not.
    fn is_null(&self) -> bool;

    /// Returns None if underlying pointer is null, else returns Some(Self).
    fn to_option(self) -> Option<Self>
    where
        Self: Sized;
}

#[macro_export]
macro_rules! impl_addr_funcs {
    ($t:ty) => {
        impl $crate::AddressUtils for $t {
            #[inline(always)]
            fn ptr(&self) -> $crate::Ptr {
                self.address
            }

            #[inline(always)]
            fn is_null(&self) -> bool {
                self.address == 0
            }

            #[inline(always)]
            fn to_option(self) -> Option<Self> {
                if self.is_null() {
                    None
                } else {
                    Some(self)
                }
            }
        }

        impl Into<$crate::Ptr> for $t {
            fn into(self) -> $crate::Ptr {
                self.address
            }
        }
    };
}

#[macro_export]
macro_rules! impl_get_set {
    ($varname:ident, $set_varname:ident, $typ:ty, $addr:expr) => {
        #[doc = stringify!(Reads $varname ($typ). addr: $addr.)]
        #[inline(always)]
        pub fn $varname(&self) -> $typ {
            unsafe { *$crate::read_mem(self.address + $addr) }
        }

        #[doc = stringify!(Writes $varname ($typ). addr: $addr.)]
        #[inline(always)]
        pub fn $set_varname(&self, $varname: $typ) {
            unsafe { *$crate::read_mem(self.address + $addr) = $varname }
        }
    };
}

/// Converts a string to a C string (by appending `\0` to the end).
#[macro_export]
macro_rules! cstr {
    ($str:expr) => {
        concat!($str, "\0").as_ptr() as _
    };
}

/// Generates a Windows `DllMain` entrypoint.
/// The entrypoint function will be marked as `unsafe`.
///
/// If you want the app to open with a console, you must call
/// [`winapi`]'s `AllocConsole()` (with the `consoleapi` feature)
/// and `FreeConsole()` at the start and at the end of the entrypoint.
///
/// Only available on Windows targets.
///
/// `hmod` is a [`Ptr`] so it can be shared between threads.
///
/// Example:
///
/// ```no_run
/// use geometrydash::*;
///
/// dllmain_entrypoint! {hmod, {
///     println!("DLL injected!");
///     // FreeLibraryAndExitThread is called automatically,
///     // DWORD(0) exit code gets returned
/// }}
/// ```
#[cfg(windows)]
#[macro_export]
macro_rules! dllmain_entrypoint {
    ($hmod:ident, $body:block) => {
        #[no_mangle]
        unsafe extern "system" fn on_attach(
            $hmod: $crate::Ptr,
        ) -> $crate::winapi::shared::minwindef::DWORD {
            $body
            $crate::winapi::um::libloaderapi::FreeLibraryAndExitThread(
                $hmod as $crate::winapi::shared::minwindef::HMODULE,
                0,
            );
            0
        }

        #[allow(non_snake_case)]
        #[no_mangle]
        extern "system" fn DllMain(
            hmod: $crate::winapi::shared::minwindef::HMODULE,
            reason: $crate::winapi::shared::minwindef::DWORD,
            _: $crate::winapi::shared::minwindef::LPVOID,
        ) -> $crate::winapi::shared::minwindef::BOOL {
            if reason == $crate::winapi::um::winnt::DLL_PROCESS_ATTACH {
                unsafe {
                    $crate::winapi::um::libloaderapi::DisableThreadLibraryCalls(hmod);
                    let handle = $crate::winapi::um::processthreadsapi::CreateThread(
                        0 as $crate::winapi::um::minwinbase::LPSECURITY_ATTRIBUTES,
                        0,
                        Some(::std::mem::transmute::<unsafe extern "system" fn(Ptr) -> u32, unsafe extern "system" fn(*mut winapi::ctypes::c_void) -> u32>(on_attach)),
                        hmod as $crate::winapi::shared::minwindef::LPVOID,
                        0,
                        0 as $crate::winapi::shared::minwindef::LPDWORD,
                    );
                    if handle != $crate::winapi::um::handleapi::INVALID_HANDLE_VALUE {
                        $crate::winapi::um::handleapi::CloseHandle(handle);
                    } else {
                        return $crate::winapi::shared::minwindef::FALSE;
                    }
                }
            }
            $crate::winapi::shared::minwindef::TRUE
        }
    }
}
