fn main() {
    let dynamic_libs_dir = gdsfx_files::paths::build::get_libs_target_dir()
        .expect("No dynamic library directory found");

    // ask cargo to always load the stupid dlls pretty please :3
    build_script::cargo_rustc_link_search(dynamic_libs_dir);
}
