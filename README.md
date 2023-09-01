# Mol Parsing

Demo of Rust molecule file format parsing using a grammar-based parser generator.

Grammars are based on the following:

- [Balsa](https://doi.org/10.26434/chemrxiv-2022-01ltp)
- [CTFILE FORMATS](https://discover.3ds.com/sites/default/files/2020-08/biovia_ctfileformats_2020.pdf)
- [PDB File Format Documentation](https://www.wwpdb.org/documentation/file-format)

The main dependencies are:

- [pest](https://crates.io/crates/pest), parser generator
- [serde](https://crates.io/crates/serde), JSON serialization
- [flate2](https://crates.io/crates/flate2), GZIP decompression

See the `Cargo.toml` for details.

## Usage

Ensure you have [rustup installed](https://www.rust-lang.org/learn/get-started). You can also get Rust over conda:

```
conda install -c conda-forge rust
```

Clone and enter this project and run the tool using cargo:

```
git clone https://github.com/PatrickPenner/mol-parsing
cd mol-parsing
cargo run --release  # building for release makes a significant difference when parsing larger databases
```

This should run the build and give you this usage at the end:

```
Usage: main <format: smiles | sdf | sdfgz | pdb> <path to file>
```

You can now run the tool with whatever input file you have, for example the ChEMBL 33 in `.sdf.gz`:

```
cargo run --release sdfgz chembl_33.sdf.gz
```

## Grammars

The grammars can be found in `src/grammar` and you are encouraged to hack around in them. A very useful tool is pest's web-based tool to write grammars at [pest.rs](pest.rs).
