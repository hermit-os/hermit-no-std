#![no_std] // don't link the Rust standard library
#![no_main]

#[macro_use]
extern crate log;
extern crate alloc;
extern crate hermit;

use alloc::vec;
use core::str;

use hermit::fs::{self, File};
use hermit::io::Read;
use hermit::scheduler::task::NORMAL_PRIO;
use hermit::scheduler::{join, shutdown, spawn};

#[derive(Debug, PartialEq)]
pub enum LoaderError {
	IoError(i32),
}

fn read_file(fname: &str) -> Result<(), LoaderError> {
	let meta = fs::metadata(fname)
		.map_err(|e| LoaderError::IoError(num::ToPrimitive::to_i32(&e).unwrap()))?;
	let len = meta.len();
	let mut file = File::open(fname)
		.map_err(|e| LoaderError::IoError(num::ToPrimitive::to_i32(&e).unwrap()))?;

	let mut buffer = vec![0; len];
	file.read(&mut buffer)
		.map_err(|e| LoaderError::IoError(num::ToPrimitive::to_i32(&e).unwrap()))?;

	info!("{}: {}", fname, str::from_utf8(&buffer).unwrap());

	Ok(())
}

extern "C" fn foo(_: usize) {
	info!("Hello from foo");

	read_file("/proc/version").unwrap();
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn runtime_entry(_argc: i32, _argv: *const *const u8, _env: *const *const u8) -> ! {
	info!("Start user-level process to initialize the HermitOS");

	let id = unsafe { spawn(foo, 0, NORMAL_PRIO, hermit::DEFAULT_STACK_SIZE, -1) };
	let _ = join(id);

	shutdown(0);
}
