// build.rs

// Bring in a dependency on an externally maintained `gcc` package which manages
// invoking the C compiler.
// extern crate gcc;

use std::process::Command;
use std::env;
// use std::path::Path;

fn main() {
    // gcc::compile_library("libduktape.a",
    //     &["deps/duktape-releases/src/duktape.c",
    //       "src/duk_rust_link.c"]);

    // let out_dir = env::var("OUT_DIR").unwrap();

// note that there are a number of downsides to this approach, the comments
// below detail how to improve the portability of these commands.
    Command::new("bash").args(&["deps/ChakraCore/build.sh",
                                // "--static",
                                "--test-build",
                                "--icu=/usr/local/opt/icu4c/include",
                                "-j=8"])
                        .status().unwrap();


    let lib_path = "deps/ChakraCore/BuildLinux/Test/lib/";

    println!("cargo:rustc-link-search=native={}{}", lib_path, "../pal/src/");
    println!("cargo:rustc-link-lib=dylib=Chakra.Pal");

    println!("cargo:rustc-link-search=native={}{}", lib_path, "Common/Core/");
    println!("cargo:rustc-link-lib=dylib=Chakra.Common.Core");

    println!("cargo:rustc-link-search=native={}{}", lib_path, "Jsrt/");
    println!("cargo:rustc-link-lib=dylib=Chakra.Jsrt");


    let icu4c_path = "/usr/local/opt/icu4c/lib/";

    println!("cargo:rustc-link-search=native={}", icu4c_path);
    println!("cargo:rustc-link-lib=dylib=icudata");
    println!("cargo:rustc-link-lib=dylib=icuuc");
    println!("cargo:rustc-link-lib=dylib=icui18n");


    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=framework=Security");
}
