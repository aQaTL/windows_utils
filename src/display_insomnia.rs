use std::ptr;
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::time::Duration;
use winapi::shared::winerror::*;
use winapi::um::datetimeapi::GetTimeFormatEx;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winnls::GetUserDefaultLocaleName;
use winapi::um::{winbase::SetThreadExecutionState, winnt::*};

fn main() {
	let ret = reset_idle_timer();
	if ret != 0 {
		println!("[{}] Idle timer resetter runnning", get_time());
	} else {
		println!("[{}] Failed to reset idle timer", get_time());
	}
	let (_send, recv) = channel::<()>();
	'l: loop {
		match recv.recv_timeout(Duration::from_secs(60)) {
			Err(RecvTimeoutError::Timeout) => {
				let ret = reset_idle_timer();
				if ret == 0 {
					println!("[{}] Failed to reset idle timer", get_time());
				}
			}
			_ => break 'l,
		}
	}
}

fn reset_idle_timer() -> u32 {
	unsafe { SetThreadExecutionState(ES_CONTINUOUS | ES_DISPLAY_REQUIRED) }
}

fn get_time() -> String {
	let time_str = unsafe {
		let mut locale_name = Vec::with_capacity(LOCALE_NAME_MAX_LENGTH);
		let locale_name_len =
			GetUserDefaultLocaleName(locale_name.as_mut_ptr(), locale_name.capacity() as i32)
				as usize;
		locale_name.set_len(locale_name_len);

		let buf_size = GetTimeFormatEx(
			locale_name.as_ptr(),
			0,
			ptr::null(),
			ptr::null(),
			ptr::null_mut(),
			0,
		);

		let mut time_str = Vec::with_capacity(buf_size as usize);
		let ret = GetTimeFormatEx(
			locale_name.as_ptr(),
			0,
			ptr::null(),
			ptr::null(),
			time_str.as_mut_ptr(),
			buf_size,
		) as usize;

		if ret == 0 {
			let unknown_err;
			return format!(
				"Failed to get time: {}",
				match GetLastError() {
					ERROR_INSUFFICIENT_BUFFER => "ERROR_INSUFFICIENT_BUFFER",
					ERROR_INVALID_FLAGS => "ERROR_INVALID_FLAGS",
					ERROR_INVALID_PARAMETER => "ERROR_INVALID_PARAMETER",
					ERROR_OUTOFMEMORY => "ERROR_OUTOFMEMORY",
					err @ _ => {
						unknown_err = err.to_string();
						&unknown_err
					}
				}
			);
		}

		time_str.set_len(ret);
		time_str
	};
	String::from_utf16_lossy(&time_str[..time_str.len() - 1])
}
