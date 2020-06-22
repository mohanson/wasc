#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <unistd.h>
#include <string.h>

#ifndef WAVM_POSIX_X86_64_WASI_H
#define WAVM_POSIX_X86_64_WASI_H

#define WAVM_PAGE_SIZE 0x10000
#ifndef MEMORY0_MAX_PAGE
#define MEMORY0_MAX_PAGE 65536
#endif /* MEMORY0_MAX_PAGE */

#ifdef MEMORY0_DEFINED
int32_t wavm_intrinsic_memory_grow(void *dummy, int32_t grow_by)
{
  if (grow_by == 0)
  {
    return memoryOffset0.num_pages;
  }
  int32_t old_pages = memoryOffset0.num_pages;
  int32_t new_pages = old_pages + grow_by;
  if (new_pages > MEMORY0_MAX_PAGE)
  {
    return -1;
  }
  size_t old_size = old_pages * WAVM_PAGE_SIZE;
  size_t new_size = new_pages * WAVM_PAGE_SIZE;
  memory0 = realloc(memory0, new_size);
  memset(memory0 + old_size, 0, new_size - old_size);
  memoryOffset0.base = memory0;
  memoryOffset0.num_pages = new_pages;
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
  exit(255);
}

void unreachableTrap()
{
  exit(254);
}

void divideByZeroOrIntegerOverflowTrap()
{
  exit(253);
}

void invalidFloatOperationTrap()
{
  exit(252);
}

wavm_ret_int32_t wavm_wasi_unstable_args_sizes_get(void *dummy, int32_t argc_address, int32_t arg_buf_size_address)
{
  (void)dummy;

  int32_t num_arg_buffer_bytes = 0;
  for (int32_t i = 0; i < g_argc; i++)
  {
    num_arg_buffer_bytes = num_arg_buffer_bytes + strlen(g_argv[i]) + 1;
  }
  *((uint32_t *)&memoryOffset0.base[argc_address]) = g_argc;
  *((uint32_t *)&memoryOffset0.base[arg_buf_size_address]) = num_arg_buffer_bytes;

  wavm_ret_int32_t ret;
  ret.dummy = dummy;
  ret.value = 0;
  return ret;
}

wavm_ret_int32_t wavm_wasi_unstable_args_get(void *dummy, int32_t argv_address, int32_t arg_buf_address)
{
  (void)dummy;

  int32_t next_arg_buf_address = arg_buf_address;
  for (int32_t i = 0; i < g_argc; ++i)
  {
    char *arg = g_argv[i];
    int32_t num_arg_bytes = strlen(arg) + 1;
    memcpy(&memoryOffset0.base[next_arg_buf_address], arg, num_arg_bytes);
    *((uint32_t *)&memoryOffset0.base[argv_address + i * 4]) = next_arg_buf_address;
    next_arg_buf_address += num_arg_bytes;
  }
  wavm_ret_int32_t ret;
  ret.dummy = dummy;
  ret.value = 0;
  return ret;
}

extern char **environ;

wavm_ret_int32_t wavm_wasi_unstable_environ_sizes_get(void *dummy, int32_t env_count_address, int32_t env_buf_size_address)
{
  (void)dummy;

  int32_t num_env_buffer_bytes = 0;
  int32_t envc = 0;
  for (char **ep = environ; *ep != NULL; ep++)
  {
    envc++;
    num_env_buffer_bytes = num_env_buffer_bytes + strlen(*ep) + 1;
  }
  *((uint32_t *)&memoryOffset0.base[env_count_address]) = envc;
  *((uint32_t *)&memoryOffset0.base[env_buf_size_address]) = num_env_buffer_bytes;
  wavm_ret_int32_t ret;
  ret.dummy = dummy;
  ret.value = 0;
  return ret;
}

wavm_ret_int32_t wavm_wasi_unstable_environ_get(void *dummy, int32_t env_address, int32_t env_buf_address)
{
  (void)dummy;

  int32_t next_env_buf_address = env_buf_address;
  int32_t i = 0;
  for (char **ep = environ; *ep != NULL; ep++)
  {
    char *env = *ep;
    int32_t num_env_bytes = strlen(env) + 1;
    memcpy(&memoryOffset0.base[next_env_buf_address], env, num_env_bytes);
    *((uint32_t *)&memoryOffset0.base[env_address + i * 4]) = next_env_buf_address;
    next_env_buf_address += num_env_bytes;
    ++i;
  }
  wavm_ret_int32_t ret;
  ret.dummy = dummy;
  ret.value = 0;
  return ret;
}

wavm_ret_int32_t wavm_wasi_unstable_fd_write(void *dummy, int32_t fd, int32_t address, int32_t num, int32_t written_bytes_address)
{
  (void)dummy;

  int32_t written_bytes = 0;
  for (int32_t i = 0; i < num; i++)
  {
    uint32_t buffer_address = *((uint32_t *)&memoryOffset0.base[address + i * 8]);
    uint8_t *buf = &memoryOffset0.base[buffer_address];
    uint32_t buffer_length = *((uint32_t *)&memoryOffset0.base[address + i * 8 + 4]);

    int32_t written = write(fd, buf, buffer_length);
    written_bytes += written;
  }
  if (written_bytes_address != 0)
  {
    *((uint32_t *)&memoryOffset0.base[written_bytes_address]) = written_bytes;
  }
  wavm_ret_int32_t ret;
  ret.dummy = dummy;
  ret.value = 0;
  return ret;
}

void *wavm_wasi_unstable_proc_exit(void *dummy, int32_t code)
{
  exit(code);
  return dummy;
}

#endif /* WAVM_POSIX_X86_64_WASI_H */
