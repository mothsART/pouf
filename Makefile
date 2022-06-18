# Install to /usr unless otherwise specified, such as `make PREFIX=/app`
PREFIX=/usr

# What to run to install various files
INSTALL=install -D
# Run to install the actual binary
INSTALL_PROGRAM=$(INSTALL)
# Run to install application data, with differing permissions
INSTALL_DATA=$(INSTALL) -m 644

# Directories into which to install the various files
bindir=$(DESTDIR)$(PREFIX)/bin
sharedir=$(DESTDIR)$(PREFIX)/share

.PHONY: clean clean-all install copy-data uninstall cargo-publish

# Build the application
target/release/pouf: src
	cargo build --release

test:
	cargo test

install: target/release/pouf copy-data
	# Install binary
	$(INSTALL_PROGRAM) target/release/pouf $(bindir)/pouf

copy-data:
	$(INSTALL_DATA) pouf.bash $(sharedir)/bash-completion/completions/pouf.bash
	$(INSTALL_DATA) pouf.fish $(sharedir)/fish/vendor_completions.d/pouf.fish
	$(INSTALL_DATA) _pouf $(sharedir)/zsh/site-functions/_pouf

uninstall:
	rm -f $(sharedir)/bash-completion/completions/pouf.bash
	rm -f $(sharedir)/fish/vendor_completions.d/pouf.fish
	rm -f $(sharedir)/zsh/site-functions/_pouf
	# Remove the binary
	rm -f $(bindir)/pouf

cargo-publish:
	cargo clippy && cargo fmt && cargo publish --no-verify

clean:
	cargo clean
