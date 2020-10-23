extern crate vergen;

use std::env;
use vergen::{generate_cargo_keys, ConstantsFlags};

fn main() {
    // Setup the flags, toggling off the 'SEMVER_FROM_CARGO_PKG' flag
    let mut flags = ConstantsFlags::all();
    flags.toggle(ConstantsFlags::SEMVER_FROM_CARGO_PKG);
    // Generate the 'cargo:' key output
    generate_cargo_keys(ConstantsFlags::all()).expect("Unable to generate the cargo keys!");

    // Generate Commit SHA from `SOURCE_VERSION` if exists (which will be set by heroku when build.)
    println!(
        "cargo:rustc-env=HEROKU_SHA={}",
        env::var("SOURCE_VERSION").unwrap_or("UNKNOWN".to_string())
    );
}
