#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "ckb_syscalls.h"

#ifndef WAVM_CKB_VM_WASI_ABI_H
#define WAVM_CKB_VM_WASI_ABI_H

/*
 * Since we are leveraging the unique memory model in CKB VM to optimize
 * the code, malloc is not allowed here.
 */
#define malloc malloc_is_not_allowed_in_wavm_runtime

#ifndef MEMORY0_DEFINED
extern uint8_t* memoryOffset0;
#endif

void callIndirectFail() {
  ckb_debug("Call indirect fail!");
  ckb_exit(-2);
}

void unreachableTrap() {
  ckb_debug("This should not be reached!");
  ckb_exit(-1);
}

long __atomic_load_8(void* p, int32_t _mode)
{
  (void) _mode;
  return *((uint64_t*) ((uintptr_t) p));
}

#ifndef WAVM_MAX_MEMORY
#define WAVM_MAX_MEMORY 0x300000
#endif  /* WAVM_MAX_MEMORY */
#define WAVM_PAGE_SIZE 0x10000

/*
 * Note that this optimized memory grow implementation must be used together with
 * the custom linker script `riscv64.lds` in current directory. Otherwise you might
 * run into unexpected behaviors.
 */
int32_t wavm_intrinsic_memory_grow(void* dummy, int32_t grow_by) {
  if ((uintptr_t) (memoryOffset0 + memory0_length + grow_by * WAVM_PAGE_SIZE) > WAVM_MAX_MEMORY) {
    ckb_debug("Grow page failure!");
    return -1;
  }

  int32_t old_pages = memory0_length / WAVM_PAGE_SIZE;
  memory0_length += grow_by * WAVM_PAGE_SIZE;
  return old_pages;
}

wavm_ret_int32_t wavm_wasi_unstable_fd_write(void* dummy, int32_t fd, int32_t address, int32_t num, int32_t written_bytes_address)
{
  static uint8_t temp_buffer[65];
  uint8_t* current_memory = (uint8_t*) memoryOffset0;

  int32_t written_bytes = 0;
  for (int32_t i = 0; i < num; i++) {

    uint32_t buffer_address = *((uint32_t*) &current_memory[address + i * 8]);
    uint8_t* buf = &current_memory[buffer_address];
    uint32_t buffer_length = *((uint32_t*) &current_memory[address + i * 8 + 4]);

    int32_t written = 0;
    while (written < buffer_length) {
      int32_t left_bytes = buffer_length - written;
      int32_t b = (left_bytes > 64) ? 64 : left_bytes;
      memcpy(temp_buffer, &buf[written], b);
      temp_buffer[b] = '\0';
      ckb_debug((const char*) temp_buffer);

      written += b;
    }

    written_bytes += buffer_length;
  }
  if (written_bytes_address != 0) {
    *((uint32_t*) &current_memory[written_bytes_address]) = written_bytes;
  }

  wavm_ret_int32_t ret;
  ret.dummy = dummy;
  ret.value = 0;
  return ret;
}

void* wavm_wasi_unstable_proc_exit(void* dummy, int32_t code)
{
  ckb_exit(code);

  return dummy;
}

#endif  /* WAVM_CKB_VM_WASI_ABI_H */
