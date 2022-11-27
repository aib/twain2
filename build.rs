use std::env;
use std::path::PathBuf;

const TWAIN_WRAPPER_H: &str = "src/twain_wrapper.h";

fn main() {
	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

	println!("cargo:rerun-if-changed={}", TWAIN_WRAPPER_H);
	bindgen::Builder::default()
		.header(TWAIN_WRAPPER_H)
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.generate()
		.expect("Unable to generate twain.h bindings")
		.write_to_file(out_path.join("twain_h_bindings.rs"))
		.expect("Unable to write twain.h bindings");
}
