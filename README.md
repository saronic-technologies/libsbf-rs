# libsbf

A no_std parser for the SBF (Septentrio Binary Format) using the
[sans-io](https://sans-io.readthedocs.io/) philosophy.

## Fuzz Testing

There is a fuzz test written for the base parser. To run the test do
the following:

```
nix develop
cd sbf-parser-fuzz
cargo afl build --release
cargo afl fuzz -i in -o out ../target/release/sbf-parser-fuzz

## fuzz testing std iterator
cargo afl build --bin reader-fuzz --release
cargo afl fuzz -i in -o out ../target/release/sbf-parser-fuzz
```
