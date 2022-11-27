use twain::twain_h::*;
mod helper;

use std::ptr;

#[test]
fn test_open_and_close_dsm() {
	let lib = helper::load_twain_lib();

	let mut identity = helper::get_app_identity(false);
	let ret = unsafe { (lib.dsm_entry)(&mut identity, ptr::null_mut(), DG_CONTROL as TW_UINT32, DAT_PARENT as TW_UINT16, MSG_OPENDSM as TW_UINT16, ptr::null_mut()) };
	assert_eq!(0, ret);

	let ret = unsafe { (lib.dsm_entry)(&mut identity, ptr::null_mut(), DG_CONTROL as TW_UINT32, DAT_PARENT as TW_UINT16, MSG_CLOSEDSM as TW_UINT16, ptr::null_mut()) };
	assert_eq!(0, ret);
}
