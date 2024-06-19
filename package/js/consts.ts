import { parse as parseToml } from "jsr:@std/toml";

export default parseToml(Deno.readTextFileSync("consts.toml")) as {
  name: string;
  api_url: string;
  version: string;
};
