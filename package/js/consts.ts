import * as path from "jsr:@std/path";
import { ensureDir } from "jsr:@std/fs";
import home_dir from "https://deno.land/x/dir@1.5.2/home_dir/mod.ts";

export const BASE_PATH = path.join(home_dir()!, ".lune", "versions");
await ensureDir(BASE_PATH);

export default {
	name: "lune",
	version: "0.8.7",
	api_url: "https://api.github.com/repos/lune-org/lune/releases",
} as const;
