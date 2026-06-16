#!/bin/sh
# Wrapper that replaces -lgcc_s with -l:libgcc.a for aarch64 cross-compilation.
# Override the toolchain prefix by setting AARCH64_TOOLCHAIN (default: aarch64-linux-gnu).

TOOLCHAIN="${AARCH64_TOOLCHAIN:-aarch64-linux-gnu}"
args=""
sep=""
for arg in "$@"; do
  if [ "$arg" = "-lgcc_s" ]; then
    args="${args}${sep}-l:libgcc.a"
  else
    args="${args}${sep}${arg}"
  fi
  sep=" "
done
exec "$TOOLCHAIN-gcc" $args
