#include<stddef.h>
#include<stdint.h>
#include<stdlib.h>
#include<string.h>

#include "platform/common/wavm.h"

#ifndef ADDRESS_0_GLUE_H
#define ADDRESS_0_GLUE_H

const uint64_t functionDefMutableData = 0;
const uint64_t biasedInstanceId = 0;
const uint64_t tableReferenceBias = 0;

const uint64_t typeId0 = 0;
const uint64_t typeId1 = 0;
extern wavm_ret_int32_t (functionDef0) (void*, int32_t);
const uint64_t functionDefMutableDatas0 = 0;
extern wavm_ret_int32_t (functionDef1) (void*, int32_t);
const uint64_t functionDefMutableDatas1 = 0;
extern wavm_ret_int32_t (functionDef2) (void*, int32_t);
const uint64_t functionDefMutableDatas2 = 0;
extern wavm_ret_int32_t (functionDef3) (void*, int32_t);
const uint64_t functionDefMutableDatas3 = 0;
extern wavm_ret_int32_t (functionDef4) (void*, int32_t);
const uint64_t functionDefMutableDatas4 = 0;
extern wavm_ret_int32_t (functionDef5) (void*, int32_t);
const uint64_t functionDefMutableDatas5 = 0;
extern wavm_ret_int32_t (functionDef6) (void*, int32_t);
const uint64_t functionDefMutableDatas6 = 0;
extern wavm_ret_int32_t (functionDef7) (void*, int32_t);
const uint64_t functionDefMutableDatas7 = 0;
extern wavm_ret_int32_t (functionDef8) (void*, int32_t);
const uint64_t functionDefMutableDatas8 = 0;
extern wavm_ret_int32_t (functionDef9) (void*, int32_t);
const uint64_t functionDefMutableDatas9 = 0;
extern wavm_ret_int32_t (functionDef10) (void*, int32_t);
const uint64_t functionDefMutableDatas10 = 0;
extern wavm_ret_int32_t (functionDef11) (void*, int32_t);
const uint64_t functionDefMutableDatas11 = 0;
extern wavm_ret_int32_t (functionDef12) (void*, int32_t);
const uint64_t functionDefMutableDatas12 = 0;
extern wavm_ret_int32_t (functionDef13) (void*, int32_t);
const uint64_t functionDefMutableDatas13 = 0;
extern wavm_ret_int32_t (functionDef14) (void*, int32_t);
const uint64_t functionDefMutableDatas14 = 0;
extern wavm_ret_int32_t (functionDef15) (void*, int32_t);
const uint64_t functionDefMutableDatas15 = 0;
extern wavm_ret_int32_t (functionDef16) (void*, int32_t);
const uint64_t functionDefMutableDatas16 = 0;
extern wavm_ret_int32_t (functionDef17) (void*, int32_t);
const uint64_t functionDefMutableDatas17 = 0;
extern wavm_ret_int32_t (functionDef18) (void*, int32_t);
const uint64_t functionDefMutableDatas18 = 0;
extern wavm_ret_int32_t (functionDef19) (void*, int32_t);
const uint64_t functionDefMutableDatas19 = 0;
extern wavm_ret_int32_t (functionDef20) (void*, int32_t);
const uint64_t functionDefMutableDatas20 = 0;
extern wavm_ret_int32_t (functionDef21) (void*, int32_t);
const uint64_t functionDefMutableDatas21 = 0;
extern wavm_ret_int32_t (functionDef22) (void*, int32_t);
const uint64_t functionDefMutableDatas22 = 0;
extern wavm_ret_int32_t (functionDef23) (void*, int32_t);
const uint64_t functionDefMutableDatas23 = 0;
extern wavm_ret_int32_t (functionDef24) (void*, int32_t);
const uint64_t functionDefMutableDatas24 = 0;
extern void* (functionDef25) (void*, int32_t);
const uint64_t functionDefMutableDatas25 = 0;
extern void* (functionDef26) (void*, int32_t);
const uint64_t functionDefMutableDatas26 = 0;
extern void* (functionDef27) (void*, int32_t);
const uint64_t functionDefMutableDatas27 = 0;
extern void* (functionDef28) (void*, int32_t);
const uint64_t functionDefMutableDatas28 = 0;
extern void* (functionDef29) (void*, int32_t);
const uint64_t functionDefMutableDatas29 = 0;
uint8_t* memory0;
struct memory_instance memoryOffset0;
uint8_t memory0_data0[26] = {
  0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 
  0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a
};
#define MEMORY0_DEFINED 1
void init_memory0() {
  memory0 = calloc(65536, 1);
  memcpy(memory0 + 0, memory0_data0, 26);
  memoryOffset0.base = memory0;
  memoryOffset0.num_pages = 1;
}
#define wavm_exported_function_8u_good1 functionDef0
#define wavm_exported_function_8u_good2 functionDef1
#define wavm_exported_function_8u_good3 functionDef2
#define wavm_exported_function_8u_good4 functionDef3
#define wavm_exported_function_8u_good5 functionDef4
#define wavm_exported_function_8s_good1 functionDef5
#define wavm_exported_function_8s_good2 functionDef6
#define wavm_exported_function_8s_good3 functionDef7
#define wavm_exported_function_8s_good4 functionDef8
#define wavm_exported_function_8s_good5 functionDef9
#define wavm_exported_function_16u_good1 functionDef10
#define wavm_exported_function_16u_good2 functionDef11
#define wavm_exported_function_16u_good3 functionDef12
#define wavm_exported_function_16u_good4 functionDef13
#define wavm_exported_function_16u_good5 functionDef14
#define wavm_exported_function_16s_good1 functionDef15
#define wavm_exported_function_16s_good2 functionDef16
#define wavm_exported_function_16s_good3 functionDef17
#define wavm_exported_function_16s_good4 functionDef18
#define wavm_exported_function_16s_good5 functionDef19
#define wavm_exported_function_32_good1 functionDef20
#define wavm_exported_function_32_good2 functionDef21
#define wavm_exported_function_32_good3 functionDef22
#define wavm_exported_function_32_good4 functionDef23
#define wavm_exported_function_32_good5 functionDef24
#define wavm_exported_function_8u_bad functionDef25
#define wavm_exported_function_8s_bad functionDef26
#define wavm_exported_function_16u_bad functionDef27
#define wavm_exported_function_16s_bad functionDef28
#define wavm_exported_function_32_bad functionDef29
void init() {
  init_memory0();
}
#endif /* ADDRESS_0_GLUE_H */
