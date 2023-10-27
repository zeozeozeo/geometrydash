# Geometry Dash modding library for Rust

This is a Rust library made for modding the Geometry Dash game.

## References

* [Gd-addresses](https://github.com/spookybear0/gd-addresses)
* [Func-dump](https://github.com/matcool/re-scripts/blob/main/func_dump.txt)
* [Gd.h](https://github.com/poweredbypie/gd.h)
* [GD-Decompiled](https://github.com/Wyliemaster/GD-Decompiled)
* [Cocos-headers](https://github.com/HJfod/cocos-headers/)
* [CappuccinoSDK](https://github.com/andrenih/cappuccinoSDK/)

## Supported platforms

* Windows

## Note

When making DLL files, make sure to set the Rust toolchain to `stable-i686-pc-windows-msvc`. This sets the DLL to build with MSVC (so it recognizes the DllMain function) in 32-bit mode (GD is 32-bit). You must be running Windows and have MSVC installed.

You can do this by either creating a `rust-toolchain` file in the same directory as `Cargo.toml` with the following contents:

```
stable-i686-pc-windows-msvc
```

and running `rustup install stable-i686-pc-windows-msvc`.

...or running `rustup default stable-i686-pc-windows-msvc`, but this will override the default toolchain for all projects to 32-bit Windows MSVC.

## TODO

* Add all known GD functions and classes
* Add all Cocos2dx functions
* GD 2.2 support when it comes out
* Consider using [libmem](https://github.com/rdbo/libmem) instead of MinHook
