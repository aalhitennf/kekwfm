// use crate::KekwResult;

pub fn create_file(path: &str) {
    println!("Create file: {}", path);
    // std::fs::File::create(path)
}

pub fn delete_file(path: &str) {
    if let Err(e) = std::fs::remove_file(path) {
        println!("Failed to delete file: {}", path);
        println!("{:?}", e);
    }
}

pub fn xdg_open_file(path: &str) {
    match std::process::Command::new("xdg-open").arg(path).spawn() {
        Ok(_) => println!("openent"),
        Err(e) => println!("errored: {:?}", e),
    }
}
