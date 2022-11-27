use twain::twain_h_ext::DSMEntryProc;

#[cfg(unix)]
const DSM_FILE: &str = "ext/libtwaindsm.so";

pub struct TwainLib {
	_lib: libloading::Library,
	pub dsm_entry: DSMEntryProc,
}

pub fn load_twain_lib() -> TwainLib {
	let lib = unsafe { libloading::Library::new(DSM_FILE).unwrap() };

	let dsm_entry = *(unsafe { lib.get(b"DSM_Entry\0") }.unwrap());

	TwainLib {
		_lib: lib,
		dsm_entry,
	}
}

#[test]
fn test_load_twain_lib() {
	let _lib = load_twain_lib();
	assert!(true);
}
