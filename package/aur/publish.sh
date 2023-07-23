#!/bin/bash

set -e

source ./utils.sh

function build() {
  # Get the name of the package to build from the current working directory
  dirname=$(pwd)
  
  shopt -s extglob        
  cwd_name=${dirname%%+(/)}   
  cwd_name=${cwd_name##*/} 
  cwd_name=${cwd_name:-/}  

  # Move the PKGBUILD for our package into the working directory
  mv ../packaging_scripts/$cwd_name.PKGBUILD PKGBUILD

  # Increment the `pkgver`
  current_version=$(get_current_version)
  new_version=`increment_version $current_version 2> /dev/null`
  sed -i 's/$current_version/$new_version/g' PKGBUILD

  # Generate `.SRCINFO`
  makepkg --printsrcinfo > .SRCINFO
}

function test() {
  # Test with our modified `PKGBUILD` in a temporary directory
  orig_dir=$(pwd)
  testing_dir=$(mktmp)
  cp PKGBUILD $testing_dir/ && cd $testing_dir
  
  makepkg

  # Go back to our original directory if all goes well
  cd $orig_dir
}

function push() {
  pkgname=$1
  version=$2
  
  git add PKGBUILD .SRCINFO
  git commit -m "chore(release): $1 v$2"
}

# Move our AUR specific config for SSH
mv ./ssh_config ~/.ssh/config

# Declare our AUR git URLs
declare -a aur_ssh_urls=("ssh://aur@aur.archlinux.org/lune.git" "ssh://aur@aur.archlinux.org/lune-git.git")

# As for now, we only publish the precompiled binary to the AUR
for ssh_url in "${aur_ssh_urls[@]}"
do
  if [ $ssh_url -ne "ssh://aur@aur.archlinux.org/lune-git.git" ]; then
    # TODO: Don't just directly try to cd into `lune` in the future
    git clone $ssh_url && cd lune
    build && test && push lune `get_current_version`
  fi
done
