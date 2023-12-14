use crate::{impl_addr_funcs, Ptr};
use windows::Win32::Foundation::FARPROC;

use super::get_hmod;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CCScheduler {
    address: Ptr,
}

impl CCScheduler {
    pub const fn from_address(address: Ptr) -> Self {
        Self { address }
    }

    pub fn get_timescale(&self) -> f32 {
        unsafe {
            (std::mem::transmute::<FARPROC, unsafe extern "thiscall" fn(Ptr) -> f32>(
                windows::Win32::System::LibraryLoader::GetProcAddress(
                    get_hmod(),
                    windows::core::s!("?getTimeScale@CCScheduler@cocos2d@@QAEMXZ"),
                ),
            ))(self.address)
        }
    }
}

impl_addr_funcs!(CCScheduler);
