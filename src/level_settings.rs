use crate::{impl_addr_funcs, read_mem, Ptr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct LevelSettings {
    address: Ptr,
}

impl LevelSettings {
    #[inline(always)]
    pub fn from_address(address: Ptr) -> Self {
        Self { address }
    }

    #[inline(always)]
    pub fn is_2player(&self) -> bool {
        unsafe { *read_mem(self.address + 0xFA) }
    }
}

impl_addr_funcs!(LevelSettings);
