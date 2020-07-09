#include "address_3_glue.h"
#include "platform/posix_x86_64_spectest.h"

int main() {
  init();
  wavm_ret_float wavm_ret1 = wavm_exported_function_32_good1(NULL,0);
  if (*(uint32_t *)&wavm_ret1.value != 0) {
    return 1;
  }
  
  wavm_ret_float wavm_ret2 = wavm_exported_function_32_good2(NULL,0);
  if (*(uint32_t *)&wavm_ret2.value != 0) {
    return 2;
  }
  
  wavm_ret_float wavm_ret3 = wavm_exported_function_32_good3(NULL,0);
  if (*(uint32_t *)&wavm_ret3.value != 0) {
    return 3;
  }
  
  wavm_ret_float wavm_ret4 = wavm_exported_function_32_good4(NULL,0);
  if (*(uint32_t *)&wavm_ret4.value != 0) {
    return 4;
  }
  
  wavm_ret_float wavm_ret5 = wavm_exported_function_32_good5(NULL,0);
  if (*(uint32_t *)&wavm_ret5.value != 2144337921) {
    return 5;
  }
  
  wavm_ret_float wavm_ret6 = wavm_exported_function_32_good1(NULL,65524);
  if (*(uint32_t *)&wavm_ret6.value != 0) {
    return 6;
  }
  
  wavm_ret_float wavm_ret7 = wavm_exported_function_32_good2(NULL,65524);
  if (*(uint32_t *)&wavm_ret7.value != 0) {
    return 7;
  }
  
  wavm_ret_float wavm_ret8 = wavm_exported_function_32_good3(NULL,65524);
  if (*(uint32_t *)&wavm_ret8.value != 0) {
    return 8;
  }
  
  wavm_ret_float wavm_ret9 = wavm_exported_function_32_good4(NULL,65524);
  if (*(uint32_t *)&wavm_ret9.value != 0) {
    return 9;
  }
  
  wavm_ret_float wavm_ret10 = wavm_exported_function_32_good5(NULL,65524);
  if (*(uint32_t *)&wavm_ret10.value != 0) {
    return 10;
  }
  
  wavm_ret_float wavm_ret11 = wavm_exported_function_32_good1(NULL,65525);
  if (*(uint32_t *)&wavm_ret11.value != 0) {
    return 11;
  }
  
  wavm_ret_float wavm_ret12 = wavm_exported_function_32_good2(NULL,65525);
  if (*(uint32_t *)&wavm_ret12.value != 0) {
    return 12;
  }
  
  wavm_ret_float wavm_ret13 = wavm_exported_function_32_good3(NULL,65525);
  if (*(uint32_t *)&wavm_ret13.value != 0) {
    return 13;
  }
  
  wavm_ret_float wavm_ret14 = wavm_exported_function_32_good4(NULL,65525);
  if (*(uint32_t *)&wavm_ret14.value != 0) {
    return 14;
  }
  
}
