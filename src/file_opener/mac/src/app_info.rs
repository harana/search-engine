#![cfg(target_os = "macos")]

use swift_rs::*;

pub type NSObject = *mut std::ffi::c_void;

#[repr(C)]
pub struct OpenWithApplication {
	pub name: SRString,
	pub id: SRString,
	pub url: SRString,
	pub icon: SRData,
}

swift!(pub fn get_open_with_applications(url: &SRString) -> SRObjectArray<OpenWithApplication>);
swift!(pub(crate) fn open_file_path_with(file_url: &SRString, with_url: &SRString));

pub fn open_file_paths_with(file_urls: &[&str], with_url: &str) {
	let file_url = file_urls.join("\0");
	unsafe { open_file_path_with(&file_url.as_str().into(), &with_url.into()) }
}
