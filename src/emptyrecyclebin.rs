use std::ptr;
use winapi::um::shellapi::*;

fn main() {
	unsafe { main_() }
}

unsafe fn main_() {
	SHEmptyRecycleBinA(
		ptr::null_mut(),
		ptr::null_mut(),
		SHERB_NOCONFIRMATION | SHERB_NOPROGRESSUI | SHERB_NOSOUND,
	);
}
