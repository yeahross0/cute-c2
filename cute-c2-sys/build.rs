use cc::Build;
use std::{env, path::PathBuf};

fn main() {
    let project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let src = project_dir.join("cute-header");

    Build::new()
        .file(src.join("cute_c2.c"))
        .include(&src)
        .warnings(false)
        .compile("cute_c2");
}
