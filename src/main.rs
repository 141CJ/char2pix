use opencv::core::{self, MatExprTraitConst, MatTrait};
use opencv::imgcodecs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = "meow meow meow meow";
    let text_array = text.chars();

    let img_size = ((text.len() / 3) as f64).sqrt().ceil();

    let mut colors: Vec<u8> = Vec::with_capacity(3);
    let mut img = core::Mat::zeros(img_size as i32, img_size as i32, core::CV_8UC3)?.to_mat()?;

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
