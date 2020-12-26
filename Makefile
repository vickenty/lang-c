.PHONY: all lib

peg?=rust-peg

all: src/parser.rs
	cargo b --lib
	cargo t

trace:
	rm -f src/parser.rs
	make peg=rust-peg-trace

.INTERMEDIATE: src/parser.rs.raw src/parser.rs.pst src/parser.rs.fmt

src/parser.rs.raw: grammar.rustpeg grammar.rustfmt grammar.header grammar.sed
	$(peg) $< > $@

src/parser.rs.pst: src/parser.rs.raw
	sed -f grammar.sed $< > $@

src/parser.rs.fmt: src/parser.rs.pst
	rustfmt --config-path grammar.rustfmt < $< > $@

src/parser.rs: src/parser.rs.fmt
	cat grammar.header $< > $@

check:
	test src/parser.rs -nt grammar.rustpeg
