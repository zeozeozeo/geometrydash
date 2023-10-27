use winapi::shared::minwindef::FARPROC;

use crate::{cstr, impl_addr_funcs, Ptr};

use super::get_hmod;

/// Class that creates and handle the main Window and manages how
/// and when to execute the Scenes.
///  
/// The CCDirector is also responsible for:
/// - initializing the OpenGL context
/// - setting the OpenGL pixel format (default on is RGB565)
/// - setting the OpenGL buffer depth (default one is 0-bit)
/// - setting the projection (default one is 3D)
/// - setting the orientation (default one is Portrait)
///  
/// Since the CCDirector is a singleton, the standard way to use it is by calling:
///   
/// ```no_run
/// CCDirector::shared().method_name();
/// ````
///  
/// The CCDirector also sets the default OpenGL context:
/// - GL_TEXTURE_2D is enabled
/// - GL_VERTEX_ARRAY is enabled
/// - GL_COLOR_ARRAY is enabled
/// - GL_TEXTURE_COORD_ARRAY is enabled
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CCDirector {
    address: Ptr,
}

impl CCDirector {
    pub const fn from_address(address: Ptr) -> Self {
        Self { address }
    }

    /// Get the shared [`CCDirector`].
    pub fn shared() -> Self {
        unsafe {
            let address = (std::mem::transmute::<FARPROC, unsafe extern "cdecl" fn() -> usize>(
                winapi::um::libloaderapi::GetProcAddress(
                    get_hmod(),
                    cstr!("?sharedDirector@CCDirector@cocos2d@@SAPAV12@XZ"),
                ),
            ))();
            Self { address }
        }
    }

    /// Get the animation interval value.
    pub fn get_animation_interval(&self) -> f64 {
        unsafe {
            (std::mem::transmute::<FARPROC, unsafe extern "cdecl" fn(Ptr) -> f64>(
                winapi::um::libloaderapi::GetProcAddress(
                    get_hmod(),
                    cstr!("?getAnimationInterval@CCDirector@cocos2d@@QAENXZ"),
                ),
            ))(self.address)
        }
    }
}

impl_addr_funcs!(CCDirector);
