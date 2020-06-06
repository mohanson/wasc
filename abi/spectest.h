#include <stdlib.h>

#ifndef WAVM_SPECTEST_ABI_H
#define WAVM_SPECTEST_ABI_H

#ifndef MEMORY0_DEFINED
extern uint8_t *memoryOffset0;
#endif

int32_t wavm_spectest_global_i32 = 42;

void callIndirectFail()
{
    exit(-2);
}

void unreachableTrap()
{
    exit(-2);
}

void divideByZeroOrIntegerOverflowTrap()
{
    exit(-2);
}

void invalidFloatOperationTrap()
{
    exit(-2);
}

#endif /* WAVM_SPECTEST_ABI_H */
