#include "address_0_glue.h"
#include "platform/posix_x86_64_spectest.h"

int main() {
  init();
  wavm_ret_int32_t wavm_ret1 = wavm_exported_function_8u_good1(NULL,0);
  if (*(uint32_t *)&wavm_ret1.value != 97) {
    return 1;
  }
  
  wavm_ret_int32_t wavm_ret2 = wavm_exported_function_8u_good2(NULL,0);
  if (*(uint32_t *)&wavm_ret2.value != 97) {
    return 2;
  }
  
  wavm_ret_int32_t wavm_ret3 = wavm_exported_function_8u_good3(NULL,0);
  if (*(uint32_t *)&wavm_ret3.value != 98) {
    return 3;
  }
  
  wavm_ret_int32_t wavm_ret4 = wavm_exported_function_8u_good4(NULL,0);
  if (*(uint32_t *)&wavm_ret4.value != 99) {
    return 4;
  }
  
  wavm_ret_int32_t wavm_ret5 = wavm_exported_function_8u_good5(NULL,0);
  if (*(uint32_t *)&wavm_ret5.value != 122) {
    return 5;
  }
  
  wavm_ret_int32_t wavm_ret6 = wavm_exported_function_8s_good1(NULL,0);
  if (*(uint32_t *)&wavm_ret6.value != 97) {
    return 6;
  }
  
  wavm_ret_int32_t wavm_ret7 = wavm_exported_function_8s_good2(NULL,0);
  if (*(uint32_t *)&wavm_ret7.value != 97) {
    return 7;
  }
  
  wavm_ret_int32_t wavm_ret8 = wavm_exported_function_8s_good3(NULL,0);
  if (*(uint32_t *)&wavm_ret8.value != 98) {
    return 8;
  }
  
  wavm_ret_int32_t wavm_ret9 = wavm_exported_function_8s_good4(NULL,0);
  if (*(uint32_t *)&wavm_ret9.value != 99) {
    return 9;
  }
  
  wavm_ret_int32_t wavm_ret10 = wavm_exported_function_8s_good5(NULL,0);
  if (*(uint32_t *)&wavm_ret10.value != 122) {
    return 10;
  }
  
  wavm_ret_int32_t wavm_ret11 = wavm_exported_function_16u_good1(NULL,0);
  if (*(uint32_t *)&wavm_ret11.value != 25185) {
    return 11;
  }
  
  wavm_ret_int32_t wavm_ret12 = wavm_exported_function_16u_good2(NULL,0);
  if (*(uint32_t *)&wavm_ret12.value != 25185) {
    return 12;
  }
  
  wavm_ret_int32_t wavm_ret13 = wavm_exported_function_16u_good3(NULL,0);
  if (*(uint32_t *)&wavm_ret13.value != 25442) {
    return 13;
  }
  
  wavm_ret_int32_t wavm_ret14 = wavm_exported_function_16u_good4(NULL,0);
  if (*(uint32_t *)&wavm_ret14.value != 25699) {
    return 14;
  }
  
  wavm_ret_int32_t wavm_ret15 = wavm_exported_function_16u_good5(NULL,0);
  if (*(uint32_t *)&wavm_ret15.value != 122) {
    return 15;
  }
  
  wavm_ret_int32_t wavm_ret16 = wavm_exported_function_16s_good1(NULL,0);
  if (*(uint32_t *)&wavm_ret16.value != 25185) {
    return 16;
  }
  
  wavm_ret_int32_t wavm_ret17 = wavm_exported_function_16s_good2(NULL,0);
  if (*(uint32_t *)&wavm_ret17.value != 25185) {
    return 17;
  }
  
  wavm_ret_int32_t wavm_ret18 = wavm_exported_function_16s_good3(NULL,0);
  if (*(uint32_t *)&wavm_ret18.value != 25442) {
    return 18;
  }
  
  wavm_ret_int32_t wavm_ret19 = wavm_exported_function_16s_good4(NULL,0);
  if (*(uint32_t *)&wavm_ret19.value != 25699) {
    return 19;
  }
  
  wavm_ret_int32_t wavm_ret20 = wavm_exported_function_16s_good5(NULL,0);
  if (*(uint32_t *)&wavm_ret20.value != 122) {
    return 20;
  }
  
  wavm_ret_int32_t wavm_ret21 = wavm_exported_function_32_good1(NULL,0);
  if (*(uint32_t *)&wavm_ret21.value != 1684234849) {
    return 21;
  }
  
  wavm_ret_int32_t wavm_ret22 = wavm_exported_function_32_good2(NULL,0);
  if (*(uint32_t *)&wavm_ret22.value != 1684234849) {
    return 22;
  }
  
  wavm_ret_int32_t wavm_ret23 = wavm_exported_function_32_good3(NULL,0);
  if (*(uint32_t *)&wavm_ret23.value != 1701077858) {
    return 23;
  }
  
  wavm_ret_int32_t wavm_ret24 = wavm_exported_function_32_good4(NULL,0);
  if (*(uint32_t *)&wavm_ret24.value != 1717920867) {
    return 24;
  }
  
  wavm_ret_int32_t wavm_ret25 = wavm_exported_function_32_good5(NULL,0);
  if (*(uint32_t *)&wavm_ret25.value != 122) {
    return 25;
  }
  
  wavm_ret_int32_t wavm_ret26 = wavm_exported_function_8u_good1(NULL,65507);
  if (*(uint32_t *)&wavm_ret26.value != 0) {
    return 26;
  }
  
  wavm_ret_int32_t wavm_ret27 = wavm_exported_function_8u_good2(NULL,65507);
  if (*(uint32_t *)&wavm_ret27.value != 0) {
    return 27;
  }
  
  wavm_ret_int32_t wavm_ret28 = wavm_exported_function_8u_good3(NULL,65507);
  if (*(uint32_t *)&wavm_ret28.value != 0) {
    return 28;
  }
  
  wavm_ret_int32_t wavm_ret29 = wavm_exported_function_8u_good4(NULL,65507);
  if (*(uint32_t *)&wavm_ret29.value != 0) {
    return 29;
  }
  
  wavm_ret_int32_t wavm_ret30 = wavm_exported_function_8u_good5(NULL,65507);
  if (*(uint32_t *)&wavm_ret30.value != 0) {
    return 30;
  }
  
  wavm_ret_int32_t wavm_ret31 = wavm_exported_function_8s_good1(NULL,65507);
  if (*(uint32_t *)&wavm_ret31.value != 0) {
    return 31;
  }
  
  wavm_ret_int32_t wavm_ret32 = wavm_exported_function_8s_good2(NULL,65507);
  if (*(uint32_t *)&wavm_ret32.value != 0) {
    return 32;
  }
  
  wavm_ret_int32_t wavm_ret33 = wavm_exported_function_8s_good3(NULL,65507);
  if (*(uint32_t *)&wavm_ret33.value != 0) {
    return 33;
  }
  
  wavm_ret_int32_t wavm_ret34 = wavm_exported_function_8s_good4(NULL,65507);
  if (*(uint32_t *)&wavm_ret34.value != 0) {
    return 34;
  }
  
  wavm_ret_int32_t wavm_ret35 = wavm_exported_function_8s_good5(NULL,65507);
  if (*(uint32_t *)&wavm_ret35.value != 0) {
    return 35;
  }
  
  wavm_ret_int32_t wavm_ret36 = wavm_exported_function_16u_good1(NULL,65507);
  if (*(uint32_t *)&wavm_ret36.value != 0) {
    return 36;
  }
  
  wavm_ret_int32_t wavm_ret37 = wavm_exported_function_16u_good2(NULL,65507);
  if (*(uint32_t *)&wavm_ret37.value != 0) {
    return 37;
  }
  
  wavm_ret_int32_t wavm_ret38 = wavm_exported_function_16u_good3(NULL,65507);
  if (*(uint32_t *)&wavm_ret38.value != 0) {
    return 38;
  }
  
  wavm_ret_int32_t wavm_ret39 = wavm_exported_function_16u_good4(NULL,65507);
  if (*(uint32_t *)&wavm_ret39.value != 0) {
    return 39;
  }
  
  wavm_ret_int32_t wavm_ret40 = wavm_exported_function_16u_good5(NULL,65507);
  if (*(uint32_t *)&wavm_ret40.value != 0) {
    return 40;
  }
  
  wavm_ret_int32_t wavm_ret41 = wavm_exported_function_16s_good1(NULL,65507);
  if (*(uint32_t *)&wavm_ret41.value != 0) {
    return 41;
  }
  
  wavm_ret_int32_t wavm_ret42 = wavm_exported_function_16s_good2(NULL,65507);
  if (*(uint32_t *)&wavm_ret42.value != 0) {
    return 42;
  }
  
  wavm_ret_int32_t wavm_ret43 = wavm_exported_function_16s_good3(NULL,65507);
  if (*(uint32_t *)&wavm_ret43.value != 0) {
    return 43;
  }
  
  wavm_ret_int32_t wavm_ret44 = wavm_exported_function_16s_good4(NULL,65507);
  if (*(uint32_t *)&wavm_ret44.value != 0) {
    return 44;
  }
  
  wavm_ret_int32_t wavm_ret45 = wavm_exported_function_16s_good5(NULL,65507);
  if (*(uint32_t *)&wavm_ret45.value != 0) {
    return 45;
  }
  
  wavm_ret_int32_t wavm_ret46 = wavm_exported_function_32_good1(NULL,65507);
  if (*(uint32_t *)&wavm_ret46.value != 0) {
    return 46;
  }
  
  wavm_ret_int32_t wavm_ret47 = wavm_exported_function_32_good2(NULL,65507);
  if (*(uint32_t *)&wavm_ret47.value != 0) {
    return 47;
  }
  
  wavm_ret_int32_t wavm_ret48 = wavm_exported_function_32_good3(NULL,65507);
  if (*(uint32_t *)&wavm_ret48.value != 0) {
    return 48;
  }
  
  wavm_ret_int32_t wavm_ret49 = wavm_exported_function_32_good4(NULL,65507);
  if (*(uint32_t *)&wavm_ret49.value != 0) {
    return 49;
  }
  
  wavm_ret_int32_t wavm_ret50 = wavm_exported_function_32_good5(NULL,65507);
  if (*(uint32_t *)&wavm_ret50.value != 0) {
    return 50;
  }
  
  wavm_ret_int32_t wavm_ret51 = wavm_exported_function_8u_good1(NULL,65508);
  if (*(uint32_t *)&wavm_ret51.value != 0) {
    return 51;
  }
  
  wavm_ret_int32_t wavm_ret52 = wavm_exported_function_8u_good2(NULL,65508);
  if (*(uint32_t *)&wavm_ret52.value != 0) {
    return 52;
  }
  
  wavm_ret_int32_t wavm_ret53 = wavm_exported_function_8u_good3(NULL,65508);
  if (*(uint32_t *)&wavm_ret53.value != 0) {
    return 53;
  }
  
  wavm_ret_int32_t wavm_ret54 = wavm_exported_function_8u_good4(NULL,65508);
  if (*(uint32_t *)&wavm_ret54.value != 0) {
    return 54;
  }
  
  wavm_ret_int32_t wavm_ret55 = wavm_exported_function_8u_good5(NULL,65508);
  if (*(uint32_t *)&wavm_ret55.value != 0) {
    return 55;
  }
  
  wavm_ret_int32_t wavm_ret56 = wavm_exported_function_8s_good1(NULL,65508);
  if (*(uint32_t *)&wavm_ret56.value != 0) {
    return 56;
  }
  
  wavm_ret_int32_t wavm_ret57 = wavm_exported_function_8s_good2(NULL,65508);
  if (*(uint32_t *)&wavm_ret57.value != 0) {
    return 57;
  }
  
  wavm_ret_int32_t wavm_ret58 = wavm_exported_function_8s_good3(NULL,65508);
  if (*(uint32_t *)&wavm_ret58.value != 0) {
    return 58;
  }
  
  wavm_ret_int32_t wavm_ret59 = wavm_exported_function_8s_good4(NULL,65508);
  if (*(uint32_t *)&wavm_ret59.value != 0) {
    return 59;
  }
  
  wavm_ret_int32_t wavm_ret60 = wavm_exported_function_8s_good5(NULL,65508);
  if (*(uint32_t *)&wavm_ret60.value != 0) {
    return 60;
  }
  
  wavm_ret_int32_t wavm_ret61 = wavm_exported_function_16u_good1(NULL,65508);
  if (*(uint32_t *)&wavm_ret61.value != 0) {
    return 61;
  }
  
  wavm_ret_int32_t wavm_ret62 = wavm_exported_function_16u_good2(NULL,65508);
  if (*(uint32_t *)&wavm_ret62.value != 0) {
    return 62;
  }
  
  wavm_ret_int32_t wavm_ret63 = wavm_exported_function_16u_good3(NULL,65508);
  if (*(uint32_t *)&wavm_ret63.value != 0) {
    return 63;
  }
  
  wavm_ret_int32_t wavm_ret64 = wavm_exported_function_16u_good4(NULL,65508);
  if (*(uint32_t *)&wavm_ret64.value != 0) {
    return 64;
  }
  
  wavm_ret_int32_t wavm_ret65 = wavm_exported_function_16u_good5(NULL,65508);
  if (*(uint32_t *)&wavm_ret65.value != 0) {
    return 65;
  }
  
  wavm_ret_int32_t wavm_ret66 = wavm_exported_function_16s_good1(NULL,65508);
  if (*(uint32_t *)&wavm_ret66.value != 0) {
    return 66;
  }
  
  wavm_ret_int32_t wavm_ret67 = wavm_exported_function_16s_good2(NULL,65508);
  if (*(uint32_t *)&wavm_ret67.value != 0) {
    return 67;
  }
  
  wavm_ret_int32_t wavm_ret68 = wavm_exported_function_16s_good3(NULL,65508);
  if (*(uint32_t *)&wavm_ret68.value != 0) {
    return 68;
  }
  
  wavm_ret_int32_t wavm_ret69 = wavm_exported_function_16s_good4(NULL,65508);
  if (*(uint32_t *)&wavm_ret69.value != 0) {
    return 69;
  }
  
  wavm_ret_int32_t wavm_ret70 = wavm_exported_function_16s_good5(NULL,65508);
  if (*(uint32_t *)&wavm_ret70.value != 0) {
    return 70;
  }
  
  wavm_ret_int32_t wavm_ret71 = wavm_exported_function_32_good1(NULL,65508);
  if (*(uint32_t *)&wavm_ret71.value != 0) {
    return 71;
  }
  
  wavm_ret_int32_t wavm_ret72 = wavm_exported_function_32_good2(NULL,65508);
  if (*(uint32_t *)&wavm_ret72.value != 0) {
    return 72;
  }
  
  wavm_ret_int32_t wavm_ret73 = wavm_exported_function_32_good3(NULL,65508);
  if (*(uint32_t *)&wavm_ret73.value != 0) {
    return 73;
  }
  
  wavm_ret_int32_t wavm_ret74 = wavm_exported_function_32_good4(NULL,65508);
  if (*(uint32_t *)&wavm_ret74.value != 0) {
    return 74;
  }
  
}
