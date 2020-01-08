use std::{
	io::{stdin, stdout, Write},
	path::PathBuf,

};
use winreg::{
	enums::{RegType, HKEY_CURRENT_USER},
	RegKey, RegValue,
};
use byteorder::ReadBytesExt;

#[derive(Debug, failure_derive::Fail)]
enum ProgramError {
	#[fail(display = "argument must be a valid path")]
	InvalidArguments,
}

fn main() {
	match main_() {
		Ok(_) => (),
		Err(e) => eprintln!("Error: {}", e),
	}
}

fn main_() -> Result<(), failure::Error> {
	let path = std::env::args()
		.nth(1)
		.ok_or(ProgramError::InvalidArguments)?;
	let path = PathBuf::from(path);
	if !path.exists() || path.is_dir() {
		Err(ProgramError::InvalidArguments)?;
	}
	let path = path.canonicalize()?.into_os_string().into_string().unwrap();
	let final_path = path.trim_start_matches(r"\\?\");

	println!("Your path: \"{}\"", final_path);
	print!("Okay? [y/N] ");
	stdout().flush()?;

	if std::str::from_utf8(&[stdin().read_u8()?])? != "y" {
		println!("Exiting");
		return Ok(());
	}

	let path_encoded = unsafe {
		let mut path_encoded = final_path.encode_utf16().collect::<Vec<u16>>();
		let path_len = path_encoded.len() * 2;
		let path_cap = path_encoded.capacity() * 2;
		let path_u8 = std::mem::transmute::<*mut u16, *mut u8>(path_encoded.as_mut_ptr());
		std::mem::forget(path_encoded);
		Vec::from_raw_parts(path_u8, path_len, path_cap)
	};

	let hkcr = RegKey::predef(HKEY_CURRENT_USER);
	let (cmd_key, _disposition) = hkcr.create_subkey(r"Software\Microsoft\Command Processor")?;
	let reg_value = RegValue {
		bytes: path_encoded,
		vtype: RegType::REG_SZ,
	};
	cmd_key.set_raw_value("AutoRun", &reg_value)?;

	println!("Value set successfully");
	Ok(())
}
