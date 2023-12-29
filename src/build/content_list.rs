use std::{env, path::Path, fs::File, io::Write, ops::Add};

use regex::Regex;

pub fn compile_content_list() {
	let content_list = include_str!("../content/content_list.ui");

	let content_tag = Regex::new(r#"<property name="window-path">\W*(?<path>[\w:]+)\W*<\/property>\W*<property name="window">\W*<object class="JCC(?<name>\w+)">"#).unwrap();
	let names = content_tag.captures_iter(content_list).map(|caps| {
		let name = caps.name("name").unwrap();
		let path = caps.name("path").unwrap();
		
		let full_name = format!("::{}", name.as_str());
		String::from(path.as_str()).add(full_name.as_str())
	});

	
	let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("content_list.rs");
	let mut out_file = File::create(dest_path).expect("Could not create file.");

	out_file.write(b"pub fn ensure_types() {\n").expect("Could not write bytes.");
	for name in names {
		let out = format!("\tcrate::content::{}::ensure_all_types();\n", name);
		out_file.write(out.as_bytes()).expect("Could not write bytes.");
	}
	out_file.write(b"}").expect("Could not write bytes.");
	
	println!("cargo:rerun-if-changed=src/build/content_list.rs");
}