use std::{collections::HashMap, env, fs::File, io::Write, iter::Map, path::Path};

use regex::Regex;

mod content_reader;

#[derive(Debug)]
struct ContentWindowItem {
	xml_def_path : String,
	mod_location : String,
	window_name : String,
	
	content_info : Vec<ContentInfo>,
}

#[derive(Debug)]
struct ContentInfo {
	content_type : String,
	subcontent_info : Vec<SubcontentInfo>,
}

#[derive(Debug)]
struct SubcontentInfo {
	args: Vec<String>,
}

pub fn compile_content_list() {
	let content_list = include_str!("../../content/content_list.ui");

	let content_tag : Regex = Regex::new(r#"<property name="xml-definition">\W*(?<def>[\w\/.]+)\W*<\/property>"#).unwrap();

	let content : Vec<ContentWindowItem> = content_tag.captures_iter(content_list).map(move |caps| {
		let def = caps.name("def").unwrap();
		let result = ContentWindowItem::read(def.as_str().to_string());
		if result.is_err() {
			panic!("Could not read {}: {}", def.as_str(), result.unwrap_err());
		}
		result.unwrap()
	}).collect();

	
	let out_dir = env::var_os("OUT_DIR").unwrap();

    let content_list_pth = Path::new(&out_dir).join("content_list.rs");
	let mut content_list_out = File::create(content_list_pth).expect("Could not create file.");

	content_list_out.write(b"pub fn create_window(window_type : String) -> ContentWindow {\nreturn match window_type.as_str() {\n").expect("Could not write bytes.");
	for c in &content {
		let out = format!("\t\"{}\" => {{crate::content::{}::ensure_all_types(); gtk::glib::Object::new::<crate::content::{}>().upcast()}},\n", c.xml_def_path, c.mod_location, c.mod_location);
		content_list_out.write(out.as_bytes()).expect("Could not write bytes.");
	}
	content_list_out.write(b"\t_=>panic!(\"Window type {{window_type}} not found.\")};\n}\n").expect("Could not write bytes.");

	content_list_out.write(b"pub fn get_subcontent_args(window_type : String, content_type : String, subcontent : Vec<SubcontentBox>) -> Vec<Vec<String>> {\n\tmatch window_type {\n").expect("Could not write bytes.");
	for c in &content {
		let info = &c.content_info;
		content_list_out.write(b"\tmatch content_type {\n").expect("Could not write bytes.");
		for i in info {
			let i_out = format!("\t\t{} => vec![", i.content_type);
			content_list_out.write(i_out.as_bytes()).expect("Could not write bytes.");
			for s in &i.subcontent_info {
				let s_out = format!("vec![{}]", s.args.join(","));
				content_list_out.write(s_out.as_bytes()).expect("Could not write subcontent info.");
			}
			content_list_out.write(b"]\n").expect("Could not write bytes.");
		}
		content_list_out.write(b"\t},\n").expect("Could not write bytes.");
	}
	content_list_out.write(b"\t_=>panic!(\"Window type {{window_type}} not found.\"),\n}").expect("Could not write bytes.");
	
	println!("cargo:rerun-if-changed=src/build/content_list/mod.rs");
	println!("cargo:rerun-if-changed=src/build/content_list/content_reader.rs");
	println!("cargo:rerun-if-changed=src/content/content_list.ui");
}