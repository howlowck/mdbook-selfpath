all: build
check: build clippy test install compare
build:
	cargo build
clippy:
	cargo clippy
test:
	cargo test
install:
	cargo install --path .
compare: testbook/book
	diff testbook/book/chapter1/index.html testbook/expected/chapter1_index.html || (echo "Differences found!" && exit 1)
	diff testbook/book/Intro.html testbook/expected/intro.html || (echo "Differences found!" && exit 1)
regenerate: testbook/book
	cp testbook/book/chapter1/index.html testbook/expected/chapter1_index.html
	cp testbook/book/Intro.html testbook/expected/intro.html
testbook/book: install
	(cd testbook && mdbook build)