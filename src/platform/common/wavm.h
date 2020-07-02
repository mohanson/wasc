#include <stdint.h>

#ifndef WAVM_H
#define WAVM_H

typedef struct
{
    void *dummy;
    int32_t value;
} wavm_ret_int32_t;

typedef struct
{
    void *dummy;
    int64_t value;
} wavm_ret_int64_t;

typedef struct
{
    void *dummy;
    float value;
} wavm_ret_float;

typedef struct
{
    void *dummy;
    double value;
} wavm_ret_double;

typedef struct memory_instance
{
    uint8_t *base;
    uint64_t num_pages;
} memory_instance;

#endif /* WAVM_H */
