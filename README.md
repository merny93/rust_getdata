# GetData bindings for Rust

This project aims to wrap a sufficient subset of the reference implementation [GetData](https://getdata.sourceforge.net/index.html) for the [Dirfile Standard](https://getdata.sourceforge.net/dirfile.html). The development of these bindings is tied to the development of agent software for the [Taurus experiment](https://sites.wustl.edu/cmblab/research/taurus/). These bindings come with no guarantees and are minimally tested. If the binding for the function you want is not present (or does not work) please open an issue or submit a pull-request.

_As of April 2024 only enough was wrapped to `putdata` of type `RAW`_

## Building

Low level bindings are built using [`bindgen`](https://rust-lang.github.io/rust-bindgen/) and thus require a valid install of `GetData`. The build script (`build.rs`) assumes that `getdata.h` is in the usual location, if this is not the case it must be updated accordingly. To build the project run `cargo build`, a small test-suite can be run using `cargo test` to confirm basic functionality (although the library is in active development thus a failing test is not necessarily a sign of a failed build)

## API overview

In addition to providing a `ffi` to _all_ `GetData` function the bindings provide a `rust-y` API for common functionality which does not require `unsafe` blocks. The API is loosely designed to mimic the `python` bindings and provides 2 main building blocks `Entry` which encapsulates the `gd_entry_t` allowing access to field metadata and `Dirfile` which encapsulates the `DIRFILE` pointer and allows access to the Dirfile.
