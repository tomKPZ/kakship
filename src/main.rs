mod error;

use crate::error::Error;
use std::env;
use std::process::Command;
use std::path::Path;
use yew_ansi::{SgrEffect, ColorEffect, get_sgr_segments};

fn print_options(effect: &SgrEffect) {
	print!("+");
	if effect.italic { print!("i"); }
	if effect.underline { print!("u"); }
	if effect.bold { print!("b"); }
	if effect.reverse { print!("r"); }
	if effect.dim { print!("d"); }
}

fn print_color(color: &ColorEffect) {
	match color {
		ColorEffect::Name(color) => print!("{}", color),
		ColorEffect::NameBright(color) => print!("bright-{}", color),
		ColorEffect::Rgb(color) => print!("rgb:{:X}", color),
		ColorEffect::None => ()
	}
}

fn main() -> Result<(), Error>{
	let config_dir = env::var("kak_config")?;
	let config = Path::new(&config_dir).join("starship.toml");
	let args: Vec<String> = env::args().skip(1).collect();
	let starship= Command::new("starship")
		.env("STARSHIP_SHELL", "")
		.env("STARSHIP_CONFIG", config)
		.args(&args)
		.output()?;

	return if starship.status.code() != Some(0) {
		Err(Error::StarshipError(String::from_utf8_lossy(&starship.stderr).to_string()))
	} else {
		let stdout = String::from_utf8_lossy(&starship.stdout);
		if let Some(verb) = args.get(0) {
			if verb == "prompt" {
				for (effect, txt) in get_sgr_segments(&stdout) {
					let has_option = effect.italic || effect.underline || effect.bold || effect.reverse || effect.dim;
					let has_color = effect.bg != ColorEffect::None || effect.fg != ColorEffect::None;
					if has_option || has_color {
						let has_colors = effect.bg != ColorEffect::None && effect.fg != ColorEffect::None;
						print!("{{");
						print_color(&effect.fg);
						if has_colors { print!(","); }
						print_color(&effect.bg);
						if has_option { print_options(&effect); }
						print!("}}");
					}
					print!("{}", txt.replace("%", "%%").replace("val{","%val{"));
				}
			} else {
				println!("{}", stdout);
				eprintln!("{}", String::from_utf8_lossy(&starship.stderr));
			}
		}
		Ok(())
	}
}
