use twain2::twain_h::*;
use twain2::twain_h_ext::*;

#[cfg(unix)]
const DSM_FILE: &str = "ext/libtwaindsm.so";
#[cfg(windows)]
const DSM_FILE: &str = "ext/TWAINDSM.dll";

pub struct TwainLib {
	_lib: libloading::Library,
	pub dsm_entry: DSMENTRYPROC,
}

pub fn load_twain_lib() -> TwainLib {
	let lib = unsafe { libloading::Library::new(DSM_FILE).unwrap() };

	let dsm_entry = Some(*unsafe { lib.get(b"DSM_Entry\0") }.unwrap());

	TwainLib {
		_lib: lib,
		dsm_entry,
	}
}

pub fn get_app_identity(support_app2:bool) -> TW_IDENTITY {
	TW_IDENTITY {
		Id: 0,
		Version: TW_VERSION {
			MajorNum: env!("CARGO_PKG_VERSION_MAJOR").parse::<u16>().unwrap() as TW_UINT16,
			MinorNum: env!("CARGO_PKG_VERSION_MINOR").parse::<u16>().unwrap() as TW_UINT16,
			Language: TWLG_ENGLISH_USA as TW_UINT16,
			Country: TWCY_USA as TW_UINT16,
			Info: tw_str32(env!("CARGO_PKG_VERSION")),
		},
		ProtocolMajor: TWON_PROTOCOLMAJOR as TW_UINT16,
		ProtocolMinor: TWON_PROTOCOLMINOR as TW_UINT16,
		SupportedGroups: DG_IMAGE | DG_CONTROL | if support_app2 { DF_APP2 } else { 0 },
		Manufacturer: tw_str32("Rust TWAIN Library"),
		ProductFamily: tw_str32("Tests"),
		ProductName: tw_str32("Integration Test"),
	}
}
