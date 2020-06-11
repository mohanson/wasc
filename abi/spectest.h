#include <stdio.h>
#include <stdlib.h>

#ifndef WAVM_SPECTEST_ABI_H
#define WAVM_SPECTEST_ABI_H

#ifndef WAVM_MAX_PAGE
#define WAVM_MAX_PAGE 65536
#endif /* WAVM_MAX_PAGE */
#define WAVM_PAGE_SIZE 0x10000

#ifdef MEMORY0_DEFINED
int32_t wavm_intrinsic_memory_grow(void *dummy, int32_t grow_by)
{
  int32_t old_pages = memoryOffset0.num_pages;
  int32_t new_pages = old_pages + grow_by;
  if (new_pages > WAVM_MAX_PAGE)
  {
    return -1;
  }
  uint8_t *old_memory0 = memoryOffset0.base;
  size_t old_size = old_pages * WAVM_PAGE_SIZE * sizeof(uint8_t);
  size_t new_size = new_pages * WAVM_PAGE_SIZE * sizeof(uint8_t);
  uint8_t *new_memory0 = malloc(new_size);
  memcpy(new_memory0, old_memory0, old_size);
  memoryOffset0.base = new_memory0;
  memoryOffset0.num_pages = new_pages;
  return old_pages;
}
#else
int32_t wavm_intrinsic_memory_grow(void *dummy, int32_t grow_by)
{
  return -1;
}
#endif

void callIndirectFail()
{
  exit(255);
}

void unreachableTrap()
{
  exit(254);
}

void divideByZeroOrIntegerOverflowTrap()
{
  exit(253);
}

void invalidFloatOperationTrap()
{
  exit(252);
}

int32_t wavm_spectest_global_i32 = 42;

void *wavm_spectest_print_i32(void *dummy, int32_t i)
{
  printf("wavm_spectest_print_i32 %d\n", i);
}

#endif /* WAVM_SPECTEST_ABI_H */
