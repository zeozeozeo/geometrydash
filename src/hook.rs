pub use minhook;

/// A macro to define a hook.
///
/// To make a hook, you have to provide a hook function and a trampoline
/// function. The hook function is a global variable that stores the function
/// type, and the trampoline function is the actual function. The third token
/// that you have to pass to the macro is the function's calling convention.
/// Even if the actual function calling convention is `thiscall` (if it's
/// a class method), you can't use `thiscall` outside of classes and function
/// types, so you have to approximate it by using `fastcall` instead, which
/// is done by making `this` (or `self` in Rust) be the first argument of the
/// function, and between the actual arguments, an unused argument which is
/// the `edx` register. You can define it like `_edx: usize`. For example,
/// if you have the following C++ function that you want to hook:
///
/// ```cpp
/// PlayLayer::init(GJGameLevel*) // GJ stands for Geometry Jump :)
/// ```
///
/// the `define_hook!(...)` macro would look like
///
/// ```no_run
/// use geometrydash::*;
/// hook::define_hook!(PLAYLAYER_INIT -> playlayer_init_h, "fastcall"(playlayer: Ptr, _edx: Ptr, level: Ptr));
/// ```
///
/// ([`crate::Ptr`] is a "safe" pointer type defined by this crate).
///
/// If the function doesn't have any arguments (besides `self`), the unused argument
/// for the `edx` register is not required, but you can still leave it there if you want.
///
/// P.S. Not every function is a `thiscall`, for example if it's a static function it
/// can be a `fastcall`, which is usually easier to deal with. However sometimes the
/// compiler optimizes them, and floats go into `xmm` registers, and in that case
/// you have to write some assembly to hook the function properly.
#[macro_export]
macro_rules! define_hook {
    ($def_name:ident -> $hook_name:ident, $callconv:literal($($($n:ident: $t:ty),+)?) $(-> $ret:ty)? $body:block) => {
        pub static mut $def_name: *mut std::ffi::c_void = 0 as _;

        pub unsafe extern $callconv fn $hook_name($($($n: $t),+)?) $(-> $ret)? {
            $body
        }
    }
}

/// Initializes the function hook (uses MinHook under the hood).
///
/// # Example:
///
/// ```no_run
/// use geometrydash::*;
///
/// // define PLAYLAYER_INIT hook...
/// hook::define_hook!(PLAYLAYER_INIT -> playlayer_init_h, "fastcall"(playlayer: Ptr, _edx: Ptr, level: Ptr));
///
/// // hook by memory address (0xDEADBEEF = function address in memory)
/// hook::create_hook!(PLAYLAYER_INIT -> playlayer_init_h @ 0xDEADBEEF);
/// // or (hook by base + address)...
/// hook::create_hook!(PLAYLAYER_INIT -> playlayer_init_h @ base + 0x1234); // gd base + 0x1234
/// // or... (hook cocos scheduler update by library and symbol name)
/// // make sure to define the SCHEDULE_UPDATE hook before calling this
/// hook::create_hook!(SCHEDULE_UPDATE -> schedule_update_h @ "libcocos2d.dll"."?update@CCScheduler@cocos2d@@UAEXM@Z");
/// ```
#[macro_export]
macro_rules! create_hook {
    // CLASSNAME_INIT -> classname_init_h @ 0xDEADBEEF
    ($hook_name:ident -> $def_name:ident @ $addr:literal) => {
        unsafe {
            $def_name = $crate::minhook::MinHook::create_hook($addr as _, $hook_name as _)
                .expect("failed to create hook");
        };
    };
    // CLASSNAME_INIT -> classname_init_h @ base + 0xDEADBEEF (base = gd base)
    ($def_name:ident -> $hook_name:ident @ base + $addr:literal) => {
        unsafe {
            $def_name = $crate::minhook::MinHook::create_hook(
                ($crate::get_base() + $addr) as _,
                $hook_name as _,
            )
            .expect("failed to create hook");
        };
    };
    // SCHEDULE_UPDATE -> schedule_update_h @ "libcocos2d.dll"."?update@CCScheduler@cocos2d@@UAEXM@Z"
    ($def_name:ident -> $hook_name:ident @ $lib:literal.$func:literal) => {
        unsafe {
            let hmod = winapi::um::libloaderapi::GetModuleHandleA($crate::cstr!($lib));
            assert!(
                hmod as usize != 0,
                "failed to get module handle for {}.{}",
                $lib,
                $func,
            );
            $def_name = $crate::minhook::MinHook::create_hook(
                winapi::um::libloaderapi::GetProcAddress(hmod, $crate::cstr!($func)) as _,
                $hook_name as _,
            )
            .expect("failed to create hook");
        };
    };
}

/// Removes a function hook.
///
/// Syntax is the same as in [`define_hook`] and [`create_hook`]
#[macro_export]
macro_rules! remove_hook {
    ($addr:literal) => {
        unsafe {
            $crate::minhook::MinHook::remove_hook($addr as _).expect("failed to remove hook")
        };
    };
    (base + $addr:literal) => {
        unsafe {
            $crate::minhook::MinHook::remove_hook(($crate::get_base() + $addr) as _)
                .expect("failed to remove hook")
        };
    };
    ($lib:literal.$func:literal) => {
        unsafe {
            let hmod = winapi::um::libloaderapi::GetModuleHandleA($crate::cstr!($lib));
            assert!(
                hmod as usize != 0,
                "failed to get module handle for {}.{}",
                $lib,
                $func,
            );
            $crate::minhook::MinHook::remove_hook(winapi::um::libloaderapi::GetProcAddress(
                hmod,
                $crate::cstr!($func),
            ) as _)
            .expect("failed to remove hook");
        };
    };
}

/// Gets a function pointer from a function signature.
///
/// Example:
///
/// ```no_run
/// use geometrydash::*;
///
/// define_hook! { PUSH_BUTTON -> push_button_h, "fastcall"(playlayer: usize, _edx: usize, param: i32, button: bool) -> u32 {
///     println!("button pushed");
///
///     // get the original function and call it so the game counts it
///     get_func!(PUSH_BUTTON "fastcall"(usize, usize, i32, bool) -> u32)(playlayer, 0, param, button)
/// }}
/// ```
#[macro_export]
macro_rules! get_func (
    ($def_name:ident $callconv:literal($($($t:ty),+)?) $(-> $ret:ty)?) => {
        unsafe {
            std::mem::transmute::<*mut std::ffi::c_void, unsafe extern $callconv fn($($($t),+)?) $(-> $ret)?>($def_name)
        };
    }
);

/// Enables all initialized hooks (see [`define_hook`] and [`create_hook`]).
#[inline]
pub fn enable_hooks() -> Result<(), minhook::MH_STATUS> {
    unsafe { minhook::MinHook::enable_all_hooks()? };
    Ok(())
}

/// Disables all initialized hooks (see [`define_hook`], [`enable_hooks`] and [`create_hook`]).
///
/// After calling this function, you probably should do something like this for all hooks you created:
///
/// ```no_run
/// remove_hook!(base + 0xDEADBEEF); // use whatever syntax you like, it's the same in all hook macros
/// ```
#[inline]
pub fn disable_all_hooks() -> Result<(), minhook::MH_STATUS> {
    unsafe { minhook::MinHook::disable_all_hooks()? };
    Ok(())
}
