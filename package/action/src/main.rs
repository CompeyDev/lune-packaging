use std::fs::File;

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

	install_lune(
		File::open(&zip_path).unwrap_or_else(|_| panic!("failed to open downloaded lune release zip file @ {}", zip_path.to_string_lossy())),
		meta,
	)
	.expect_or_log("failed to install lune. did we not have perms to write to the required directories?");
}
