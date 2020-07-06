# WASC

**WASC is a WebAssembly AOT compiler**

Inspired by xuejie's [article](https://xuejie.space/2020_03_03_introduction_to_ckb_script_programming_performant_wasm/), and completed the subsequent work.

**It is still in the early development stage, so the code and documentation may be changed arbitrarily.**

# Build and test

**Ubuntu 18.04**

```sh
$ apt install llvm-9
$ git clone https://github.com/mohanson/wasc
$ cd wasc
$ ./build.sh
$ ./build_test.sh
```

An example is the best way to show how it works:

```sh
$ ./build/wasc example/echo.wasm
$ ./example/echo Hello World!
# Hello World!
```

# Appendix

- [WAVM](https://github.com/WAVM/WAVM)'s is used in WASC to generate object file.

# Licences

MIT
