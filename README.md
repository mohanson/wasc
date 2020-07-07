# WASC

**WASC is a WebAssembly AOT compiler**. The main purpose is to translate the [WASI](https://wasi.dev/) WebAssembly code into machine(x86 and RISC-V) code.

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

You can find more useful examples in the `./example` and `./res/wasi`.

# Credits

- The project mainly inspired by xuejie's [article](https://xuejie.space/2020_03_03_introduction_to_ckb_script_programming_performant_wasm/), and got a lot of help from him.
- `src/platform/wasi.h` derived from [wasi-sysroot/libc-bottom-half/headers/public/wasi/core.h](https://github.com/CraneStation/wasi-sysroot/blob/320054e84f8f2440def3b1c8700cedb8fd697bf8/libc-bottom-half/headers/public/wasi/core.h).
- `src/platform/*_runtime.S` derived from [WAVM/Lib/Platform/POSIX/POSIX-X86_64.S](https://github.com/WAVM/WAVM/blob/master/Lib/Platform/POSIX/POSIX-X86_64.S).
- `src/platform/posix_x86_64_wasi.h`'s section `init_wasi()` derived from [wasc/wasi.c](https://github.com/kanaka/wac/blob/master/wasi.c)
- `src/platform/posix_x86_64_wasi.h`'s section `copy_iov_to_host()` derived from [wasc/wasi.c](https://github.com/kanaka/wac/blob/master/wasi.c)


# Licences

MIT
