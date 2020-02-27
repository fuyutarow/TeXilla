build:
	cargo build --release

watch:
	bin/autoexec entry.tex 'cargo run entry.tex'

serve:
	npx live-server -p 8080
