This repository houses the required manifests and build scripts for lune packaging. 
For more information on lune, see [lune-org/lune](https://github.com/lune-org/lune).

## Packaging Statuses

| Platform | Status                                                                                                                                                                    |
|----------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Homebrew      | ![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/CompeyDev/lune-packaging/homebrew_test.yaml?logo=apple&label=%20&color=black) |
| AUR      | ![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/CompeyDev/lune-packaging/aur_test.yaml?logo=archlinux&label=%20&color=black) |
| Scoop    | ![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/CompeyDev/lune-packaging/scoop_test.yaml?logo=windows&logoColor=blue&label=%20&color=black) |
| AppImage    | ![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/CompeyDev/lune-packaging/appimage.yaml?logo=linux&logoColor=yellow&label=%20&color=black) |
| APT    | ![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/CompeyDev/lune-packaging/apt.yaml?logo=debian&logoColor=red&label=%20&color=black) |
| COPR    | [Project](https://copr.fedorainfracloud.org/coprs/devcomp/lune/) |


## Installation
### Homebrew
Since lune has its cask published to Homebrew core, it can be installed as such:

- From precompiled binaries:
```console
brew install lune
```

- From source:
```console
brew install lune --build-from-source
```

### AUR
```console
yay -S lune
### OR ###
yay -S lune-git
### OR ###
yay -S lune-bin
```

### Scoop
```ps
scoop bucket add lune https://github.com/CompeyDev/lune-packaging.git
scoop install lune
```

### AppImage
Go to the [GitHub Actions Page](https://github.com/CompeyDev/lune-packaging/actions/workflows/appimage.yaml), and download the artifact suitable for your architecture from the build artifacts.

### APT

> [!NOTE]
> The signing key was recently changed! If you get errors such as:
> ```
> The following signatures couldn't be verified because the public key is not available: NO_PUBKEY 429EF1C337871656
> ```
>
> You need to run the commands below to import the new GPG signing keys.

- Import the GPG signing keys:
```sh
curl https://id.devcomp.xyz/hi@devcomp.xyz/4.gpg | gpg --import
sudo sh -c "curl https://id.devcomp.xyz/hi@devcomp.xyz/4.gpg | sudo gpg --dearmor > /usr/share/keyrings/lune-archive-keyring.gpg"
```
- Add the repository to `sources.list`:
```md
deb [signed-by=/usr/share/keyrings/lune-archive-keyring.gpg] https://repos.devcomp.xyz/ bookworm main
```
- Install the package:
```console
sudo apt update
sudo apt install lune
```
