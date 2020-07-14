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

void *wavm_env_abort(void *dummy, int32_t a, int32_t b, int32_t c, int32_t d)
{
    exit(1);
}

static inline long __internal_syscall(long n, long _a0, long _a1, long _a2,
                                      long _a3, long _a4, long _a5)
{
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

#define syscall(n, a, b, c, d, e, f)                                             \
    __internal_syscall(n, (long)(a), (long)(b), (long)(c), (long)(d), (long)(e), \
                       (long)(f))

#ifdef MEMORY0_DEFINED
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
#endif /* MEMORY0_DEFINED */

#endif /* WAVM_CKB_VM_ASSEMBLYSCRIPT_H */
