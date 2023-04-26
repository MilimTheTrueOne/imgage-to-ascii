fn main() {
    let img = image::open("image.png").unwrap();
    let resized = img.resize(40, 40, image::imageops::FilterType::Nearest);
    resized
        .grayscale()
        .as_luma8()
        .unwrap()
        .chunks(resized.width() as usize)
        .for_each(|l| {
            l.iter().for_each(|p| match *p {
                0..=100 => print!(" "),
                101..=u8::MAX => print!("#"),
            });
            println!("");
        });
}
