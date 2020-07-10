#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>

#include "common/wavm.h"

#ifndef WAVM_POSIX_X86_64_SPECTEST_H
#define WAVM_POSIX_X86_64_SPECTEST_H

#define WAVM_PAGE_SIZE 0x10000
#ifndef MEMORY0_MAX_PAGE
#define MEMORY0_MAX_PAGE 65536
#endif /* MEMORY0_MAX_PAGE */

#ifdef MEMORY0_DEFINED
extern memory_instance memoryOffset0;
extern uint8_t memory0[];
int32_t wavm_intrinsic_memory_grow(void *dummy, int32_t grow_by)
{
  return -1;
}
#else
int32_t wavm_intrinsic_memory_grow(void *dummy, int32_t grow_by)
{
  return -1;
}
#endif

void callIndirectFail()
{
  exit(1);
}

void unreachableTrap()
{
  exit(1);
}

void divideByZeroOrIntegerOverflowTrap()
{
  exit(1);
}

void invalidFloatOperationTrap()
{
  exit(1);
}

int32_t wavm_spectest_global_i32 = 42;
float wavm_spectest_global_f32 = 42.0;
double wavm_spectest_global_f64 = 420;

uint32_t wavm_spectest_table_length = 10;
uintptr_t wavm_spectest_table[10] = {};

void *wavm_spectest_print_i32(void *dummy, int32_t i)
{
}

void *wavm_exported_function_print32(void *dummy, int32_t i)
{
}

void *wavm_exported_function_print64(void *dummy, int64_t i)
{
}

void *wavm_spectest_print(void *dummy)
{
}

#endif /* WAVM_POSIX_X86_64_SPECTEST_H */
