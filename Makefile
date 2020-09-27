TARGET="treesitter-analyzer"
DATA="${HOME}/.local/share/treesitter-analyzer"

all:
	@ cargo build --release

install:
	@ [ -f "$(TARGET)" ] && sudo cp -vf "$(TARGET)" "/usr/bin/$(TARGET)" || true
	@ [ -d "$(DATA)" ] && cp -vf queries/* "$(DATA)/queries" || true

dev:
	@ [ ! -x watchexec ] && watchexec -s SIGKILL -r -e rs -w src 'cargo run' || echo "you need to install watchexec"

.PHONY: dev install
