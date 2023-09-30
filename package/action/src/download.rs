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
    let unknown = &String::from("UNKNOWN");

    let mut version = unknown.to_owned();
    let mut download = unknown.to_owned();
    let mut artifact_name = unknown.to_owned();

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
            version = (&parts[1]).to_owned();
            download = (&asset.browser_download_url).to_owned();
            artifact_name = (&asset.name).to_owned();
        }
    }

    LuneReleaseFetched {
        version,
        download,
        artifact_name,
        raw: data.clone(),
    }
}

pub fn download_release() -> Result<(PathBuf, LuneReleaseFetched)> {
    let parsed_release_data =
        parse_release_data(&GitHub::new(("filiptibell", "lune")).fetch_releases()?);

    let fetcher = Fetcher::new();
    let resp = fetcher.fetch::<_, ()>(
        Method::Get,
        Url::from_str(&parsed_release_data.download.as_str())?,
        false,
    )?;

    match resp {
        FetchResult::Response(res) => {
            let mut zip_archive = File::create(&parsed_release_data.artifact_name)?;

            let mut bytes = res
                .into_reader()
                .bytes()
                .map(|elem| elem.unwrap())
                .collect::<Vec<u8>>();

            zip_archive.write_all(&mut bytes)?
        }
        FetchResult::Result(_) => unreachable!("Fetcher returned Result instead of Response"),
    };

    Ok((
        PathBuf::from(&parsed_release_data.artifact_name),
        parsed_release_data,
    ))
}

pub fn install_lune(lune_archive_reader: impl Read + Seek, meta: LuneReleaseFetched) -> Result<()> {
    let dir_name = meta.artifact_name.trim_end_matches(".zip");

    let mut lune_zip = ZipArchive::new(lune_archive_reader)?;
    lune_zip.extract(dir_name)?;

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
        std::fs::create_dir(&bin_base_dir)?;
    }

    // TODO: macOS and windows support
    let lune_bin_path = bin_base_dir.join(bin_name);

    File::create(&lune_bin_path)?;
    std::fs::rename(PathBuf::from(dir_name).join(bin_name), lune_bin_path)?;

    Ok(())
}
