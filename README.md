[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)


# `picolib`
Welcome to `picolib` 🎉

`picolib` is a Rust library that offers a tiny part of the Raspberry Pi Pico C-SDK APIs as delegates that can be called
from Rust.


## Files
- `/`
  - `c`: Contains the API-defining header file and a default implementation
  - `src`: Contains the Rust source code


## Design
This library defines a simple C delegate-API that can be used to access some of the Pico's C-SDK-APIs.
Your C-SDK application is responsible to implement and export the necessary symbols (a default implementation is
provided in `c/pico_delegates.*`), but then you can call back into the C-world and use some of the C-SDK's functions.

An example call stack could like this:
1. `main` (Pico-C-SDK)
2. `rust_entry` (the entry point of your rust library)
3. `delegates::pico_stdio_getc` (picolib)
4. `pico_stdio_getc` (C delegate implemetation)
5. `getc` (Pico-C-SDK)
