use std::fs::File;
use std::path::Path;
use dmi::icon::Icon;
use std::env;

fn main() {
	let mut args: Vec<String> = env::args().collect();

	let _self_path = args.remove(0);

	match args.len() { 
		0 => {
			println!("No images found to open.\nSolution: click and drag multiple images into the executable to feed it the requried information.");
			dont_disappear::any_key_to_continue::default();
			return;
		},
		1 => {
			println!("A single image found to open.\nSolution: click and drag multiple images into the executable to feed it the requried information.");
			dont_disappear::any_key_to_continue::default();
			return;
		}
		_ => ()
	};

	let mut states = vec![];
	let mut first_icon_path = "".to_string();
	let (mut output_width, mut output_height) = (None, None);
	let mut icons_combined = 0;

	for image_path_string in args.iter() {
		let path = Path::new(&image_path_string);
		let file = match File::open(&path) {
			Ok(f) => f,
			Err(e) => {
				println!("Wrong file path: {:#?}", e);
				dont_disappear::any_key_to_continue::default();
				return;
			}
		};
		let dmi = match Icon::load(file) {
			Ok(i) => i,
			Err(e) => {
				println!("Wrong icon format: {:#?}", e);
				dont_disappear::any_key_to_continue::default();
				return;
			}
		};
		match (output_width, output_height) {
			(None, None) => {
				first_icon_path = image_path_string.clone();
				output_width = Some(dmi.width);
				output_height = Some(dmi.height);
			},
			(Some(width), Some(height)) => {
				if width != dmi.width || height != dmi.height {
					println!("Skipping icon due to width and/or height conflict.\nIncluded icon: {} ({}x{})\nSkipped icon: {} ({}x{})\n.", first_icon_path, width, height, image_path_string, dmi.width, dmi.height);
					continue
				}
			},
			_ => {
				println!("This should never happen.");
				dont_disappear::any_key_to_continue::default();
				return;
			}
		};
		states.extend(dmi.states.iter().cloned());
		icons_combined += 1;
	};

	if icons_combined < 2 {
		println!("Unable to find two or more valid icons to combine with each other. Aborting.");
		dont_disappear::any_key_to_continue::default();
		return
	}

	match states.len() {
		0 => {
			println!("No icon states found. Did you mix empty dmis?");
			dont_disappear::any_key_to_continue::default();
			return;
		},
		1 => {
			println!("Warning, a single icon state created. Did you mix empty dmis?");
		},
		_ => ()
	}

	let output_icon = Icon {
		version: Default::default(),
		width: output_width.unwrap(),
		height: output_height.unwrap(),
		states,
	};

	let output_path = Path::new("output.dmi");
	let mut output_file = match File::create(&output_path) {
		Ok(f) => f,
		Err(e) => {
			println!("Unable to create output path: {}", e);
			dont_disappear::any_key_to_continue::default();
			return;
		}
	};
	match output_icon.save(&mut output_file) {
		Ok(_) => println!("Output file successfully written."),
		Err(e) => println!("Output file failed to write: {}", e)
	};

	println!("End of program.");
	dont_disappear::any_key_to_continue::default();
}
