import { build, emptyDir } from "jsr:@deno/dnt";
import consts from "./consts.ts";

await emptyDir("./npm");

await build({
  scriptModule: false,
  packageManager: "pnpm",
  entryPoints: [{
    kind: "bin",
    name: consts.name,
    path: "./bin/lune.ts",
  }],
  outDir: "./npm",
  shims: {
    deno: true,
    undici: true,
  },
  package: {
    name: consts.name,
    version: consts.version,
    description: "",
    license: "MPL-2.0",
    repository: {
      type: "git",
      url: "git+https://github.com/CompeyDev/lune-packaging.git",
    },
    bugs: {
      url: "https://github.com/CompeyDev/lune-packaging/issues",
    },
  },
  postBuild() {
    Deno.copyFileSync("LICENSE.md", "npm/LICENSE.md");
    Deno.copyFileSync("README.md", "npm/README.md");
    Deno.copyFileSync("consts.toml", "npm/consts.toml");
  },
  typeCheck: false, // FIXME: This is a problem on dnt's end while importing things
});
