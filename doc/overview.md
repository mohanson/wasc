# WASC: 一个高效的 WebAssembly 到 RISC-V AOT 编译器

在区块链世界的链上虚拟机中, 流行着几种不同的指令集, 包括 WebAssembly, RISC-V, 与即将完成历史使命退休的 EVM. 它们运行在大致相同的抽象层级上. 在以太坊社区计划让 EVM 退休前, 他们正在编写一个 EVM 到 WebAssembly 的源到源编译器(Source to source compiler), 这使得这两个指令集可以互相复用对方的很大一部分工具(主要是后者对前者). 同时我注意到, 目前世界上缺少好用的从 WebAssembly 到 RISC-V 编译器的相关工具的研究, 因此我决定填补这一工作.

在过去的几个月时间里, 我投入了大量时间在 [WASC: https://github.com/mohanson/wasc](https://github.com/mohanson/wasc) 项目上. 它已经拥有不错的完成度, 并且很高兴的可以在此时向大家分享这一项目.

WASC 可以将 WebAssembly 规范的 WebAssembly 字节码(.wasm)或 S 表达式(.wat)文件编译为本机可执行文件. 另外, 不止于此, 它同样支持部分平台的交叉编译, 例如, 您可以在 posix x86_64 平台下编译可以直接在 RISC-V 平台上运行的可执行文件.

WASC 底层依赖一个名为 [WAVM](https://github.com/WAVM/WAVM) 的项目. 它是一种高性能的转换层, 基准测试表明这是迄今为止 WebAssembly 的最高性能的转换层实现, 它可通过 LLVM 将 WebAssembly 代码直接编译为本机代码. 但是 WAVM 正如大多数 WebAssembly 的 JIT 编译器一样, 它需要一个运行时部分, 用以桥接主机环境与虚拟机环境. 这个运行时部分大小超过 2M, 这与我们所期望的在空间受限的区块链上执行 WebAssembly 的初衷相违背.

WASC 的核心设计原理, 在于尝试将构建运行时所需的所有信息都包含在一个文件中. 它在一个单独的 C 语言文件中发出最少的运行时部分代码(在多数情况下, 仅仅只需要使用不到 50 行 C 代码), 然后通过链接器将之与 WAVM 生成的目标文件链接在一起, 最终生成一个可执行的 ELF 格式的二进制文件.

WASC 目前是在 Linux 上开发的, 但是很有可能也应该可以在 Mac OS 和 Windows 上工作. 虽然我没有做过这方面的尝试, 但经验告诉我这应该是可行的.

WASC 的主要目的旨在为使用 RISC-V 作为指令集的区块链虚拟机提供一个可以支持更多合约编程语言的平台, 使用 WASC + RISC-V 可获得多种好处，下面列出了其中的一些:

- 为合约开发者提供了可选性, 可以使用许多传统静态编程语言(例如 C, C++ 和 Rust), 也可以使用一些对用户更加友好的语言(例如 AssemblyScript)
- 访问 WebAssembly 庞大的社区和周围工具链

# 示例: echo 程序的编译与执行

一个简单的例子, 通常是对一个复杂项目原理的最好解释. 首先我们将需要安装 WASC, 由于这一步骤需要安装完整的 WebAssembly 与 RISC-V 开发环境, 你必须预留大量的人生时间和空闲的磁盘. 就在我的旧机器上, 这个数字是 27 分钟和 11 GB. 但是不必担心, 安装过程中所有生成的文件都被保存在当前目录, 因此对您的操作系统没有任何可见的副作用.

> 为了方便起见, 下文默认您的工作目录在 `/src` 路径上. 如果不是, 那么需要您自己手动替换示例中有关的路径. 示例在 Ubuntu 18.04 中演示.

```sh
$ cd /src
$ git clone https://github.com/mohanson/wasc

$ cd wasc
$ ./build.sh
```

在这之前, 确保您已经安装了 LLVM-9, 因为 RISC-V 直到 LLVM-9 才被正式支持. 如果没有, 使用以下命令来安装它.

```sh
$ apt install llvm-9
```

希望您的安装一切顺利! 不过您必须了解, WebAssembly 与 RISC-V 工具包都是超级大的项目, 稀奇古怪的问题均可能出现, 这时候去对应的社区寻求和搜索引擎上寻求帮助是个很好的主意.

WASC 项目预先构建了一个 example 程序, 它位于 `./example/echo.wasm`. 该程序最初由 C 编写而成, 然后使用 wasi-sdk 工具包编译到 WebAssembly 代码. 大体上来说, 它和我们在 Linux 下常用的 `echo` 命令差不多, 其源代码为:

```c
#include <stdio.h>

int main(int argc, char** argv)
{
  for(int argIndex = 0; argIndex <= argc; ++argIndex)
  {
    printf("%s", argv[argIndex]);
  }
  return 0;
}
```

使用 wasc 编译 `echo.wasm` 文件, 这将得到一个本机可执行的二进制文件(posix x86_64):

```sh
$ ./build/wasc --save ./example/echo.wasm
```

提示, `--save` 命令将保存编译过程中生成的临时文件, 这有助于我们在下文对 WASC 的原理进行讲解. 来尝试下由 WebAssembly 编译而来的成果, 我们调用 echo 可执行文件, 并尝试让它输出 "Hello World!".

```sh
$ ./example/echo Hello World!
# Hello World!
```

# 工作原理与步骤分析

让我们先将目光聚焦在 `./example/echo_build` 文件夹下, 它是编译过程中生成的临时文件. 只有接收 `--save` 参数它们才会被保留下来. 它的目录结构如下所示:

```no-highlight
.
├── echo                                <--- 可执行二进制文件
├── echo.c                              <--- 入口文件
├── echo_glue.h                         <--- 胶水代码, 提供极小的运行时
├── echo.o                              <--- WAVM 生成的目标文件
├── echo_precompiled.wasm               <--- WAVM 生成的预编译文件, 内含完整目标代码
└── platform
    ├── common
    │   ├── wasi.h                      <--- WASI 数据结构定义
    │   └── wavm.h                      <--- WAVM 数据结构定义
    ├── posix_x86_64_wasi.h             <--- WASI 在 posix 上的部分实现(C 代码)
    └── posix_x86_64_wasi_runtime.S     <--- WASI 在 posix 上的部分实现(汇编代码)
```

WASC 编译的第一步是利用 WAVM 生成目标文件, 文件名 `echo.o`. 它是 WAVM 在 JIT 编译阶段的实际输出, 主要是机器代码, 同时包含允许链接程序查看其中包含哪些符号以及其正常工作所需的符号的信息. 使用 objdump 工具来分析这个文件, 并截取其中 SYMBOL TABLE 的一部分如下:

```sh
$ objdump -x echo.o
```

```no-highlight
SYMBOL TABLE:
0000000000000000         *UND*  0000000000000000 biasedInstanceId
0000000000000000         *UND*  0000000000000000 functionDefMutableDatas0
0000000000000000         *UND*  0000000000000000 functionDefMutableDatas1
0000000000000000         *UND*  0000000000000000 functionDefMutableDatas2
0000000000000000         *UND*  0000000000000000 functionImport0
0000000000000000         *UND*  0000000000000000 functionImport1
0000000000000000         *UND*  0000000000000000 functionImport2
0000000000000000         *UND*  0000000000000000 functionImport3
0000000000000000         *UND*  0000000000000000 global5
0000000000000000         *UND*  0000000000000000 memoryOffset0
0000000000000000         *UND*  0000000000000000 typeId3
0000000000000000         *UND*  0000000000000000 typeId4
```

不错, 我们大概已经知道 WAVM 的运行时将要给它生成的 JIT 代码提供什么数据了. WASC 在这里做了一件事, 它仅仅使用极少量的代码(远远少于 WAVM), 为目标文件提供它缺少的东西. 这部分代码被称为 WASC 的胶水代码, 大部分由 C 编写而成. echo 程序的胶水代码如下所示.

```sh
$ cat echo_glue.h
```

```c
#include<stddef.h>
#include<stdint.h>
#include<stdlib.h>
#include<string.h>

#include "platform/common/wavm.h"

#ifndef ECHO_GLUE_H
#define ECHO_GLUE_H

const uint64_t functionDefMutableData = 0;
const uint64_t biasedInstanceId = 0;
const uint64_t tableReferenceBias = 0;

const uint64_t typeId0 = 0;
const uint64_t typeId1 = 0;
const uint64_t typeId2 = 0;
const uint64_t typeId3 = 0;
const uint64_t typeId4 = 0;
const int32_t global0 = 0;
const int32_t global1 = 1;
const int32_t global2 = 4;
const int32_t global3 = 8;
const int32_t global4 = 12;
int32_t global5 = 128;
#define wavm_wasi_args_get functionImport0
extern wavm_ret_int32_t (functionImport0) (void*, int32_t, int32_t);
#define wavm_wasi_args_sizes_get functionImport1
extern wavm_ret_int32_t (functionImport1) (void*, int32_t, int32_t);
#define wavm_wasi_fd_write functionImport2
extern wavm_ret_int32_t (functionImport2) (void*, int32_t, int32_t, int32_t, int32_t);
#define wavm_wasi_proc_exit functionImport3
extern void* (functionImport3) (void*, int32_t);
extern wavm_ret_int32_t (functionDef0) (void*, int32_t);
const uint64_t functionDefMutableDatas0 = 0;
extern wavm_ret_int32_t (functionDef1) (void*, int32_t);
const uint64_t functionDefMutableDatas1 = 0;
extern void* (functionDef2) (void*);
const uint64_t functionDefMutableDatas2 = 0;
uint32_t memory0_length = 1;
uint8_t* memory0;
struct memory_instance memoryOffset0;
uint8_t memory0_data0[1] = {
  0x20
};
uint8_t memory0_data1[1] = {
  0x0a
};
#define MEMORY0_DEFINED 1
void init_memory0() {
  memory0 = calloc(65536, 1);
  memcpy(memory0 + 0, memory0_data0, 1);
  memcpy(memory0 + 1, memory0_data1, 1);
  memoryOffset0.base = memory0;
  memoryOffset0.num_pages = 1;
}
#define wavm_exported_function__start functionDef2
void init() {
  init_memory0();
}
int32_t g_argc;
char **g_argv;
int main(int argc, char *argv[]) {
  g_argc = argc;
  g_argv = argv;
  init_wasi();
  init();
  wavm_exported_function__start(NULL);
  return 0;
}
#endif /* ECHO_GLUE_H */
```

你可以看到, 许多数据, 在胶水代码中只是给他简单的赋值为 0, 比如 `const uint64_t typeId0 = 0;`. 这是因为在 WAVM 的运行时环境中, 某些类型的数据只有它们的地址是有用的. 对于 WAVM 而言, typeId0 是一个结构体, 它保存了一个具体的函数签名, 但在运行时中判断两个函数的函数签名是否一致时, WAVM 只比对了两个函数的签名的地址是否是同一个地址. 因此, 我们不必真的在胶水代码中实现一个复杂的函数签名结构体, 这给了我们巨大的优化空间. WASC 在这里做了大量的 trick 工作, 来保证胶水代码提供的运行时是极小的.

在生成胶水代码后, WASC 将使用 gcc 来进行胶水代码和 WAVM 目标文件的编译和链接, 并最终编译并得到本机平台的二进制文件.

WASC 的大体流程就是如此, 但是在实际实现的细节中会比上面介绍的复杂得多, 比如我们不得不采用一些汇编代码和 Linker Script 来完成一部分难以实现的功能. 如果有机会的话, 我想在一篇专门的文章中进行介绍.

# AssemblyScript, Syscall, RISC-V 与 CKB-VM

接下来我想谈谈 WASC 在 RISC-V 平台上的应用. RISC-V 指令集中有一个特殊的指令: ECALL. ECALL 指令用于向执行环境(通常是操作系统)发出请求, 系统 ABI 将定义如何传递请求的参数. 为了使得 WebAssembly 可以与 RISC-V 宿主环境进行交互, 在 WebAssembly 程序中实现 ECALL 是十分有必要的. 下面我们将从 AssemblyScript 编程语言开始出发, 来对此进行简要的描述.

我已经事先编写好了代码, 您可以先将下面这两个项目 clone 到本地.

```sh
$ git clone https://github.com/libraries/wasc_dapp_demo_ckb_vm
$ git clone https://github.com/libraries/wasc_dapp_demo_assemblyscript
```

文件 `./wasc_dapp_demo_ckb_vm/example/main.c`, 它是一个简单的 RISC-V 应用程序, 在程序中调用了一次 syscall, 然后将一个字符串的地址和长度作为参数之一传向 RISC-V 宿主平台.

```c
#include <string.h>

static inline long __internal_syscall(long n, long _a0, long _a1, long _a2,
                                      long _a3, long _a4, long _a5) {
  register long a0 asm("a0") = _a0;
  register long a1 asm("a1") = _a1;
  register long a2 asm("a2") = _a2;
  register long a3 asm("a3") = _a3;
  register long a4 asm("a4") = _a4;
  register long a5 asm("a5") = _a5;
  register long syscall_id asm("a7") = n;
  asm volatile("scall"
               : "+r"(a0)
               : "r"(a1), "r"(a2), "r"(a3), "r"(a4), "r"(a5), "r"(syscall_id));
  return a0;
}

#define syscall(n, a, b, c, d, e, f)                                           \
  __internal_syscall(n, (long)(a), (long)(b), (long)(c), (long)(d), (long)(e), \
                     (long)(f))

#define SYSCODE_DEBUG 2000

int main() {
    char *s = "Hello World!";
    return syscall(SYSCODE_DEBUG, &s[0], strlen(s), 0, 0, 0, 0);
}
```

宿主平台将接收到这个 syscall 请求, 然后该字符串会被打印到标准输出. 这部分逻辑实现是在 `wasc_dapp_demo_ckb_vm` 项目里, 它是一个 RISC-V 虚拟机实现, 底层使用 ckb-vm, 并注册了处理指定系统调用的函数. 我们准备执行这个 main 程序, 直接输入以下命令:

```sh
$ cargo run -- example/main
```

您将看到:

```no-highlight
debug: Hello World!
exit=0 cycles=771
```

故事并没有结束, 相反, 精彩的内容才刚刚开始! 我们将使用 AssemblyScripy 来重写这个程序, 并演示如何在 AssemblyScript 代码中实现 syscall.

打开 `./wasc_dapp_demo_assemblyscript/assembly/index.ts` 文件, 它由 AssemblyScript 编写, 逻辑和之前 C 写的代码差不多, 其源码为:

```ts
import {
  syscall
} from './env'

export function _start(): i32 {
  let str = "Hello World!"
  let strEncoded = String.UTF8.encode(str, true)
  syscall(2000, changetype<usize>(strEncoded), strEncoded.byteLength, 0, 0, 0, 0, 0b100000)
  return 0
}
```

其中的 syscall 函数定义同目录的 `env.ts` 文件中:

```ts
export declare function syscall(n: i64, a: i64, b: i64, c: i64, d: i64, e: i64, f: i64, mode: i64): i64
```

它是一个 `export declare` 函数, 表明它自己并没有实现这个函数, 需要有运行时环境为它提供. 从底层语义上来讲, 它是一个 WebAssembly 的 Import. 在 WASC 的胶水代码部分, 会为其提供实现:

```c
wavm_ret_int64_t wavm_env_syscall(void *dummy, int64_t n, int64_t _a0, int64_t _a1, int64_t _a2, int64_t _a3, int64_t _a4, int64_t _a5, int64_t mode)
{
    wavm_ret_int64_t ret;
    ret.dummy = dummy;
    if (mode & 0b100000)
    {
        _a0 = (int64_t)&memoryOffset0.base[0] + _a0;
    }
    if (mode & 0b010000)
    {
        _a1 = (int64_t)&memoryOffset0.base[0] + _a1;
    }
    if (mode & 0b001000)
    {
        _a2 = (int64_t)&memoryOffset0.base[0] + _a2;
    }
    if (mode & 0b000100)
    {
        _a3 = (int64_t)&memoryOffset0.base[0] + _a3;
    }
    if (mode & 0b000010)
    {
        _a4 = (int64_t)&memoryOffset0.base[0] + _a4;
    }
    if (mode & 0b000001)
    {
        _a5 = (int64_t)&memoryOffset0.base[0] + _a5;
    }
    ret.value = syscall(n, _a0, _a1, _a2, _a3, _a4, _a5);
    return ret;
}
```

要额外关注的是 syscall 的最后一个参数 mode, 它目前被传入了值 0b100000. 目前 syscall 的第一个参数是 `changetype<usize>(strEncoded)`, 它是一个字符串的指针(在语法层面), 但存在问题, 因为这个字符串被定义在 WebAssembly 的内存中而非程序的运行内存中. mode 参数存在的意义便是告知我们的胶水代码, 这个传入的地址并非真实地址, 而是 WebAssembly 内存的偏移地址. 胶水代码会通过该偏移地址和 WebAssembly 内存首地址来获取真实的地址.

最后, 编译这个项目, 它首先通过 AssemblyScript 的工具集生成 WebAssembly 输出, 然后经过 WASC 获得 RISC-V 的输出. 最后在 wasc_dapp_demo_ckb_vm 中执行它, 你的标准输出将同样获得相似的结果.

```sh
$ cd /src/wasc_dapp_demo_assemblyscript
$ npm run asbuild

$ cd build
$ /src/wasc/build/wasc -v --platform ckb_vm_assemblyscript --gcc /src/wasc/third_party/ckb-riscv-gnu-toolchain/build/bin/riscv64-unknown-elf-gcc optimized.wat

$ cd /src/wasc_dapp_demo_ckb_vm
$ cargo run -- /src/wasc_dapp_demo_assemblyscript/build/optimized
```

```no-highlight
debug: Hello World!
exit=0 cycles=14418
```

# 并非完美

在实现 WASC 的过程中, 我遇到过许多问题, 有些已经解决了, 但有些还未解决. 我会列举出一些困扰我较长时间的问题, 以及我对这些问题的思考.

- WASC 未对 WebAssembly 内存进行越界访问检查. 我可以加入这部分检查, 代价是膨胀的运行时, 但这真的有必要吗? 我们知道 C 语言也不会限制越界访问, 因此越界检查不是必须存在的某个功能. 对此我引入一个安全假设: 合格的开发者开发出的合格的程序不应当存在越界访问的 BUG, 如果真的干了这种蠢事, 他们也应该有能力对此进行调试.
- AssemblyScript 缺少像 C, C++ 和 Rust 那样广泛的各种加密算法的支持, 而这些算法对区块链很重要. 事实上这是别人问我的一个问题, 我思考后认为这可以被解决, 而且方式不只一种. C 能被编译到 WebAssembly, AssemblyScript 也可以, 那这中间就有无限的想象力空间了.

最后, 我想吐槽一下 WebAssembly 的测试集, 它们是如此的盘根错节, 混乱不堪, 甚至在我实现 WASI 接口的时候, 世界上还根本不存在这部分的官方测试集(只能找到零散的第三方编写的测试). 不过好在在我反映这个问题后, 官方已经在计划对此进行优化了.

感谢您的阅读.
