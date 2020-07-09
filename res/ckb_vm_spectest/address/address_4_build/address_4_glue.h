#include<stddef.h>
#include<stdint.h>
#include<stdlib.h>
#include<string.h>

#include "platform/common/wavm.h"

#ifndef ADDRESS_4_GLUE_H
#define ADDRESS_4_GLUE_H

const uint64_t functionDefMutableData = 0;
const uint64_t biasedInstanceId = 0;
const uint64_t tableReferenceBias = 0;

const uint64_t typeId0 = 0;
const uint64_t typeId1 = 0;
extern wavm_ret_double (functionDef0) (void*, int32_t);
const uint64_t functionDefMutableDatas0 = 0;
extern wavm_ret_double (functionDef1) (void*, int32_t);
const uint64_t functionDefMutableDatas1 = 0;
extern wavm_ret_double (functionDef2) (void*, int32_t);
const uint64_t functionDefMutableDatas2 = 0;
extern wavm_ret_double (functionDef3) (void*, int32_t);
const uint64_t functionDefMutableDatas3 = 0;
extern wavm_ret_double (functionDef4) (void*, int32_t);
const uint64_t functionDefMutableDatas4 = 0;
extern void* (functionDef5) (void*, int32_t);
const uint64_t functionDefMutableDatas5 = 0;
uint8_t* memory0;
struct memory_instance memoryOffset0;
uint8_t memory0_data0[26] = {
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
  0xf4, 0x7f, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfc, 0x7f
};
#define MEMORY0_DEFINED 1
void init_memory0() {
  memory0 = calloc(65536, 1);
  memcpy(memory0 + 0, memory0_data0, 26);
  memoryOffset0.base = memory0;
  memoryOffset0.num_pages = 1;
}
#define wavm_exported_function_64_good1 functionDef0
#define wavm_exported_function_64_good2 functionDef1
#define wavm_exported_function_64_good3 functionDef2
#define wavm_exported_function_64_good4 functionDef3
#define wavm_exported_function_64_good5 functionDef4
#define wavm_exported_function_64_bad functionDef5
void init() {
  init_memory0();
}
#endif /* ADDRESS_4_GLUE_H */
