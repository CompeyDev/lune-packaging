use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct LuneReleaseData {
	pub name: String,
	pub assets: Vec<LuneAssetData>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LuneAssetData {
	pub browser_download_url: String,
	pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LuneReleaseFetched {
	pub version: String,
	pub download: String,
	pub artifact_name: String,
	pub raw: Option<LuneReleaseData>,
}
