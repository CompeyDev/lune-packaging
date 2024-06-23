# Deno/Node Packages

This package exports [lune](https://github.com/lune-org/lune), the standalone
luau runtime as a JS package to be used as a dependency in a JS project.

To install the package, run one of the following in your project, substituting
`{VERSION}` with the version of lune you want to install:

```sh
# Install from NPM for node
npm install lune@{VERSION}

# Install for Deno
deno install https://raw.githubusercontent.com/CompeyDev/lune-packaging/v{VERSION}/package/js/bin/lune.ts
```

> **Note**: Only lune versions starting from lune v0.8.6 are supported for the packages.
