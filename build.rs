
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=./nvafx/lib");
    println!("cargo:rustc-link-lib=dylib=nv_audiofx");
    println!("cargo:rerun-if-changed=./nvafx/include/nvAudioEffects.h");

    let bindings = bindgen::Builder::default()
        .header("./nvafx/include/nvAudioEffects.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings.");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings.");
}