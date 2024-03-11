use std::{env, fs::File, io::Write, path::Path};

use regex::Regex;

mod content_reader;

#[derive(Debug)]
struct ContentWindowItem {
	xml_def_path : String,
	mod_location : String,
}

pub fn compile_content_list() {
	let content_list = include_str!("../../content/content_list.ui");

	let content_tag : Regex = Regex::new(r#"<property name="xml-definition">\W*(?<def>[\w\/.]+)\W*<\/property>"#).unwrap();

	let content = content_tag.captures_iter(content_list).map(move |caps| {
		let def = caps.name("def").unwrap();
		let result = content_reader::read(def.as_str().to_string());
		if result.is_err() {
			panic!("Could not read {}: {}", def.as_str(), result.unwrap_err());
		}
		result.unwrap()
	});

	
	let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("content_list.rs");
	let mut out_file = File::create(dest_path).expect("Could not create file.");

	out_file.write(b"pub fn create_window(window_type : String) -> ContentWindow {\nreturn match window_type.as_str() {\n").expect("Could not write bytes.");
	for c in content {
		let out = format!("\t\"{}\" => {{crate::content::{}::ensure_all_types(); gtk::glib::Object::new::<crate::content::{}>().upcast()}},\n", c.xml_def_path, c.mod_location, c.mod_location);
		out_file.write(out.as_bytes()).expect("Could not write bytes.");
	}
	out_file.write(b"\t_=>panic!(\"Window type {{window_type}} not found.\")};\n}").expect("Could not write bytes.");
	
	println!("cargo:rerun-if-changed=src/build/content_list/mod.rs");
	println!("cargo:rerun-if-changed=src/build/content_list/content_reader.rs");
	println!("cargo:rerun-if-changed=src/content/content_list.ui");
}