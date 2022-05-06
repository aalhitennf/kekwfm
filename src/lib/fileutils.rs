use std::path::Path;


pub fn create_file(path: &str) {
    println!("Create file: {}", path);
    // std::fs::File::create(path)
}

pub fn trash_one(path: &str) {
    if let Err(e) = trash::delete(path) {
        println!("Failed to delete file: {}", path);
        println!("{:?}", e);
    }
}

pub fn trash_many<P: AsRef<Path>>(paths: &[P]) {
    if let Err(e) = trash::delete_all(paths) {
        println!("Failed to delete file: {e}");
    }
}

pub fn xdg_open_file(path: &str) {
    match std::process::Command::new("xdg-open").arg(path).spawn() {
        Ok(_) => println!("openent"),
        Err(e) => println!("errored: {:?}", e),
    }
}
