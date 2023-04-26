use std::env;

const ASCII: &str = " `.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";

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
            .into_luma8()
            .chunks(resized.width() as usize)
            .for_each(|l| {
                out.extend(l.iter().map(|p| {
                    ASCII.as_bytes()[((*p as f32 / 256.0) * ASCII.len() as f32) as usize] as char
                } ));
                out.extend(['\n'])
            });
    } else if let Err(e) = image::open(path){
        println!("{}", e);
        *out = "Incorect file provided".into();
    }
}
