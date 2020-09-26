dev:
	@ [ ! -x watchexec ] && watchexec -s SIGKILL -r -e rs -w src 'cargo run' || echo "you need to install watchexec"
