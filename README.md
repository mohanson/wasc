# WASC

**WASC is a WebAssembly AOT compiler**. The main purpose is to compile the wasm program using the [WASI](https://wasi.dev/) into machine(x86 and RISC-V) code.

**It is still in the early development stage, so the code and documentation may be changed arbitrarily.** Inspired by xuejie's [article](https://xuejie.space/2020_03_03_introduction_to_ckb_script_programming_performant_wasm/).

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

You can find more useful examples in the `./example` and `./res/wasi`.

# Appendix

- [WAVM](https://github.com/WAVM/WAVM)'s is used in WASC to generate object file.

# Licences

MIT
