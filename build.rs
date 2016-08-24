// build.rs

// Bring in a dependency on an externally maintained `gcc` package which manages
// invoking the C compiler.
// extern crate gcc;

use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    // gcc::compile_library("libduktape.a",
    //     &["deps/duktape-releases/src/duktape.c",
    //       "src/duk_rust_link.c"]);

    let out_dir = env::var("OUT_DIR").unwrap();

// note that there are a number of downsides to this approach, the comments
// below detail how to improve the portability of these commands.
    Command::new("bash").args(&["deps/ChakraCore/build.sh",
                                "--static",
                                "--test-build",
                                "--icu=/usr/local/opt/icu4c/include",
                                "-j=8"])
                        .status().unwrap();

    let lib_path = "deps/ChakraCore/BuildLinux/Test/lib/";


    println!("cargo:rustc-link-search=native={}{}", lib_path,
             "../pal/src/libChakra.Pal.a");

    println!("cargo:rustc-link-search=native={}{}", lib_path,
             "Common/Core/libChakra.Common.Core.a");

    println!("cargo:rustc-link-search=native={}{}", lib_path,
             "Jsrt/libChakra.Jsrt.a");
             
    println!("cargo:rustc-link-lib=static=ChakraCore");
}
