use std::ffi::CString;

use crate::{get_base, read_ptr, PlayLayer, Ptr};

// GameManager* Global_GameManager = base + 0x3222D0
// GameManager::getSharedState = base + 0xC4A50
// GameManager::getGameVariable = base + 0xC9D30
// GameManager.PlayLayer = GameManager + 0x164
// GameManager.EditorLayer = GameManager + 0x168
// GameManager.userName = GameManager + 0x198

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct GameManager {
    /// GameManager shared state
    address: Ptr,
}

impl Default for GameManager {
    /// GameManager::getSharedState
    fn default() -> Self {
        unsafe {
            let address = (std::mem::transmute::<Ptr, unsafe extern "stdcall" fn() -> Ptr>(
                get_base() + 0xC4A50,
            ))();
            Self { address }
        }
    }
}

impl GameManager {
    #[inline(always)]
    pub fn shared() -> Self {
        Self::default()
    }

    /// GameManager::getGameVariable
    #[inline]
    pub fn get_game_variable(&self, var: &str) -> bool {
        let var = CString::new(var).unwrap(); // convert to c string
        unsafe {
            (std::mem::transmute::<Ptr, unsafe extern "fastcall" fn(Ptr, Ptr, *const u8) -> bool>(
                get_base() + 0xC9D30,
            ))(self.address, 0, var.as_ptr() as *const u8)
        }
    }

    /// GameManager.PlayLayer
    #[inline(always)]
    pub fn play_layer(&self) -> PlayLayer {
        PlayLayer::from_address(unsafe { read_ptr(self.address + 0x164) })
    }

    /// GameManager.userName
    #[inline]
    pub fn user_name(&self) -> Result<String, std::str::Utf8Error> {
        unsafe {
            let cstr = CString::from_raw(read_ptr(self.address + 0x198) as _);
            match cstr.to_str() {
                Ok(s) => Ok(s.to_string()),
                Err(e) => Err(e),
            }
        }
    }
}

crate::impl_addr_funcs!(GameManager);
