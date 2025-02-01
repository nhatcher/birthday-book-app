# Python bindings for Book

You will need to have [Rust](https://www.rust-lang.org/tools/install), [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) and [Python](https://www.python.org/downloads/) (>=3.10) installed in your system.

## Install dependencies

Install the python environment:

```bash
$ python -m venv .env
$ source .env/bin/activate
(.env) $ pip install maturin
```

## Build and test Python

Run

```bash
$ source .env/bin/activate
(.env) $ maturin develop
(.env) $ python
Python 3.10.6 (main, Mar 10 2023, 10:55:28) [GCC 11.3.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> import pybook
```

## Run tests:

```bash
$ source .env/bin/activate
(.env) $ maturin develop
(.env) $ python test.py
All tests pass.
```
