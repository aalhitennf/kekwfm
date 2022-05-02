use eframe::{egui, epaint::ColorImage};

#[derive(Default)]
pub struct Textures {
    pub settings: ColorImage,
    pub arrow_left: ColorImage,
    pub arrow_right: ColorImage,
}

impl Textures {
    pub fn new(theme: &str) -> Self {
        // TODO This can be done better i thnk
        let settings = load_image_from_path(&format!("theme/{theme}/icons/settings.png"))
            .map_or(ColorImage::example(), |image| image);
        let arrow_left = load_image_from_path(&format!("theme/{theme}/icons/arrow-left.png"))
            .map_or(ColorImage::example(), |image| image);
        let arrow_right = load_image_from_path(&format!("theme/{theme}/icons/arrow-right.png"))
            .map_or(ColorImage::example(), |image| image);
        Textures {
            settings,
            arrow_left,
            arrow_right,
        }
    }
}

fn load_image_from_path(path: &str) -> Result<egui::ColorImage, image::ImageError> {
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
