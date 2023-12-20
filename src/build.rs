use std::path::Path;
use std::process::Command;
use std::{fs, io, env, path::PathBuf};

// Tries to make a distribution as close to https://www.gtk.org/docs/installations/windows/#building-and-distributing-your-application as possible.
fn compile(out_path : PathBuf) {
	let new_path = out_path.join("share/glib-2.0/schemas");
	if !new_path.exists() {
		let creation = fs::create_dir_all(new_path.clone().as_path().clone());
		assert!(creation.is_ok(), "Could not create directories: {}", creation.err().unwrap());
	}

	let target_dir = format!("--targetdir={}", out_path.as_path().to_str().expect("Could not get path str"));

	let o = Command::new("glib-compile-schemas").arg("./src").arg(target_dir).output().unwrap();
	assert!(o.status.success(), "glib-compile-schemas failed with {} and stderr: {}\n", o.status, String::from_utf8_lossy(&o.stderr));


	glib_build_tools::compile_resources(&["src/templates/", "src/content/"], "src/resources.gresource.xml", "resources.gresource");
}

// From https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust
// fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
//     fs::create_dir_all(&dst)?;
//     for entry in fs::read_dir(src)? {
//         let entry = entry?;
//         let ty = entry.file_type()?;
//         if ty.is_dir() {
//             copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
//         } else {
//             fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
//         }
//     }
//     Ok(())
// }

// fn install_theme(out_path : PathBuf) {
// 	let theme_path = Path::new("./themes/");
// 	assert!(theme_path.exists(), "Themes folder {} does not exist. Please install a theme there.", theme_path.to_str().unwrap());
// 	// Install theme to the relevant path:
// 	let new_path = out_path.join("share/themes/");
// 	if !new_path.exists() {
// 		let create = fs::create_dir_all(new_path.clone());
// 		assert!(create.is_ok(), "Creating share/themes directories failed: {}", create.err().unwrap());
// 	}

// 	let copy = copy_dir_all(theme_path, new_path);
// 	assert!(copy.is_ok(), "Copy to share/themes/ failed: {}", copy.err().unwrap());
// }

// fn install_settings(out_path : PathBuf) {
// 	let new_path = out_path.join("etc/gtk-4.0");
// 	if !new_path.exists() {
// 		let create = fs::create_dir_all(new_path.clone());
// 		assert!(create.is_ok(), "Creating etc/gtk-4.0 directories failed: {}", create.err().unwrap());
// 	}

// 	if !new_path.join("settings.ini").exists() {
// 		let copy = fs::copy("./src/settings.ini", new_path);
// 		assert!(copy.is_ok(), "Copy to etc/gtk-4.0 failed: {}", copy.err().unwrap());
// 	}
// }

#[allow(unused_parens)]
fn main() {
	let out_path = PathBuf::from(env::var_os("OUT_DIR").expect("Could not get OUT_DIR environment variable.")).join("../../../");
	
	compile(out_path.clone());
	// install_theme(out_path.clone());
	// install_settings(out_path.clone());

	println!("cargo:rerun-if-changed=src/build.rs");
}