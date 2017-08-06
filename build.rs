extern crate cmake;
extern crate pkg_config;

use cmake::Config;
use std::env;

fn main() {
    // Use system libassimp if it exists
    if let Ok(..) = pkg_config::Config::new().atleast_version("4.0.0").find("assimp") {
        return
    }

    // Compile assimp from source
    // Disable unnecessary stuff, it takes long enough to compile already
    let dst = Config::new("assimp")
        .define("ASSIMP_BUILD_ASSIMP_TOOLS", "OFF")
        .define("ASSIMP_BUILD_TESTS", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("LIBRARY_SUFFIX", "")
        .build();
    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());

    // Fix for irrXML not being linked properly (https://github.com/assimp/assimp/issues/1283)
    println!("cargo:rustc-link-search=native={}", dst.join("build/contrib/irrXML").display());
    println!("cargo:rustc-link-lib=IrrXML");

    // Link to correct versions of assimp and zlib
    // NOTE: MSVC has to link to release libs to avoid CRT mismatch
    println!("cargo:rustc-link-lib=static=assimp");
    if !pkg_config::find_library("zlib").is_ok() {
        println!("cargo:rustc-link-lib=static=zlibstatic");
    }

    // Link to libstdc++ on GNU
    let target = env::var("TARGET").unwrap();
    if target.contains("gnu") {
        println!("cargo:rustc-link-lib=stdc++");
    }


    println!("cargo:rerun-if-changed=build.rs");
}
