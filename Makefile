TARGETS = \
	x86_64-unknown-linux-gnu \
	x86_64-pc-windows-gnu \
	i686-pc-windows-gnu \
	aarch64-unknown-linux-gnu

.PHONY: all $(TARGETS) clean

all: $(TARGETS)

$(TARGETS):
	cargo build --release --target $@

clean:
	cargo clean
