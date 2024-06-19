import * as path from "jsr:@std/path";
import { unzip } from "https://deno.land/x/nzip@v1.2.1/mod.ts";
import { fetchLuneReleases } from "./github.ts";
import consts, { BASE_PATH } from "./consts.ts";

export const LUNE_VERSION = consts.version;
export const EXE_EXTENSION = Deno.build.os == "windows" ? ".exe" : "";

async function installLune() {
	let ghAuthToken: string | undefined;

	try {
		/*
      Ideally, this would look like this:

        new TextDecoder().decode(
          (await new Deno.Command("gh", {
            args: ["auth", "token"],
            stdout: "piped",
          }).output()).stdout,
        );

      However, dnt is yet to support Deno.command
    */
		ghAuthToken = new TextDecoder().decode(
			// deno-lint-ignore no-deprecated-deno-api
			await Deno.run({
				cmd: ["gh", "auth", "token"],
				stdout: "piped",
			}).output(),
		);
	} catch (_) {
		// Don't use an auth token, be subjected to GitHub ratelimit
	}
	const releases = await fetchLuneReleases(ghAuthToken);
	const currentRelease = releases.find((release) =>
		release.tag_name === "v" + LUNE_VERSION
	)!;
	const platformRelease = currentRelease.assets.find((asset) =>
		asset.name == `lune-${LUNE_VERSION}-${Deno.build.os}-${Deno.build.arch}.zip`
	);

	if (!platformRelease) {
		throw new Deno.errors.NotFound(
			`UnsupportedPlatform: ${Deno.build.os}-${Deno.build.arch}`,
		);
	}

	const resp = await fetch(platformRelease.browser_download_url);
	if (!resp.ok) {
		throw new Deno.errors.NotFound(
			`Failed to download Lune: ${resp.status}`,
		);
	}

	const zipFile = await Deno.makeTempFile({ suffix: ".zip" });
	const finalDestDir = path.join(BASE_PATH, consts.version);

	const binaryBlob = resp.body!;
	await binaryBlob.pipeTo(
		(await Deno.open(
			zipFile,
			{ write: true },
		)).writable,
	);

	const binaryPaths = await unzip(zipFile, finalDestDir, {
		useWebWorkers: true,
	});

	if (Deno.build.os !== "windows") await Deno.chmod(binaryPaths[0], 0o777);
}

export async function checkAndInstallLune() {
	const luneExePath = path.join(
		BASE_PATH,
		consts.version,
		"lune" + EXE_EXTENSION,
	);

	const luneExists = await Deno.lstat(luneExePath).then(
		(stat) => {
			if (stat.isDirectory) {
				Deno.remove(luneExePath, { recursive: true });
			} else if (stat.isFile) {
				return true;
			}

			return false;
		},
		(err) => {
			if (err instanceof Deno.errors.NotFound) {
				return false;
			}

			throw err;
		},
	);

	if (!luneExists) await installLune();
}

await checkAndInstallLune();
