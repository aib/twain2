const TWAIN_WRAPPER_H: &str = "src/twain_wrapper.h";

fn main() {
	println!("cargo:rerun-if-changed={}", TWAIN_WRAPPER_H);
}
