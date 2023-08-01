#!/bin/sh

set -e 

declare -a aur_packages=("lune-bin" "lune-git" "lune")
root_dir=$(pwd)

for package in "${aur_packages[@]}"
do
  echo "#############################"
  echo "##### Building $package #####"
  echo "#############################"
  
  cd "$package" && makepkg || exit 1

  cd "$root_dir"
done
