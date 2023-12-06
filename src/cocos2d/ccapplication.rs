use super::get_hmod;
use crate::Ptr;
use windows::Win32::Foundation::FARPROC;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CCApplication {
    address: Ptr,
}

impl CCApplication {
    pub const fn from_address(address: Ptr) -> Self {
        Self { address }
    }

    /// Get the shared [`CCApplication`].
    pub fn shared() -> Self {
        unsafe {
            let address = (std::mem::transmute::<FARPROC, unsafe extern "cdecl" fn() -> Ptr>(
                windows::Win32::System::LibraryLoader::GetProcAddress(
                    get_hmod(),
                    windows::core::s!("?sharedApplication@CCApplication@cocos2d@@SAPAV12@XZ"),
                ),
            ))();
            Self { address }
        }
    }

    /// Set the animation interval value.
    pub fn set_animation_interval(&self, interval: f64) {
        unsafe {
            (std::mem::transmute::<FARPROC, unsafe extern "fastcall" fn(Ptr, Ptr, f64)>(
                windows::Win32::System::LibraryLoader::GetProcAddress(
                    get_hmod(),
                    windows::core::s!("?setAnimationInterval@CCApplication@cocos2d@@UAEXN@Z"),
                ),
            ))(self.address, 0, interval)
        }
    }
}

crate::impl_addr_funcs!(CCApplication);
