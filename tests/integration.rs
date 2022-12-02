use twain2::*;
use twain2::twain_h::*;
use twain2::twain_h_ext::*;
mod helper;

use std::ptr;
use std::sync::Arc;

use parking_lot::Mutex;

static TWAIN_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_open_and_close_dsm() {
	helper::init();
	let _twain_mutex = TWAIN_MUTEX.lock();

	let lib = helper::load_twain_library();
	let dsm_entry: DSMENTRYPROC = Some(*unsafe { lib.get(b"DSM_Entry\0") }.unwrap());

	let mut identity = helper::get_app_identity(false);
	let ret = unsafe { (dsm_entry.unwrap())(&mut identity, ptr::null_mut(), DG_CONTROL as TW_UINT32, DAT_PARENT as TW_UINT16, MSG_OPENDSM as TW_UINT16, ptr::null_mut()) };
	assert_eq!(0, ret);

	let ret = unsafe { (dsm_entry.unwrap())(&mut identity, ptr::null_mut(), DG_CONTROL as TW_UINT32, DAT_PARENT as TW_UINT16, MSG_CLOSEDSM as TW_UINT16, ptr::null_mut()) };
	assert_eq!(0, ret);
}

#[test]
fn test_dsmentrywrapper_open_and_close_dsm() {
	helper::init();
	let _twain_mutex = TWAIN_MUTEX.lock();

	let wrapper = helper::get_dsm_entry_wrapper();

	let mut identity = helper::get_app_identity(false);
	let res = wrapper.do_dsm_entry(Some(&mut identity), None, DG_CONTROL, DAT_PARENT, MSG_OPENDSM, ptr::null_mut());
	assert_eq!(response::ReturnCode::Success, res.return_code);

	let res = wrapper.do_dsm_entry(Some(&mut identity), None, DG_CONTROL, DAT_PARENT, MSG_CLOSEDSM, ptr::null_mut());
	assert_eq!(response::ReturnCode::Success, res.return_code);
}

#[test]
fn test_openeddsm_new_and_get_data_sources() {
	helper::init();
	let _twain_mutex = TWAIN_MUTEX.lock();

	let wrapper = helper::get_dsm_entry_wrapper();

	let identity = helper::get_app_identity(false);
	let dsm = OpenedDSM::new(wrapper, identity);
	assert!(dsm.is_ok());

	let dsm = dsm.unwrap();
	let data_sources = dsm.get_data_sources();
	assert!(data_sources.is_ok());
}

fn get_software_scanner(wrapper: DSMEntryWrapper) -> Option<(Arc<OpenedDSM>, Box<OpenedDS>)> {
	let identity = helper::get_app_identity(true);
	let dsm = OpenedDSM::new(wrapper, identity).unwrap();
	for ds in dsm.get_data_sources().unwrap() {
		if tw_str32_to_string(&ds.ProductName) == "TWAIN2 Software Scanner" {
			let ds = dsm.open_data_source(ds).unwrap();
			return Some((dsm, ds));
		}
	}
	None
}

#[test]
fn test_open_software_scanner_ds() {
	helper::init();
	let _twain_mutex = TWAIN_MUTEX.lock();

	if let Some((_dsm, _ds)) = get_software_scanner(helper::get_dsm_entry_wrapper()) {
		assert!(true);
	}
}

#[test]
fn test_enable_software_scanner_ds() {
	helper::init();
	let _twain_mutex = TWAIN_MUTEX.lock();

	if let Some((_dsm, ds)) = get_software_scanner(helper::get_dsm_entry_wrapper()) {
		let ui = TW_USERINTERFACE {
			ShowUI: 0,
			ModalUI: 0,
			hParent: ptr::null_mut(),
		};
		ds.enable(ui).unwrap();
	}
}
