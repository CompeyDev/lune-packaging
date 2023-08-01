#!/bin/sh

set -e 

declare -a aur_packages=("lune-bin" "lune-git" "lune")
root_dir=$(pwd)

for package in "${aur_packages[@]}"
do
  cd "$package" && makepkg

  cd "$root_dir"
done
