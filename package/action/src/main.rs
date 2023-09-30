use std::fs::{read_dir, remove_dir_all, remove_file, DirEntry, File};

use actions_core as core;
use setup_lune::{
	download::{download_release, install_lune},
	fmt::LogFormatter,
};
use tracing::Level;
use tracing_unwrap::ResultExt;

fn main() {
	let version = match core::input("version") {
		Ok(val) => Some(val),
		Err(_) => None,
	};

	if cfg!(debug_assertions) {
		better_panic::install();
	}

	tracing_subscriber::fmt()
		.with_max_level(match core::is_debug() || cfg!(debug_assertions) {
			true => Level::DEBUG,
			false => Level::INFO,
		})
		.event_format(LogFormatter)
		.init();

	let (zip_path, meta) = download_release(version).expect_or_log("failed to download latest lune release");

	let mut install_path = install_lune(
		File::open(&zip_path).unwrap_or_else(|_| panic!("failed to open downloaded lune release zip file @ {}", zip_path.to_string_lossy())),
		meta,
	)
	.expect_or_log("failed to install lune. did we not have perms to write to the required directories?");

	install_path.pop();

	core::add_path(install_path.to_string_lossy());

	// Cleanup downloaded & unzipped stuff
	let to_remove = read_dir(".")
		.expect_or_log("failed to read current working directory")
		.filter(|f| {
			f.as_ref()
				.expect_or_log("failed to read file of directory")
				.file_name()
				.to_string_lossy()
				.starts_with("lune-")
		})
		.map(|elem| elem.unwrap())
		.collect::<Vec<DirEntry>>();

	for entry in to_remove {
		let file_type = entry.file_type().expect_or_log("failed to get filetype for cleanup");

		if file_type.is_file() {
			remove_file(entry.path()).expect("failed to cleanup after installation!")
		}

		if file_type.is_dir() {
			remove_dir_all(entry.path()).expect("failed to cleanup after installation!")
		}
	}
}
