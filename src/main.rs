use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut out = String::new();

    match args.len() {
        1 => out = "No args provided!".into(),
        2 => img_to_ascii(&args[1], &mut out, 25, 25),
        4 => img_to_ascii(
            &args[1],
            &mut out,
            args[2].parse().unwrap(),
            args[3].parse().unwrap(),
        ),
        _ => out = "Malformed arguments supplied".into(),
    }

    println!("{out}");
}

fn img_to_ascii(path: &str, out: &mut String, width: u32, height: u32) {
    if let Ok(img) = image::open(path) {
        let resized = img.resize_exact(width, height, image::imageops::FilterType::Nearest);
        resized
            .grayscale()
            .as_luma8()
            .unwrap()
            .chunks(resized.width() as usize)
            .for_each(|l| {
                out.extend(l.iter().map(|p| match *p {
                    0..=100 => " ",
                    101..=u8::MAX => "#",
                }));
                out.extend(['\n']);
            });
    } else {
        *out = "Incorect file provided".into();
    }
}
