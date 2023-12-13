pub mod cocos2d;
mod game_manager;
mod game_object;
mod level_settings;
mod play_layer;
mod player_object;

#[cfg(feature = "fmod")]
pub mod fmod;

#[cfg(feature = "fmod")]
mod fmod_audio_engine;

#[cfg(feature = "fmod")]
pub use fmod_audio_engine::*;

pub use game_manager::*;
pub use game_object::*;
pub use level_settings::*;
pub use play_layer::*;
pub use player_object::*;
pub use windows;

pub type Ptr = usize;

/// GetModuleHandle(NULL)
#[inline]
pub fn get_base() -> Ptr {
    // unsafe { winapi::um::libloaderapi::GetModuleHandleA(std::ptr::null()) as Ptr }
    // unsafe {}
    use windows::core::PCSTR;
    use windows::Win32::System::LibraryLoader::GetModuleHandleA;
    unsafe {
        let hmod = GetModuleHandleA(PCSTR(std::ptr::null())).unwrap();
        hmod.0 as Ptr
    }
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
pub fn patch_mem(address: Ptr, data: &[u8]) -> windows::core::Result<()> {
    use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
    use windows::Win32::System::Memory::{
        VirtualProtectEx, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS,
    };
    use windows::Win32::System::Threading::GetCurrentProcess;
    unsafe {
        let mut old_prot = PAGE_PROTECTION_FLAGS(0);
        VirtualProtectEx(
            GetCurrentProcess(),
            address as _,
            256,
            PAGE_EXECUTE_READWRITE,
            &mut old_prot as _,
        )?;
        WriteProcessMemory(
            GetCurrentProcess(),
            address as _,
            data.as_ptr() as _,
            data.len(),
            None,
        )
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

        impl From<$t> for $crate::Ptr {
            fn from(value: $t) -> $crate::Ptr {
                value.address
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
