fn main() {
    // ask cargo to always load the stupid dlls pretty please :3
    build_script::cargo_rustc_link_search(gdsfx_files::paths::build::LIBS_TARGET_DIR);
}
