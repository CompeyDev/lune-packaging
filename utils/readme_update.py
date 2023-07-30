from platform import platform
import tomllib as toml

README_HEADER = """This repository houses the require manifests and build scripts for lune packaging. For more information on lune, see [filiptibell/lune](https://github.com/filiptibell/lune).
"""
MARKDOWN_TABLE_HEADER = """| Package Registry |  Status  |
|------|------|
"""

__VERSION__ = f"v0.1.0-{platform()}"

def main():
    print('running readme_updater ', __VERSION__)
    with open("status.toml", 'rb') as status_file:
        packaging_statuses = toml.load(status_file)
        
        generated_md_table = MARKDOWN_TABLE_HEADER + ""

        for package, status in packaging_statuses.items():
            generated_md_table += f"{package} | {status}"

        updated_readme = """<details>
  <summary>
    Packaging Status
  </summary>
  {}
</details>
        """.format(generated_md_table)
        
    print(generated_md_table)


if __name__ == "__main__":
    main()
