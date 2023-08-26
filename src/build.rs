use std::process::Command;
use std::{fs, path::PathBuf};

#[allow(unused_parens)]
fn main() {
	glib_build_tools::compile_resources(&["src/templates/resources"], "src/templates/resources/resources.gresource.xml", "resources.gresource");

	let o = Command::new("glib-compile-schemas").arg("./src").output().unwrap();
	assert!(o.status.success(), "glib-compile-schemas failed with {} and stderr: {}\n", o.status, String::from_utf8_lossy(&o.stderr));

	#[cfg(debug_assertions)]
	let subpath = "debug/";

	#[cfg(not(debug_assertions))]
	let subpath = "release/";

	let mut dirpath = PathBuf::from("./target/");
	dirpath.push(subpath);
	dirpath.push("share/glib-2.0/schemas/");

	if (!dirpath.exists()) {
		let creation = fs::create_dir_all(dirpath.as_path().clone());
		assert!(creation.is_ok(), "Could not create directories: {}", creation.err().unwrap());
	}

	dirpath.push("gschemas.compiled");

	// Install as part of the executable:
	let move_file = fs::rename("./src/gschemas.compiled", dirpath.clone());
	assert!(move_file.is_ok(), "{}, file path: {}", move_file.err().unwrap(), dirpath.display());
}