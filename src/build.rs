#[allow(unused_parens)]
fn main() {
	// let out_path = PathBuf::from(env::var_os("OUT_DIR").expect("Could not get OUT_DIR environment variable."));
	glib_build_tools::compile_resources(&["src/templates/", "src/content/"], "src/resources.gresource.xml", "resources.gresource");
	println!("cargo:rerun-if-changed=src/build.rs");
}