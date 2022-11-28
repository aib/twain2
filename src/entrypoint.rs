use super::twain_h::{TW_ENTRYPOINT, TW_HANDLE, TW_MEMREF, TW_UINT32};

pub struct EntryPoints {
	pub allocate: Box<dyn Fn(TW_UINT32) -> TW_HANDLE + Send + Sync>,
	pub free:     Box<dyn Fn(TW_HANDLE) + Send + Sync>,
	pub lock:     Box<dyn Fn(TW_HANDLE) -> TW_MEMREF + Send + Sync>,
	pub unlock:   Box<dyn Fn(TW_HANDLE) + Send + Sync>,
}

impl EntryPoints {
	pub fn from_tw_entrypoint(ep: TW_ENTRYPOINT) -> Option<EntryPoints> {
		let allocate = ep.DSM_MemAllocate.map(|f| {
			Box::new(move |size| unsafe {
				(f)(size)
			})
		})?;

		let free = ep.DSM_MemFree.map(|f| {
			Box::new(move |handle| unsafe {
				(f)(handle)
			})
		})?;

		let lock = ep.DSM_MemLock.map(|f| {
			Box::new(move |handle| unsafe {
				(f)(handle)
			})
		})?;

		let unlock = ep.DSM_MemUnlock.map(|f| {
			Box::new(move |handle| unsafe {
				(f)(handle)
			})
		})?;

		Some(EntryPoints { allocate, free, lock, unlock })
	}
}
