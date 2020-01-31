extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg("-Irayfork/rayfork")
        .clang_arg("-Irayfork/dependencies/tinyobjloader-c")
        .clang_arg("-Irayfork/dependencies/cgltf")
        .clang_arg("-Irayfork/dependencies/par")
        .clang_arg("-Irayfork/dependencies/stb")
        .clang_arg("-Irayfork/examples/dependencies/glad")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // compile wrapper.c
    cc::Build::new()
        .file("wrapper.c")
        .file("rayfork/examples/dependencies/glad/glad.c")
        .include(".")
        .include("./rayfork/rayfork")
        .include("./rayfork/dependencies/tinyobjloader-c")
        .include("./rayfork/dependencies/cgltf")
        .include("./rayfork/dependencies/par")
        .include("./rayfork/dependencies/stb")
        .include("./rayfork/examples/dependencies/glad")
        .warnings(false)
        .extra_warnings(false)
        .compile("rayfork");
}
