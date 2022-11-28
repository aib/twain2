pub mod entrypoint;
pub mod response;
pub mod twain_h;
pub mod twain_h_ext;

use entrypoint::*;
use response::*;
use twain_h::*;
use twain_h_ext::*;

use std::mem::MaybeUninit;
use std::ptr;
use std::sync::Arc;
use parking_lot::RwLock;

pub struct DSMEntryWrapper {
	entry_proc: DSMENTRYPROC,
}

pub struct OpenedDSM {
	pub app_identity: RwLock<TW_IDENTITY>,
	dsm_entry_wrapper: DSMEntryWrapper,
}

impl DSMEntryWrapper {
	pub fn new(dsm_entry: DSMENTRYPROC) -> Self {
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

		let dsm_entry = self.entry_proc.unwrap(); //FIXME: Workaround for DSMENTRYPROC being Option<>

		let rc = unsafe { dsm_entry(p_origin, p_dest, dg as TW_UINT32, dat as TW_UINT16, msg as TW_UINT16, data) };
		let return_code = ReturnCode::from_rc(rc);

		let mut tw_status: MaybeUninit<TW_STATUS> = MaybeUninit::uninit();
		let src = unsafe { dsm_entry(p_origin, p_dest, DG_CONTROL as TW_UINT32, DAT_STATUS as TW_UINT16, MSG_GET as TW_UINT16, tw_status.as_mut_ptr() as _) };
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

impl OpenedDSM {
	pub fn new(dsm_entry_wrapper: DSMEntryWrapper, app_identity: TW_IDENTITY) -> Result<Arc<Self>, Response> {
		let app_identity = RwLock::new(app_identity);

		log::debug!("Opening TWAIN DSM...");

		let res = dsm_entry_wrapper.do_dsm_entry(Some(&mut app_identity.write()), None, DG_CONTROL, DAT_PARENT, MSG_OPENDSM, ptr::null_mut());
		if !res.is_success() {
			return Err(res);
		}

		Ok(Arc::new(OpenedDSM { app_identity, dsm_entry_wrapper }))
	}

	pub fn get_data_sources(&self) -> Result<Vec<TW_IDENTITY>, Response> {
		let mut data_sources = Vec::new();

		let mut first = true;
		loop {
			let mut identity: TW_IDENTITY = Default::default();
			let res = self.do_dsm_entry(None, DG_CONTROL, DAT_IDENTITY, if first { MSG_GETFIRST } else { MSG_GETNEXT }, &mut identity as *mut TW_IDENTITY as _);
			match res {
				Response { return_code: ReturnCode::Success, .. } => data_sources.push(identity),
				Response { return_code: ReturnCode::EndOfList, .. } => break,
				res => return Err(res),
			}
			first = false;
		}

		Ok(data_sources)
	}

	pub fn do_dsm_entry(&self, dest: Option<&mut TW_IDENTITY>, dg: TwainUConst, dat: TwainUConst, msg: TwainUConst, data: TW_MEMREF) -> Response {
		self.dsm_entry_wrapper.do_dsm_entry(Some(&mut self.app_identity.write()), dest, dg, dat, msg, data)
	}
}

impl Drop for OpenedDSM {
	fn drop(&mut self) {
		log::debug!("Closing TWAIN DSM");
		let res = self.do_dsm_entry(None, DG_CONTROL, DAT_PARENT, MSG_CLOSEDSM, ptr::null_mut());
		if !res.is_success() {
			log::warn!("CLOSEDSM failed: {}", res);
		}
	}
}
