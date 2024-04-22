#!/bin/sh

set -e 

export CARGO_TERM_COLOR=always

declare -a aur_packages=("lune-bin" "lune-git" "lune")
root_dir=$(pwd)

for package in "${aur_packages[@]}"
do
  echo "#############################"
  echo "##### Building $package #####"
  echo "#############################"
  
  cd "$package" && makepkg -s || exit 1

  cd "$root_dir"
done
