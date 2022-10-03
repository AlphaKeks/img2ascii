use image::GenericImageView;

const FACTOR: f32 = 2.5;

fn main() {
	if std::env::args().nth(1) == Some(String::from("--help")) {
		println!("Usage: img2ascii [PATH] [SCALE]");
		println!("");
		println!("PATH:");
		println!("file path to an image");
		println!("");
		println!("SCALE:");
		println!("downscale factor");
		return;
	}

	let mut msg = String::new();
	let target = match std::env::args().nth(1) {
		Some(path) => {
			msg.push_str(&format!("Converting {path}"));
			path
		}
		None => return die("Not a valid filepath. See `--help` for usage info."),
	};

	let scale: u32 = match std::env::args().nth(2) {
		Some(scale) => {
			msg.push_str(&format!(" with a scale of {scale}..."));
			match scale.parse() {
				Ok(num) => num,
				Err(_) => return die("Invalid scale. See `--help` for usage info."),
			}
		}
		None => return die("No scale was specified. See `--help` for usage info."),
	};

	println!("{msg}");

	let img = match image::open(target) {
		Ok(img) => img,
		Err(why) => return die(&why.to_string()),
	};

	let (w, h) = img.dimensions();
	for y in 0..h {
		for x in 0..w {
			if y % (scale as f32 * FACTOR) as u32 == 0 && x % scale == 0 {
				let px = img.get_pixel(x, y);
				let mut intent = px[0] / 3 + px[1] / 3 + px[2] / 3;

				if px[3] == 0 {
					intent = 0;
				}

				print!("{}", get_str_ascii(intent));
			}
		}

		if y % (scale as f32 * FACTOR) as u32 == 0 {
			println!("");
		}
	}
}

fn get_str_ascii(intent: u8) -> &'static str {
	let idx = intent / 32;
	let ascii_chars = [" ", ".", ",", "-", "~", "+", "=", "@"];

	ascii_chars[idx as usize]
}

fn die(why: &str) {
	println!("{why}");
	std::process::exit(1);
}
