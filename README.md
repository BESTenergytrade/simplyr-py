# SimplyR Python Lib
Python wrapper of the simplyr-lib market matching algorithms implemented in RUST

## Getting Started
Use Pyo3 Lib to wrap the rust library into a python lib.

```
python -m venv venv/
source venv/bin/activate
pip install maturin
```

To install the lib to your current python environment use:
```
maturin develop
```

Run example:
```
python run_matching.py
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE))
 * MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.