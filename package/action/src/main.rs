use std::fs::File;

use action::{
    download::{download_release, install_lune},
    fmt::LogFormatter,
};
use actions_core as core;
use tracing::Level;
use tracing_unwrap::ResultExt;

fn main() {
    println!(
        "debug mode: {}",
        (core::is_debug() || cfg!(debug_assertions))
    );

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

    let (zip_path, meta) =
        download_release().expect_or_log("failed to download latest lune release");

    install_lune(
        File::open(&zip_path).expect(
            format!(
                "failed to open downloaded lune release zip file @ {}",
                zip_path.to_string_lossy().to_string()
            )
            .as_str(),
        ),
        meta,
    )
    .expect_or_log(
        "failed to install lune. did we not have perms to write to the required directories?",
    );
}
