use crate::{
    fmod::{FMOD_CHANNEL, FMOD_SYSTEM},
    get_base, impl_addr_funcs, impl_get_set, Ptr,
};
use std::ffi::c_void;

pub struct FMODAudioEngine {
    address: Ptr,
}

impl FMODAudioEngine {
    pub fn shared() -> Self {
        Self {
            address: unsafe {
                std::mem::transmute::<Ptr, unsafe extern "stdcall" fn() -> Ptr>(
                    get_base() + 0x239f0,
                )()
            },
        }
    }

    impl_get_set!(system, set_system, *mut FMOD_SYSTEM, 0x128);
    impl_get_set!(
        current_sound_channel,
        set_current_sound_channel,
        *mut FMOD_CHANNEL,
        0x130
    );
    impl_get_set!(extra_driver_data, set_extra_driver_data, *mut c_void, 0x140);
}

impl_addr_funcs!(FMODAudioEngine);
