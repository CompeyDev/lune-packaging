use std::{
    env,
    fs::File,
    io::{Error, ErrorKind, Read, Seek, Write},
    path::PathBuf,
    str::FromStr,
};

use anyhow::Result;
use directories::BaseDirs;
use url::Url;
use zip::ZipArchive;

use crate::{
    github::{FetchResult, Fetcher, GitHub, Method},
    types::{LuneReleaseData, LuneReleaseFetched},
};

fn parse_release_data(data: &LuneReleaseData) -> LuneReleaseFetched {
    const TARGET: &str = "download::parse_release_data";

    tracing::info!(target: TARGET, "Parsing received LuneReleaseData");

    let unknown = &String::from("UNKNOWN");

    let mut version = unknown.to_owned();
    let mut download = unknown.to_owned();
    let mut artifact_name = unknown.to_owned();

    tracing::info!(
        target: TARGET,
        "Defaulting to `unknown` values before parsing"
    );

    for asset in &data.assets {
        let parts = asset
            .name
            .split("-")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        // [0] => always "lune"
        // [1] => version
        // [2] => platform
        // [3] => arch .. ".zip"

        if parts[2] == env::consts::OS && parts[3].trim_end_matches(".zip") == env::consts::ARCH {
            tracing::info!(
                target: TARGET,
                "Supported platform found, overwriting `unknown` values"
            );

            version = (&parts[1]).to_owned();
            download = (&asset.browser_download_url).to_owned();
            artifact_name = (&asset.name).to_owned();
        } else {
            tracing::warn!(
                target: TARGET,
                "Supported platform not found for asset {:?}",
                asset
            );
        }
    }

    tracing::info!(target: TARGET, "Done.");

    LuneReleaseFetched {
        version,
        download,
        artifact_name,
        raw: Some(data.clone()),
    }
}

pub fn download_release(version: Option<String>) -> Result<(PathBuf, LuneReleaseFetched)> {
    const TARGET: &str = "download::download_release";

    tracing::info!(target: TARGET, "Initializing routine");

    let parsed_release_data: LuneReleaseFetched;

    if let Some(ver) = version {
        let artifact_name = format!("lune-{ver}-{}-{}.zip", env::consts::OS, env::consts::ARCH);

        parsed_release_data = LuneReleaseFetched {
            version: ver.to_string(),
            download: format!(
                "https://github.com/filiptibell/lune/releases/download/v{ver}/{artifact_name}"
            ),
            artifact_name,
            raw: None,
        }
    } else {
        parsed_release_data =
            parse_release_data(&GitHub::new(("filiptibell", "lune")).fetch_releases()?);

        tracing::info!(target: TARGET, "Received API resp and parsed release data");
    }

    tracing::info!(target: TARGET, "Got lune {}", parsed_release_data.version);

    let fetcher = Fetcher::new();

    tracing::info!(
        target: TARGET,
        "Proceeding to download release from API resp (url = {})",
        &parsed_release_data.download
    );

    let resp = fetcher.fetch::<_, ()>(
        Method::Get,
        Url::from_str(&parsed_release_data.download.as_str())?,
        false,
    )?;

    match resp {
        FetchResult::Response(res) => {
            tracing::info!(
                target: TARGET,
                "Received download resp, proceeding to write zip to disk at {}",
                &parsed_release_data.artifact_name
            );

            let mut zip_archive = File::create(&parsed_release_data.artifact_name)?;

            let mut bytes = res
                .into_reader()
                .bytes()
                .map(|elem| elem.unwrap())
                .collect::<Vec<u8>>();

            zip_archive.write_all(&mut bytes)?;

            tracing::info!(target: TARGET, "Successfully downloaded release");
        }
        FetchResult::Result(_) => unreachable!("Fetcher returned Result instead of Response"),
    };

    Ok((
        PathBuf::from(&parsed_release_data.artifact_name),
        parsed_release_data,
    ))
}

pub fn install_lune(lune_archive_reader: impl Read + Seek, meta: LuneReleaseFetched) -> Result<()> {
    const TARGET: &str = "download::install_lune";

    tracing::info!(target: TARGET, "Initializing routine");

    let dir_name = meta.artifact_name.trim_end_matches(".zip");

    tracing::info!(target: TARGET, "Proceeding to extract release zip");

    let mut lune_zip = ZipArchive::new(lune_archive_reader)?;
    lune_zip.extract(dir_name)?;

    tracing::info!(target: TARGET, "Successfully extracted release zip");

    let bin_name = match env::consts::OS {
        "windows" => "lune.exe",
        "macos" | "linux" => "lune",
        _ => panic!("unsupported platform"),
    };

    let bin_base_dir = BaseDirs::new()
        .ok_or(Error::new(
            ErrorKind::NotFound,
            "failed to create BaseDirs instance",
        ))?
        .home_dir()
        .join("bin");

    if !bin_base_dir.exists() {
        tracing::warn!(target: TARGET, "bin_base_dir nonexistent, creating");
        std::fs::create_dir(&bin_base_dir)?;
    }

    tracing::warn!(
        target: TARGET,
        "macOS and windows support for this action is incomplete"
    );

    // TODO: macOS and windows support
    let lune_bin_path = bin_base_dir.join(bin_name);

    File::create(&lune_bin_path)?;
    std::fs::rename(PathBuf::from(dir_name).join(bin_name), &lune_bin_path)?;

    tracing::info!(
        target: TARGET,
        "Installed lune bin to {}",
        lune_bin_path.to_string_lossy()
    );

    Ok(())
}
