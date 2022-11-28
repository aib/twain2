pub mod response;
pub mod twain_h;
pub mod twain_h_ext;

use response::*;
use twain_h::*;
use twain_h_ext::*;

use std::mem::MaybeUninit;
use std::ptr;

pub struct DSMEntryWrapper {
	entry_proc: DSMEntryProc,
}

impl DSMEntryWrapper {
	pub fn new(dsm_entry: DSMEntryProc) -> Self {
		Self { entry_proc: dsm_entry }
	}

	pub fn do_dsm_entry(&self, origin: Option<&mut TW_IDENTITY>, dest: Option<&mut TW_IDENTITY>, dg: TwainUConst, dat: TwainUConst, msg: TwainUConst, data: TW_MEMREF) -> Response {
		let p_origin = match origin {
			None => ptr::null_mut(),
			Some(r) => r as *mut TW_IDENTITY,
		};

		let p_dest = match dest {
			None => ptr::null_mut(),
			Some(r) => r as *mut TW_IDENTITY,
		};

		let rc = unsafe { (self.entry_proc)(p_origin, p_dest, dg as TW_UINT32, dat as TW_UINT16, msg as TW_UINT16, data) };
		let return_code = ReturnCode::from_rc(rc);

		let mut tw_status: MaybeUninit<TW_STATUS> = MaybeUninit::uninit();
		let src = unsafe { (self.entry_proc)(p_origin, p_dest, DG_CONTROL as TW_UINT32, DAT_STATUS as TW_UINT16, MSG_GET as TW_UINT16, tw_status.as_mut_ptr() as _) };
		let status_return_code = ReturnCode::from_rc(src);

		let condition_code = if status_return_code == ReturnCode::Success {
			let tw_status = unsafe { tw_status.assume_init() };
			ConditionCode::from_cc(tw_status.ConditionCode)
		} else {
			ConditionCode::NoConditionCode(status_return_code)
		};

		Response { return_code, condition_code }
	}
}
