# libsbf

A no_std parser for the SBF (Septentrio Binary Format) using the
[sans-io](https://sans-io.readthedocs.io/) philosophy.

## Fuzz Testing

There is a fuzz test written for the reader using cargo-fuzz (libFuzzer). To run the fuzzer:

```
nix develop
cargo fuzz run reader_fuzz
```

To run with a specific number of jobs or other options:

```
cargo fuzz run reader_fuzz -- -jobs=8 -workers=8
```
