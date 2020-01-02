use winapi::um::winuser::*;

fn main() {
	unsafe { main_() }
}

unsafe fn main_() {
	LockWorkStation();
	SendMessageA(HWND_BROADCAST, WM_SYSCOMMAND, SC_MONITORPOWER, 2);
}
