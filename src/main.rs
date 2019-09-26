use libc::{c_char, c_float, ioctl};
use std::fmt;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::mem;

pub const MAX_CHANNELS: usize = 32;

#[repr(C)]
#[derive(Debug)]
struct multi_description {
	info_size: isize,
	interface_version: u32,
	interface_minimum: u32,
	
	friendly_name: [c_char; 32],
	vendor_info: [c_char; 32],
	
	output_channel_count: i32,
	input_channel_count: i32,
	output_bus_channel_count: i32,
	input_bus_channel_count: i32,
	aux_bus_channel_count: i32,
	
	request_channel_count: i32,
	channels: *mut multi_channel_info,
	
	output_rates: u32,
	input_rates: u32,
	min_cvsr_rate: c_float,
	max_cvsr_rate: c_float,
	output_formats: u32,
	input_formats: u32,
	lock_sources: u32,
	timecode_sources: u32,

	interface_flags: u32,
	start_latency: i64,				//bigtime_t
	_reserved: [u32; 11],
	// split because Debug only formats arrays up to 32 characters
	control_panel_1: [c_char; 32],
	control_panel_2: [c_char; 32]
}

#[repr(C)]
#[derive(Debug)]
enum channel_kind {
	NoChannelKind = 0,
	OutputChannel = 0x1,
	InputChannel = 0x2,
	OutputBus = 0x4,
	InputBus = 0x8,
	AuxBus = 0x10
}  

#[repr(C)]
#[derive(Debug)]
struct multi_channel_info {
	channel_id: i32,
	kind: channel_kind,
	designations: u32,
	connectors: u32,
	reserved: [u32; 4]
}

const B_MULTI_GET_DESCRIPTION: i32 = 8000 + 20;

fn main() {
    let f = File::open("/dev/audio/hmulti/hda/0").unwrap();
    let mut description: multi_description = unsafe { mem::zeroed() };
    let mut info_list: [multi_channel_info; MAX_CHANNELS] = unsafe { mem::zeroed() };
	description.request_channel_count = MAX_CHANNELS as i32;
	description.channels = info_list.as_mut_ptr();
    let ret = unsafe {
    	ioctl(f.as_raw_fd(), B_MULTI_GET_DESCRIPTION, &mut description, mem::size_of::<multi_channel_info>())
    };

	println!("descriptor: {:#?}", description);
    
    let channel_count = description.input_channel_count + description.output_channel_count;
	for i in 0..channel_count {
		println!("channel #{}: {:#?}", i, info_list[i as usize]);
	}
}
