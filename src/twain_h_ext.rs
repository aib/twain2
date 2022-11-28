use super::twain_h::*;

use std::ffi::CString;

pub type TwainUConst = u32;

pub const STR32_LEN: usize = 34;
pub const STR32_EMPTY: TW_STR32 = [0; STR32_LEN];

impl Default for TW_ENTRYPOINT {
	fn default() -> Self {
		Self {
			Size: std::mem::size_of::<Self>() as TW_UINT32,
			DSM_Entry: None,
			DSM_MemAllocate: None,
			DSM_MemFree: None,
			DSM_MemLock: None,
			DSM_MemUnlock: None,
		}
	}
}

impl Default for TW_IDENTITY {
	fn default() -> Self {
		Self {
			Id: 0,
			Version: TW_VERSION {
				MajorNum: 0,
				MinorNum: 0,
				Language: 0,
				Country: 0,
				Info: STR32_EMPTY,
			},
			ProtocolMajor: 0,
			ProtocolMinor: 0,
			SupportedGroups: 0,
			Manufacturer: STR32_EMPTY,
			ProductFamily: STR32_EMPTY,
			ProductName: STR32_EMPTY,
		}
	}
}

pub fn tw_str32<S: AsRef<str>>(string: S) -> TW_STR32 {
	let mut twstr = STR32_EMPTY;

	let slen = std::cmp::min(twstr.len() - 1, string.as_ref().len());
	let sbytes = string.as_ref().as_bytes();

	for i in 0..slen {
		twstr[i] = sbytes[i] as i8;
	}
	twstr[slen] = 0;

	twstr
}

pub fn tw_str32_to_string(twstr: &TW_STR32) -> String {
	let slen = twstr.len();
	let mut sbytes = Vec::with_capacity(twstr.len());

	for i in 0..slen {
		let b = twstr[i] as u8;
		sbytes.push(b);
		if b == 0 { break };
	}

	CString::from_vec_with_nul(sbytes)
		.map(|cstr| cstr.to_string_lossy().into_owned())
		.unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn str32_empty_is_empty() {
		assert_eq!("", tw_str32_to_string(&STR32_EMPTY));
	}

	#[test]
	fn empty_string_to_str32_and_back() {
		let twstr = tw_str32("");
		assert_eq!("", tw_str32_to_string(&twstr));
	}

	#[test]
	fn simple_string_to_str32_and_back() {
		let s = String::from("Test string!");
		let twstr = tw_str32(s.clone());
		assert_eq!(s, tw_str32_to_string(&twstr));
	}

	#[test]
	fn simple_str_to_str32_and_back() {
		let s = "Test string!";
		let twstr = tw_str32(s.clone());
		assert_eq!(s, tw_str32_to_string(&twstr));
	}

	#[test]
	fn too_long_string_to_str32_and_back() {
		let s = String::from("This is a very long string yes indeed it is");
		let twstr = tw_str32(s.clone());
		assert_eq!(s[0..STR32_LEN-1], tw_str32_to_string(&twstr));
	}

	#[test]
	fn string_with_nul_to_str32_and_back() {
		let s = String::from("Test\0string!");
		let twstr = tw_str32(s.clone());
		assert_eq!("Test", tw_str32_to_string(&twstr));
	}
}
