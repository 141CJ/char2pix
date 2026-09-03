use opencv::core::{self, MatExprTraitConst, MatTrait};
use opencv::imgcodecs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = "meow meow meow meow";
    let text_array = text.chars();

    let pixels_needed = text.len() / 3;
    let width = (pixels_needed as f64).sqrt().ceil() as i32;
    let height = ((pixels_needed as f64) / (width as f64)).ceil() as i32;
    let mut img = core::Mat::zeros(height, width, core::CV_8UC3)?.to_mat()?;

    let mut colors: Vec<u8> = Vec::with_capacity(3);

    let mut index = 0;
    for i in text_array.into_iter() {
        colors.push(i as u8);
        if colors.len() == 3 {
            let pixel = img.at_mut::<core::Vec3b>(index)?;
            pixel[0] = colors[0];
            pixel[1] = colors[1];
            pixel[2] = colors[2];
            colors.clear();
            index += 1;
        }
    }
    imgcodecs::imwrite("pixels.png", &img, &core::Vector::new())?;
    Ok(())
}
