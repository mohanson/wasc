#include <stdio.h>
#include <stdlib.h>

#ifndef WAVM_SPECTEST_ABI_H
#define WAVM_SPECTEST_ABI_H

#ifndef MEMORY0_MAX_PAGE
#define MEMORY0_MAX_PAGE 65536
#endif /* MEMORY0_MAX_PAGE */
#define WAVM_PAGE_SIZE 0x10000

#ifdef MEMORY0_DEFINED
int32_t wavm_intrinsic_memory_grow(void *dummy, int32_t grow_by)
{
  if (grow_by == 0)
  {
    return memoryOffset0.num_pages;
  }
  int32_t old_pages = memoryOffset0.num_pages;
  int32_t new_pages = old_pages + grow_by;
  if (new_pages > MEMORY0_MAX_PAGE)
  {
    return -1;
  }
  size_t old_size = old_pages * WAVM_PAGE_SIZE;
  size_t new_size = new_pages * WAVM_PAGE_SIZE;
  memory0 = realloc(memory0, new_size);
  memset(memory0 + old_size, 0, new_size - old_size);
  memoryOffset0.base = memory0;
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

#endif /* WAVM_SPECTEST_ABI_H */
