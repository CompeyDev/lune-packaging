import consts, { BASE_PATH } from "../consts.ts";
import * as path from "jsr:@std/path";
import { checkAndInstallLune, EXE_EXTENSION } from "../install.ts";

// Install lune, if unavailable
await checkAndInstallLune();

// Wrap around the lune executable
/*
  dnt does not support Deno.Command yet, but once they do, this would
  like so:

    new Deno.Command(path.join(BASE_PATH, "lune" + EXE_EXTENSION), {
      args: Deno.args,
      stdout: "inherit",
      stderr: "inherit",
      stdin: "inherit",
    }).spawn();
*/
// deno-lint-ignore no-deprecated-deno-api
const luneStatus = await Deno.run({
	cmd: [
		path.join(BASE_PATH, consts.version, "lune" + EXE_EXTENSION),
		...Deno.args,
	],
	stdout: "inherit",
	stderr: "inherit",
	stdin: "inherit",
}).status();

Deno.exit(luneStatus.signal);
