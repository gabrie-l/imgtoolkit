use image::DynamicImage;

pub fn resize_image(img: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    // Resize the image to the specified width and height
    img.resize(width, height, image::imageops::FilterType::Lanczos3)
}
