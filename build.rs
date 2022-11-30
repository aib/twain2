use std::env;
use std::path::PathBuf;

fn main() {
	let target_windows = std::env::var("CARGO_CFG_TARGET_OS").map_or(false, |t| t.eq_ignore_ascii_case("windows"));

	let twain_wrapper_h = if target_windows { "ext/twain_wrapper_windows.h" } else { "ext/twain_wrapper_unix.h" };

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

	println!("cargo:rerun-if-changed={}", twain_wrapper_h);
	bindgen::Builder::default()
		.header(twain_wrapper_h)
		.allowlist_file("ext/.*\\.h")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.generate()
		.expect("Unable to generate twain.h bindings")
		.write_to_file(out_path.join("twain_h_bindings.rs"))
		.expect("Unable to write twain.h bindings");
}
