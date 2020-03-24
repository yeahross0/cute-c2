use cc::Build;
use std::{env, path::PathBuf};

fn main() {
    let project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let root_dir = project_dir.parent().unwrap();
    let src = root_dir.join("cute-header");

    Build::new()
        .file(src.join("cute_c2.c"))
        .include(&src)
        .warnings(false)
        .compile("cute_c2");
}
