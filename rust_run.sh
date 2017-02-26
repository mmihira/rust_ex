#!/bin/bash
base_path="/home/mihira/c/rust_ex/target/"

exe_name=$(echo "$1" | sed 's/\.rs//g' | sed 's/\/.*\///g')
echo "exe_name :  $exe_name"
echo 'sdfsfsdf'
if [ -e "$base_path$exe_name" ]; then
  rm "$base_path$exe_name"
fi

$(rustc $1 --out-dir "$base_path")
if [ $? -eq 0 ]; then
  "$base_path$exe_name"
fi
