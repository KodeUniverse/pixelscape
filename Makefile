MAKEFLAGS += -j$(shell nproc 2>/dev/null || echo 2)

AARCH64_TOOLCHAIN ?= aarch64-linux-gnu
AARCH64_SYSROOT   ?=

TARGETS = \
	x86_64-unknown-linux-gnu \
	x86_64-pc-windows-gnu \
	i686-pc-windows-gnu \
	aarch64-unknown-linux-gnu

.PHONY: all $(TARGETS) clean

all: $(TARGETS)

x86_64-unknown-linux-gnu:
	cargo build --release --target $@

x86_64-pc-windows-gnu i686-pc-windows-gnu:
	cargo build --release --target $@

aarch64-unknown-linux-gnu:
	CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER="$(CURDIR)/.cargo/aarch64-linker.sh" \
	AARCH64_TOOLCHAIN=$(AARCH64_TOOLCHAIN) \
	AARCH64_SYSROOT="$(AARCH64_SYSROOT)" \
		cargo build --release --target $@

clean:
	cargo clean
