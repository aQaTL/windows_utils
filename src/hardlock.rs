use core::ptr;
use std::mem::{size_of, size_of_val};
use winapi::shared::minwindef::{FALSE, HMODULE, MAKELONG};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::psapi::{EnumProcessModules, EnumProcesses, GetModuleBaseNameA};
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use winapi::um::winuser::*;

fn main() {
	unsafe { main_() }
}

unsafe fn main_() {
	LockWorkStation();

	stop_media();

	let processes = get_processes();
	let display_insomnia_process = {
		match processes {
			Some(processes) => processes
				.into_iter()
				.filter_map(|id| get_process_name(id))
				.find(|proc_name| proc_name == "display_insomnia.exe"),
			None => None,
		}
	};

	if display_insomnia_process.is_none() {
		SendMessageA(HWND_BROADCAST, WM_SYSCOMMAND, SC_MONITORPOWER, 2);
	}
}

unsafe fn stop_media() {
	SendMessageA(
		HWND_BROADCAST,
		WM_APPCOMMAND,
		0,
		MAKELONG(0, APPCOMMAND_MEDIA_STOP as u16) as isize,
	);
}

unsafe fn get_processes() -> Option<Vec<u32>> {
	let mut processes = Vec::<u32>::with_capacity(1024);
	let mut processes_size: u32 = 0;
	if EnumProcesses(
		processes.as_mut_ptr(),
		(processes.capacity() * size_of::<u32>()) as u32,
		&mut processes_size as *mut u32,
	) == 0
	{
		return None;
	}
	processes.set_len((processes_size / size_of::<u32>() as u32) as usize);
	Some(processes)
}

unsafe fn get_process_name(proc_id: u32) -> Option<String> {
	if proc_id == 0 {
		return None;
	}
	let process_h = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, proc_id);
	if process_h.is_null() {
		return None;
	}

	let mut hmod: HMODULE = ptr::null_mut();
	let mut cb_needed: u32 = 0;
	if EnumProcessModules(
		process_h,
		&mut hmod as *mut HMODULE,
		size_of_val(&hmod) as u32,
		&mut cb_needed as *mut u32,
	) == 0
	{
		return None;
	}

	let mut process_name: Vec<u8> = Vec::with_capacity(260);
	let process_name_len = GetModuleBaseNameA(
		process_h,
		hmod,
		process_name.as_mut_ptr() as *mut i8,
		(process_name.capacity() * size_of::<u8>()) as u32,
	);
	if process_name_len == 0 {
		return None;
	}
	process_name.set_len(process_name_len as usize);
	String::from_utf8(process_name).ok()
}
