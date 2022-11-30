use super::entrypoint::EntryPoints;
use super::twain_h::TW_HANDLE;

use std::ops::Deref;
use std::ptr;

pub struct PointerFromHandle<'a, T> {
	ptr: *mut T,
	on_drop: Box<dyn Fn() + 'a>,
}

impl<'a, T> PointerFromHandle<'a, T> {
	pub fn new(ep: &'a EntryPoints, handle: TW_HANDLE) -> Option<PointerFromHandle<'a, T>> {
		let EntryPoints { lock, unlock, .. } = ep;

		let ptr = lock(handle) as *mut T;
		if ptr == ptr::null_mut() {
			None
		} else {
			let on_drop = Box::new(move || unlock(handle));
			Some(PointerFromHandle { ptr, on_drop })
		}
	}
}

impl<T> Deref for PointerFromHandle<'_, T> {
	type Target = *mut T;

	fn deref(&self) -> &Self::Target {
		&self.ptr
	}
}

impl<T> Drop for PointerFromHandle<'_, T> {
	fn drop(&mut self) {
		(self.on_drop)();
	}
}
