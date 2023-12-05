pub mod cocos2d;
mod game_manager;
mod game_object;
mod level_settings;
mod play_layer;
mod player_object;

#[cfg(feature = "fmod")]
pub mod fmod;

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
#[inline]
pub fn patch_mem(address: Ptr, data: &[u8]) {
    use windows::Win32::System::Memory::VirtualProtect;
    use windows::Win32::System::Memory::{PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS};
    unsafe {
        let mut old_prot = PAGE_PROTECTION_FLAGS(0);
        VirtualProtect(
            address as _,
            data.len(),
            PAGE_EXECUTE_READWRITE,
            &mut old_prot, // old protection
        )
        .unwrap();
        std::ptr::copy_nonoverlapping(data.as_ptr(), address as _, data.len());
        VirtualProtect(address as _, data.len(), old_prot, 0 as _).unwrap();
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
