mod requests;
mod encoding;
mod library;
mod gui;

fn main() {
    gui::GdSfx::run(Default::default());

    // let local_appdata_folder = PathBuf::from(env::var("localappdata").unwrap());
    // let gd_folder = local_appdata_folder.join("GeometryDash");




    // println!("{:?}", deciphered);

    // for i in 170..14260 {
    //     let filename = format!("s{i}.ogg");
    //     let filepath = gd_folder.join(&filename);
    //     let file_url = format!("{cdn_url}/sfx/{filename}");

    //     if filepath.exists() { continue }

    //     let request = client.get(file_url).send().unwrap();

    //     if request.status().is_success() {
    //         let data = request.bytes().unwrap();

    //         fs::write(filepath, data).unwrap();

    //         println!("[SUCCESS] {filename}");
    //     } else {
    //         println!("[FAILED] {filename}");
    //     }

    //     // let a = get();
    // }
}
