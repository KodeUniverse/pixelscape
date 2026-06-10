#!/bin/sh
# Replace -lgcc_s with -l:libgcc.a for aarch64 cross-compilation
# Using a temp file approach to handle args safely
build=""
sep=""
for arg in "$@"; do
  if [ "$arg" = "-lgcc_s" ]; then
    build="${build}${sep}-l:libgcc.a"
  else
    build="${build}${sep}${arg}"
  fi
  sep=" "
done
exec aarch64-linux-gnu-gcc $build
