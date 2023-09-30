use std::fs::File;

use action::download::{download_release, install_lune};

fn main() {
    better_panic::install();

    let (zip_path, meta) = download_release().expect("failed to download latest lune release");

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
    .expect("failed to install lune. did we not have perms to write to the required directories?");
}
