pub const TEXT_HEADER_HEAD_TEMPLATE: &str = "\
#include <stddef.h>
#include <stdint.h>

#ifndef ${header_id}
#define ${header_id}

typedef struct {
  void* dummy;
  int32_t value;
} wavm_ret_int32_t;

typedef struct {
  void* dummy;
  int64_t value;
} wavm_ret_int64_t;

typedef struct {
  void* dummy;
  float value;
} wavm_ret_float;

typedef struct {
  void* dummy;
  double value;
} wavm_ret_double;

const uint64_t functionDefMutableData = 0;
const uint64_t biasedInstanceId = 0;
";
