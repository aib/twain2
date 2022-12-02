pub mod data;
pub mod entrypoint;
pub mod response;
pub mod twain_h;
pub mod twain_h_ext;

use entrypoint::*;
use response::*;
use twain_h::*;
use twain_h_ext::*;

use std::fmt;
use std::mem::MaybeUninit;
use std::ptr;
use std::sync::Arc;
use parking_lot::RwLock;

pub struct DSMEntryWrapper {
	entry_proc: Box<dyn Fn(*mut TW_IDENTITY, *mut TW_IDENTITY, TW_UINT32, TW_UINT16, TW_UINT16, TW_MEMREF) -> TW_UINT16>,
	_libloading_library: Option<libloading::Library>,
}

pub struct OpenedDSM {
	pub app_identity: RwLock<TW_IDENTITY>,
	pub entry_points: Option<EntryPoints>,
	dsm_entry_wrapper: DSMEntryWrapper,
}

pub struct OpenedDS {
	pub ds_identity: RwLock<TW_IDENTITY>,
	pub dsm: Arc<OpenedDSM>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DSState {
	SourceOpen,
	SourceEnabled,
	TransferReady,
	Transferring,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DSError {
	InvalidState(DSState),
	BadResponse(Response),
}

fn id_to_label(id: &TW_IDENTITY) -> String {
	tw_str32_to_string(&id.ProductName)
}

impl DSMEntryWrapper {
	pub fn from_dsmentryproc(dsm_entry: DSMENTRYPROC) -> Option<Self> {
		let dsm_entry = dsm_entry?;
		let entry_proc = move |origin: *mut TW_IDENTITY, dest: *mut TW_IDENTITY, dg: TW_UINT32, dat: TW_UINT16, msg: TW_UINT16, data: TW_MEMREF| -> TW_UINT16 {
			unsafe { dsm_entry(origin, dest, dg, dat, msg, data) }
		};
		Some(Self { entry_proc: Box::new(entry_proc), _libloading_library: None })
	}

	pub fn from_libloading_library(library: libloading::Library) -> Option<Self> {
		let dsm_entry_symbol = unsafe { library.get(b"DSM_Entry\0") }.ok()?;
		let dsm_entry: DSMENTRYPROC = Some(*dsm_entry_symbol);
		let entry_proc = move |origin: *mut TW_IDENTITY, dest: *mut TW_IDENTITY, dg: TW_UINT32, dat: TW_UINT16, msg: TW_UINT16, data: TW_MEMREF| -> TW_UINT16 {
			unsafe { (dsm_entry.unwrap())(origin, dest, dg, dat, msg, data) }
		};
		Some(Self { entry_proc: Box::new(entry_proc), _libloading_library: Some(library) })
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

		let dsm_entry = &self.entry_proc;

		let rc = dsm_entry(p_origin, p_dest, dg as TW_UINT32, dat as TW_UINT16, msg as TW_UINT16, data);
		let return_code = ReturnCode::from_rc(rc);

		let mut tw_status: MaybeUninit<TW_STATUS> = MaybeUninit::uninit();
		let src = dsm_entry(p_origin, p_dest, DG_CONTROL as TW_UINT32, DAT_STATUS as TW_UINT16, MSG_GET as TW_UINT16, tw_status.as_mut_ptr() as _);
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

		let use_twain2 = app_identity.read().SupportedGroups & DF_APP2 != 0 && app_identity.read().SupportedGroups & DF_DSM2 != 0;

		let entry_points = if use_twain2 {
			let mut ep: TW_ENTRYPOINT = Default::default();
			let res = dsm_entry_wrapper.do_dsm_entry(Some(&mut app_identity.write()), None, DG_CONTROL, DAT_ENTRYPOINT, MSG_GET, &mut ep as *mut TW_ENTRYPOINT as _);
			if res.is_success() {
				EntryPoints::from_tw_entrypoint(ep)
			} else {
				log::warn!("Unable to obtain TWAIN 2 entry points: {}", res);
				None
			}
		} else {
			None
		}.or_else(|| EntryPoints::os_default());

		Ok(Arc::new(OpenedDSM { app_identity, entry_points, dsm_entry_wrapper }))
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

	pub fn open_data_source(self: &Arc<Self>, ds_identity: TW_IDENTITY) -> Result<Arc<OpenedDS>, Response> {
		OpenedDS::new(self.clone(), ds_identity)
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

impl OpenedDS {
	fn new(dsm: Arc<OpenedDSM>, ds_identity: TW_IDENTITY) -> Result<Arc<Self>, Response> {
		let ds_identity = RwLock::new(ds_identity);

		log::debug!("Opening TWAIN DS \"{}\"", id_to_label(&ds_identity.read()));

		let res = dsm.do_dsm_entry(None, DG_CONTROL, DAT_IDENTITY, MSG_OPENDS, &mut *ds_identity.write() as *mut TW_IDENTITY as _);
		if !res.is_success() {
			return Err(res);
		}

		let opened_ds = Arc::new(Self { dsm, ds_identity });

		let mut callback = TW_CALLBACK2 {
			CallBackProc: Self::callback as _,
			RefCon: &opened_ds as *const _ as _,
			Message: 0, //NOTE: This field seems to be undocumented/unused
		};
		let res = opened_ds.do_dsm_entry(DG_CONTROL, DAT_CALLBACK2, MSG_REGISTER_CALLBACK, &mut callback as *mut TW_CALLBACK2 as _);
		if !res.is_success() {
			log::warn!("Unable to set callback for TWAIN DS \"{}\": {}", id_to_label(&opened_ds.ds_identity.read()), res);
		}

		Ok(opened_ds)
	}

	pub fn do_dsm_entry(&self, dg: TwainUConst, dat: TwainUConst, msg: TwainUConst, data: TW_MEMREF) -> Response {
		self.dsm.do_dsm_entry(Some(&mut self.ds_identity.write()), dg, dat, msg, data)
	}

	extern "C" fn callback(origin: pTW_IDENTITY, dest: pTW_IDENTITY, dg: TW_UINT32, dat: TW_UINT16, msg: TW_UINT16, data: TW_MEMREF) -> TW_UINT16 {
		let origin_id = unsafe { *origin };
		let dest_id = unsafe { *dest };
		let _self = unsafe { &*(data as *const Arc<Self>) };
		log::debug!("TWAIN callback {:08x}/{:04x}/{:04x} \"{}\" -> \"{}\"", dg, dat, msg, id_to_label(&origin_id), id_to_label(&dest_id));
		0
	}
}

impl Drop for OpenedDS {
	fn drop(&mut self) {
		log::debug!("Closing TWAIN DS \"{}\"", id_to_label(&self.ds_identity.read()));

		let res = self.dsm.do_dsm_entry(None, DG_CONTROL, DAT_IDENTITY, MSG_CLOSEDS, &mut *self.ds_identity.write() as *mut TW_IDENTITY as _);
		if !res.is_success() {
			log::warn!("CLOSEDS failed: {}", res);
		}
	}
}

impl fmt::Display for DSState {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		match self {
			Self::SourceOpen    => write!(f, "SourceOpen"),
			Self::SourceEnabled => write!(f, "SourceEnabled"),
			Self::TransferReady => write!(f, "TransferReady"),
			Self::Transferring  => write!(f, "Transferring"),
		}
	}
}

impl fmt::Display for DSError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		match self {
			Self::InvalidState(state) => write!(f, "InvalidState({})", state),
			Self::BadResponse(res)    => write!(f, "BadResponse({})", res),
		}
	}
}
