extern crate cheddar;

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    // Generate mp4parse.h.
    cheddar::Cheddar::new().expect("could not read manifest")
        .insert_code("// THIS FILE IS AUTOGENERATED BY mp4parse-rust/build.rs - DO NOT EDIT\n\n")
        .insert_code("// This Source Code Form is subject to the terms of the Mozilla Public\n")
        .insert_code("// License, v. 2.0. If a copy of the MPL was not distributed with this\n")
        .insert_code("// file, You can obtain one at https://mozilla.org/MPL/2.0/.")
        .run_build("include/mp4parse.h");
}
