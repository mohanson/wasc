#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>

#include "common/wavm.h"

#ifndef WAVM_CKB_VM_ASSEMBLYSCRIPT_H
#define WAVM_CKB_VM_ASSEMBLYSCRIPT_H

#define WAVM_PAGE_SIZE 0x10000
#ifndef MEMORY0_MAX_PAGE
#define MEMORY0_MAX_PAGE 65536
#endif /* MEMORY0_MAX_PAGE */

#ifdef MEMORY0_DEFINED
extern memory_instance memoryOffset0;
extern uint8_t memory0[];
extern uint32_t memory0_length;
int32_t wavm_intrinsic_memory_grow(void *dummy, int32_t grow_by)
{
    if ((memoryOffset0.num_pages + grow_by) > MEMORY0_MAX_PAGE)
    {
        return -1;
    }
    int32_t old_pages = memoryOffset0.num_pages;
    memory0_length += grow_by * WAVM_PAGE_SIZE;
    memoryOffset0.num_pages += grow_by;
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

uint64_t __atomic_load_8(void *p, int32_t _mode)
{
    (void)_mode;
    return *((uint64_t *)((uintptr_t)p));
}

void* wavm_env_abort(void *dummy, int32_t a, int32_t b, int32_t c, int32_t d)
{
    exit(1);
}

#endif /* WAVM_CKB_VM_ASSEMBLYSCRIPT_H */
