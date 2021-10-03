//! VGA (Video Graphics Array) is an interface allowing to display video on screens connected to
//! the computer.
//! Nowdays, this specification is obsolete and is used for backward compatibility.

#![no_std]

extern crate kernel;

use kernel::io;
use kernel::module::version::Version;

/// The VGA controller base physical address.
pub const BASE_ADDR: *const u8 = 0xa0000 as _;

kernel::module!("vga", Version::new(1, 0, 0));

/// Defines the planes affected by write operations. Bit 0 correspond to plane 0, bit 1 to plane 1,
/// etc...
fn set_map_mask(mask: u8) {
	unsafe {
		io::outb(0x3c4, 0x02);
		let val = io::inb(0x3c5);

		io::outb(0x3c4, 0x02);
		io::outb(0x3c5, (val & 0b11110000) | (mask & 0b1111));
	}
}

/// Sets the timing mode.
/// `clock`: If true, the clock runs at 28 MHz. If false, it runs at 25 MHz.
/// `dot`: If true, a character is 8 pixels wide. If false, a character is 9 pixels wide.
fn set_timing_mode(clock: bool, dot: bool) {
	// Setting clock
	unsafe {
		let val = io::inb(0x3cc);
		io::outb(0x3c2, (val & 0b11110011) | ((clock as u8) << 2));
	}

	// Setting dot
	unsafe {
		io::outb(0x3c4, 0x01);
		let val = io::inb(0x3c5);

		io::outb(0x3c4, 0x01);
		io::outb(0x3c5, (val & !0b1) | ((dot as u8) & 0b1));
	}
}

/// Called on module load
#[no_mangle]
pub extern "C" fn init() -> bool {
	// TODO Create device file

	true
}

/// Called on module unload
#[no_mangle]
pub extern "C" fn fini() {}
