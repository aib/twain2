use twain::*;
use twain::twain_h::*;
use twain::twain_h_ext::*;
mod helper;

use std::ptr;
use std::sync::Arc;

use parking_lot::Mutex;

static TWAIN_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_open_and_close_dsm() {
	let _twain_mutex = TWAIN_MUTEX.lock();

	let lib = helper::load_twain_lib();

	let mut identity = helper::get_app_identity(false);
	let ret = unsafe { (lib.dsm_entry.unwrap())(&mut identity, ptr::null_mut(), DG_CONTROL as TW_UINT32, DAT_PARENT as TW_UINT16, MSG_OPENDSM as TW_UINT16, ptr::null_mut()) };
	assert_eq!(0, ret);

	let ret = unsafe { (lib.dsm_entry.unwrap())(&mut identity, ptr::null_mut(), DG_CONTROL as TW_UINT32, DAT_PARENT as TW_UINT16, MSG_CLOSEDSM as TW_UINT16, ptr::null_mut()) };
	assert_eq!(0, ret);
}

#[test]
fn test_dsmentrywrapper_open_and_close_dsm() {
	let _twain_mutex = TWAIN_MUTEX.lock();

	let lib = helper::load_twain_lib();
	let wrapper = DSMEntryWrapper::new(lib.dsm_entry);

	let mut identity = helper::get_app_identity(false);
	let res = wrapper.do_dsm_entry(Some(&mut identity), None, DG_CONTROL, DAT_PARENT, MSG_OPENDSM, ptr::null_mut());
	assert_eq!(response::ReturnCode::Success, res.return_code);

	let res = wrapper.do_dsm_entry(Some(&mut identity), None, DG_CONTROL, DAT_PARENT, MSG_CLOSEDSM, ptr::null_mut());
	assert_eq!(response::ReturnCode::Success, res.return_code);
}

#[test]
fn test_openeddsm_new_and_get_data_sources() {
	let _twain_mutex = TWAIN_MUTEX.lock();

	let lib = helper::load_twain_lib();
	let wrapper = DSMEntryWrapper::new(lib.dsm_entry);

	let identity = helper::get_app_identity(false);
	let dsm = OpenedDSM::new(wrapper, identity);
	assert!(dsm.is_ok());

	let dsm = dsm.unwrap();
	let data_sources = dsm.get_data_sources();
	assert!(data_sources.is_ok());
}

fn get_software_scanner(lib: &helper::TwainLib) -> Option<(Arc<OpenedDSM>, Arc<OpenedDS>)> {
	let wrapper = DSMEntryWrapper::new(lib.dsm_entry);
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
	let _twain_mutex = TWAIN_MUTEX.lock();

	let lib = helper::load_twain_lib();
	if let Some((_dsm, _ds)) = get_software_scanner(&lib) {
		assert!(true);
	}
}
