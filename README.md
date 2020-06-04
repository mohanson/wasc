# WASC

**WASC is a WebAssembly AOT compiler**

Inspired by xuejie's [article](https://xuejie.space/2020_03_03_introduction_to_ckb_script_programming_performant_wasm/), and completed the subsequent work.

# Build and test

WASC uses [WAVM](https://github.com/WAVM/WAVM) and [LLVM-9](https://llvm.org/) to compile WebAssembly code to machine code. It has no runtime, which means that the compilation result is just a binary file that can be run separately.

Let's demonstrate the installation steps under ubuntu system(I will supplement other systems later):

```sh
$ apt install llvm-9

$ git clone https://github.com/mohanson/wasc
$ cd wasc
$ ./build.sh
```

Run tests:

```sh
$ cargo test -- --nocapture

# "/src/wasc/res/spectest_wasc/address/address_0.c" exit code: 0
# "/src/wasc/res/spectest_wasc/address/address_2.c" exit code: 0
# "/src/wasc/res/spectest_wasc/address/address_3.c" exit code: 0
# "/src/wasc/res/spectest_wasc/address/address_4.c" exit code: 0
# ... ...
```

# Getting Started

TODO

# Licences

MIT
