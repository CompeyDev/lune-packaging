import { parse as parseToml } from "jsr:@std/toml";
import * as path from "jsr:@std/path";

export const BASE_PATH = path.dirname(path.fromFileUrl(import.meta.url));

export default parseToml(
	Deno.readTextFileSync(path.join(BASE_PATH, "consts.toml")),
) as {
	name: string;
	api_url: string;
	version: string;
};
