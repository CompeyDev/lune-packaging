import constants from "./consts.ts";

export const GITHUB_RELEASE_API_URL = constants.api_url;

type RawGitHubRelease = {
  url: string;
  assets_url: string;
  upload_url: string;
  html_url: string;
  id: number;
  tag_name: string;
  created_at: string;
  published_at: string;
  assets: RawGitHubReleaseAsset[];
};

type RawGitHubReleaseAsset = {
  url: string;
  id: number;
  node_id: string;
  name: string;
  label: string;
  content_type: string;
  state: string;
  size: number;
  download_count: number;
  created_at: string;
  published_at: string;
  browser_download_url: string;
};

export async function fetchLuneReleases(
  token?: string,
): Promise<RawGitHubRelease[]> {
  const headers: Record<string, string> = token
    ? { "Authorization": "Bearer " + token }
    : {};

  const resp: RawGitHubRelease[] = await fetch(
    new Request(GITHUB_RELEASE_API_URL, {
      headers,
      method: "GET",
    }),
  ).then((resp) => resp.json());

  return resp;
}
