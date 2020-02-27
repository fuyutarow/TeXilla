tex?=entry.tex
port?=8080

build:
	cargo build --release
	cp ./target/release/TeXilla ./bin/

watch:
	bin/autoexec "$(tex)" './bin/TeXilla "$(tex)"'

serve:
	npx live-server ./result.html --port=$(port)
