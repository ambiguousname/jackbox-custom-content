fn main() {
	glib_build_tools::compile_resources(&["src/templates/resources"], "src/templates/resources/resources.gresource.xml", "resources.gresource");
}