#!/bin/bash

set -e

if [ $ACTIONS_RUNNER_DEBUG  == "true" ]; then
  set -x
fi

source ./utils.sh

function build() {
  log "*" "Initializing PKGBUILD patch/build step"

  # Get the name of the package to build from the current working directory
  dirname=$(pwd)
  
  shopt -s extglob        
  cwd_name=${dirname%%+(/)}
  cwd_name=${cwd_name##*/}
  cwd_name=${cwd_name:-/}

  # Move the PKGBUILD for our package into the working directory
  mv ../packaging_scripts/$cwd_name.PKGBUILD PKGBUILD

  log "#" "Moved \`PKGBUILD\` into working dir"

  # Increment the `pkgver`
  current_version=$(get_current_version)
  new_version=`increment_version $current_version 2> /dev/null`
  sed -i 's/$current_version/$new_version/g' PKGBUILD

  log "#" "Incremented \`pkgver\`"

  # Generate `.SRCINFO`
  makepkg --printsrcinfo > .SRCINFO

  log "#" "Generated \`.SRCINFO\`"
}

function test() {
  log "*" "Initializing makepkg test step"

  # Test with our modified `PKGBUILD` in a temporary directory
  orig_dir=$(pwd)
  testing_dir=$(mktemp)
  cp PKGBUILD $testing_dir/PKGBUILD && cd $testing_dir

  log "#" "Moved \`PKGBUILD\` into testing directory $testing_dir"
  
  makepkg

  log "#" "Finished testing with makepkg"

  # Go back to our original directory if all goes well
  cd $orig_dir
}

function push() {
  pwd 
  log "*" "Initializing AUR push step"

  pkgname=$1
  version=$2

  log "#" "Committing changes to AUR repository"
  
  git add PKGBUILD .SRCINFO
  git commit -m "chore(release): $1 v$2"
  git push

  log "#" "Pushed commit to AUR"
}

# Move our AUR specific config for SSH
mv ./ssh_config ~/.ssh/config

# Declare our AUR git URLs
declare -a aur_ssh_urls=("ssh://aur@aur.archlinux.org/lune.git" "ssh://aur@aur.archlinux.org/lune-git.git")

# As for now, we only publish the precompiled binary package to the AUR
# TODO: For the `lune` package, we need to also replace the sha256 sums in the `PKGBUILD`
for ssh_url in "${aur_ssh_urls[@]}"
do
  if [ $ssh_url != "ssh://aur@aur.archlinux.org/lune-git.git" ]; then
    log "*" "Cloning AUR package from $ssh_url"

    # NOTE: We need to have our public and private keys in `~/.ssh/aur.pub` & `~/.ssh/aur` respectively
    # TODO: Don't just directly try to cd into `lune` in the future
    git clone $ssh_url && pwd && cd lune

    log "*" "Building, testing and pushing changes to AUR"

    build && test && ls && push lune `get_current_version`
  fi
done

log "*" "AUR packaging routine complete."
