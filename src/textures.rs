use eframe::{
    egui,
    epaint::{ColorImage, TextureHandle},
};

#[derive(Clone)]
pub struct TextureLoader {
    pub settings: TextureHandle,
    pub arrow_left: TextureHandle,
    pub arrow_right: TextureHandle,
    pub arrow_up: TextureHandle,
    pub home: TextureHandle,
    pub folder: TextureHandle,
    pub file: TextureHandle,
    pub star: TextureHandle,
    pub hard_drive: TextureHandle,
}

impl TextureLoader {
    pub fn new(theme: &str, ctx: &egui::Context) -> Self {
        // TODO This can be done better i thnk
        let settings = load_image_from_path(&format!("theme/{theme}/icons/settings.png"))
            .map_or(ColorImage::example(), |image| image);
        let settings = ctx.load_texture("settings-icon", settings);

        let arrow_left = load_image_from_path(&format!("theme/{theme}/icons/arrow-left.png"))
            .map_or(ColorImage::example(), |image| image);
        let arrow_left = ctx.load_texture("arrow_left", arrow_left);

        let arrow_right = load_image_from_path(&format!("theme/{theme}/icons/arrow-right.png"))
            .map_or(ColorImage::example(), |image| image);
        let arrow_right = ctx.load_texture("arrow-right", arrow_right);

        let arrow_up = load_image_from_path(&format!("theme/{theme}/icons/arrow-up.png"))
            .map_or(ColorImage::example(), |image| image);
        let arrow_up = ctx.load_texture("arrow-up", arrow_up);

        let home = load_image_from_path(&format!("theme/{theme}/icons/home.png"))
            .map_or(ColorImage::example(), |image| image);
        let home = ctx.load_texture("home-icon", home);

        let folder = load_image_from_path(&format!("theme/{theme}/icons/folder.png"))
            .map_or(ColorImage::example(), |image| image);
        let folder = ctx.load_texture("folder-icon", folder);

        let file = load_image_from_path(&format!("theme/{theme}/icons/file.png"))
            .map_or(ColorImage::example(), |image| image);
        let file = ctx.load_texture("file-icon", file);

        let star = load_image_from_path(&format!("theme/{theme}/icons/star.png"))
            .map_or(ColorImage::example(), |image| image);
        let star = ctx.load_texture("star-icon", star);

        let hard_drive = load_image_from_path(&format!("theme/{theme}/icons/hard-drive.png"))
            .map_or(ColorImage::example(), |image| image);
        let hard_drive = ctx.load_texture("hard-drive-icon", hard_drive);

        TextureLoader {
            settings,
            arrow_left,
            arrow_right,
            arrow_up,
            home,
            folder,
            file,
            star,
            hard_drive,
        }
    }
}

pub fn load_image_from_path(path: &str) -> Result<egui::ColorImage, image::ImageError> {
    let path = std::path::Path::new(path);
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}
