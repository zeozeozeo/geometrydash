use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR enviroment variable not set"));

    let bindings = bindgen::Builder::default()
        .header("src/fmod/fmod-headers/include/fmod.h")
        .header("src/fmod/fmod-headers/include/fmod_codec.h")
        .header("src/fmod/fmod-headers/include/fmod_common.h")
        .header("src/fmod/fmod-headers/include/fmod_dsp.h")
        .header("src/fmod/fmod-headers/include/fmod_dsp_effects.h")
        .header("src/fmod/fmod-headers/include/fmod_errors.h")
        .header("src/fmod/fmod-headers/include/fmod_output.h")
        .prepend_enum_name(false)
        .derive_debug(true);

    bindings
        .generate()
        .expect("unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("unable to write bindings");

    // link libraries
    link_libs();
}

#[cfg(windows)]
fn link_libs() {
    let path = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR enviroment variable not set"),
    )
    .join("src")
    .join("fmod")
    .join("fmod-headers")
    .join("lib")
    .join("x86");

    println!("cargo:rustc-link-search=native={}", path.display());
    println!("cargo:rustc-link-lib=fmod_vc");
}
