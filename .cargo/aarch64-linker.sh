#!/bin/sh
# Wrapper that replaces -lgcc_s with -l:libgcc.a for aarch64 cross-compilation.
# Environment variables:
#   AARCH64_TOOLCHAIN  — cross-compiler prefix (default: aarch64-linux-gnu)
#   AARCH64_SYSROOT    — sysroot path (optional, set on Fedora where the compiler's
#                        built-in sysroot may be empty)

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
if [ -n "$AARCH64_SYSROOT" ]; then
  exec "$TOOLCHAIN-gcc" --sysroot="$AARCH64_SYSROOT" $args
else
  exec "$TOOLCHAIN-gcc" $args
fi
