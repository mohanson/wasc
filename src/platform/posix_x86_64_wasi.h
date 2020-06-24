#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <unistd.h>
#include <string.h>
#include <time.h>

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

#define __WASI_ESUCCESS 0
#define __WASI_E2BIG 1
#define __WASI_EACCES 2
#define __WASI_EADDRINUSE 3
#define __WASI_EADDRNOTAVAIL 4
#define __WASI_EAFNOSUPPORT 5
#define __WASI_EAGAIN 6
#define __WASI_EALREADY 7
#define __WASI_EBADF 8
#define __WASI_EBADMSG 9
#define __WASI_EBUSY 10
#define __WASI_ECANCELED 11
#define __WASI_ECHILD 12
#define __WASI_ECONNABORTED 13
#define __WASI_ECONNREFUSED 14
#define __WASI_ECONNRESET 15
#define __WASI_EDEADLK 16
#define __WASI_EDESTADDRREQ 17
#define __WASI_EDOM 18
#define __WASI_EDQUOT 19
#define __WASI_EEXIST 20
#define __WASI_EFAULT 21
#define __WASI_EFBIG 22
#define __WASI_EHOSTUNREACH 23
#define __WASI_EIDRM 24
#define __WASI_EILSEQ 25
#define __WASI_EINPROGRESS 26
#define __WASI_EINTR 27
#define __WASI_EINVAL 28
#define __WASI_EIO 29
#define __WASI_EISCONN 30
#define __WASI_EISDIR 31
#define __WASI_ELOOP 32
#define __WASI_EMFILE 33
#define __WASI_EMLINK 34
#define __WASI_EMSGSIZE 35
#define __WASI_EMULTIHOP 36
#define __WASI_ENAMETOOLONG 37
#define __WASI_ENETDOWN 38
#define __WASI_ENETRESET 39
#define __WASI_ENETUNREACH 40
#define __WASI_ENFILE 41
#define __WASI_ENOBUFS 42
#define __WASI_ENODEV 43
#define __WASI_ENOENT 44
#define __WASI_ENOEXEC 45
#define __WASI_ENOLCK 46
#define __WASI_ENOLINK 47
#define __WASI_ENOMEM 48
#define __WASI_ENOMSG 49
#define __WASI_ENOPROTOOPT 50
#define __WASI_ENOSPC 51
#define __WASI_ENOSYS 52
#define __WASI_ENOTCONN 53
#define __WASI_ENOTDIR 54
#define __WASI_ENOTEMPTY 55
#define __WASI_ENOTRECOVERABLE 56
#define __WASI_ENOTSOCK 57
#define __WASI_ENOTSUP 58
#define __WASI_ENOTTY 59
#define __WASI_ENXIO 60
#define __WASI_EOVERFLOW 61
#define __WASI_EOWNERDEAD 62
#define __WASI_EPERM 63
#define __WASI_EPIPE 64
#define __WASI_EPROTO 65
#define __WASI_EPROTONOSUPPORT 66
#define __WASI_EPROTOTYPE 67
#define __WASI_ERANGE 68
#define __WASI_EROFS 69
#define __WASI_ESPIPE 70
#define __WASI_ESRCH 71
#define __WASI_ESTALE 72
#define __WASI_ETIMEDOUT 73
#define __WASI_ETXTBSY 74
#define __WASI_EXDEV 75
#define __WASI_ENOTCAPABLE 76

wavm_ret_int32_t pack_errno(void *dummy, int32_t value)
{
  wavm_ret_int32_t ret;
  ret.dummy = dummy;
  ret.value = value;
  return ret;
}

int32_t wasi_errno(int error)
{
  switch (error)
  {
  case ESPIPE:
    return __WASI_ESPIPE;
  case EIO:
    return __WASI_EIO;
  case EINTR:
    return __WASI_EINTR;
  case EISDIR:
    return __WASI_EISDIR;
  case EFAULT:
    return __WASI_EFAULT;
  case EFBIG:
    return __WASI_EFBIG;
  case EPERM:
    return __WASI_EPERM;
  case EOVERFLOW:
    return __WASI_EOVERFLOW;
  case EMFILE:
    return __WASI_EMFILE;
  case ENOTDIR:
    return __WASI_ENOTDIR;
  case EACCES:
    return __WASI_EACCES;
  case EEXIST:
    return __WASI_EEXIST;
  case ENAMETOOLONG:
    return __WASI_ENAMETOOLONG;
  case ENFILE:
    return __WASI_ENFILE;
  case ENOENT:
    return __WASI_ENOENT;
  case ENOSPC:
    return __WASI_ENOSPC;
  case EROFS:
    return __WASI_EPERM;
  case ENOMEM:
    return __WASI_ENOMEM;
  case EDQUOT:
    return __WASI_EDQUOT;
  case ELOOP:
    return __WASI_ELOOP;
  case EAGAIN:
    return __WASI_EAGAIN;
  case EINPROGRESS:
    return __WASI_EINPROGRESS;
  case ENOSR:
    return __WASI_ENOMEM;
  case ENXIO:
    return __WASI_ENXIO;
  case ETXTBSY:
    return __WASI_EACCES;
  case EBUSY:
    return __WASI_EBUSY;
  case ENOTEMPTY:
    return __WASI_ENOTEMPTY;
  case EMLINK:
    return __WASI_EMLINK;
  case ENOTSUP:
    return __WASI_ENOTSUP;
  case EINVAL:
    return __WASI_EINVAL;
  case EBADF:
    return __WASI_EBADF;
  default:
    return error;
  };
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
  return pack_errno(dummy, 0);
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
  return pack_errno(dummy, 0);
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
  return pack_errno(dummy, 0);
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
  return pack_errno(dummy, 0);
}

wavm_ret_int32_t wavm_wasi_unstable_clock_res_get(void *dummy, uint32_t clock_id, uint32_t resolution_address)
{
  (void)dummy;
  struct timespec tp;
  if (!clock_getres(clock_id, &tp))
  {
    return pack_errno(dummy, __WASI_EINVAL);
  }
  *((uint64_t *)&memoryOffset0.base[resolution_address]) = tp.tv_nsec;
  return pack_errno(dummy, 0);
}

wavm_ret_int32_t wavm_wasi_unstable_clock_time_get(void *dummy, uint32_t clock_id, uint64_t precision, uint32_t time_address)
{
  (void)dummy;
  struct timespec tp;
  if (!clock_gettime(clock_id, &tp))
  {
    return pack_errno(dummy, __WASI_EINVAL);
  }
  *((uint64_t *)&memoryOffset0.base[time_address]) = tp.tv_nsec;
  return pack_errno(dummy, 0);
}

void *wavm_wasi_unstable_fd_advise(void *dummy) {}
void *wavm_wasi_unstable_fd_allocate(void *dummy) {}

wavm_ret_int32_t wavm_wasi_unstable_fd_close(void *dummy, int32_t fd)
{
  (void)dummy;
  int32_t r = close(fd);
  if (r != 0)
  {
    return pack_errno(dummy, __WASI_EBADF);
  }
  return pack_errno(dummy, 0);
}

void *wavm_wasi_unstable_fd_datasync(void *dummy) {}

wavm_ret_int32_t wavm_wasi_unstable_fd_fdstat_get(void *dummy, int32_t fd, int32_t fdstat_address)
{
  return pack_errno(dummy, 0);
}

void *wavm_wasi_unstable_fd_fdstat_set_flags(void *dummy) {}
void *wavm_wasi_unstable_fd_fdstat_set_rights(void *dummy) {}
void *wavm_wasi_unstable_fd_filestat_get(void *dummy) {}
void *wavm_wasi_unstable_fd_filestat_set_size(void *dummy) {}
void *wavm_wasi_unstable_fd_filestat_set_times(void *dummy) {}
void *wavm_wasi_unstable_fd_pread(void *dummy) {}
void *wavm_wasi_unstable_fd_prestat_get(void *dummy) {}
void *wavm_wasi_unstable_fd_prestat_dir_name(void *dummy) {}
void *wavm_wasi_unstable_fd_pwrite(void *dummy) {}
void *wavm_wasi_unstable_fd_read(void *dummy) {}
void *wavm_wasi_unstable_fd_readdir(void *dummy) {}
void *wavm_wasi_unstable_fd_renumber(void *dummy) {}

wavm_ret_int32_t wavm_wasi_unstable_fd_seek(void *dummy, int32_t fd, int64_t offset, int32_t whence, int32_t new_offset_address)
{
  (void)dummy;
  off_t result = lseek(fd, (off_t)offset, whence);
  if (result == -1)
  {
    if (errno == EINVAL)
    {
      return pack_errno(dummy, __WASI_EINVAL);
    }
    return pack_errno(dummy, wasi_errno(errno));
  }
  *((uint64_t *)&memoryOffset0.base[new_offset_address]) = result;
  return pack_errno(dummy, 0);
}

void *wavm_wasi_unstable_fd_sync(void *dummy) {}
void *wavm_wasi_unstable_fd_tell(void *dummy) {}
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
  return pack_errno(dummy, 0);
}

void *wavm_wasi_unstable_path_create_directory(void *dummy) {}
void *wavm_wasi_unstable_path_filestat_get(void *dummy) {}
void *wavm_wasi_unstable_path_filestat_set_times(void *dummy) {}
void *wavm_wasi_unstable_path_link(void *dummy) {}
void *wavm_wasi_unstable_path_open(void *dummy) {}
void *wavm_wasi_unstable_path_readlink(void *dummy) {}
void *wavm_wasi_unstable_path_remove_directory(void *dummy) {}
void *wavm_wasi_unstable_path_rename(void *dummy) {}
void *wavm_wasi_unstable_path_symlink(void *dummy) {}
void *wavm_wasi_unstable_path_unlink_file(void *dummy) {}

void *wavm_wasi_unstable_poll_oneoff(void *dummy) {}
void *wavm_wasi_unstable_proc_exit(void *dummy, int32_t code)
{
  exit(code);
  return dummy;
}
void *wavm_wasi_unstable_proc_raise(void *dummy) {}
void *wavm_wasi_unstable_sched_yield(void *dummy) {}
void *wavm_wasi_unstable_random_get(void *dummy) {}
void *wavm_wasi_unstable_sock_recv(void *dummy) {}
void *wavm_wasi_unstable_sock_send(void *dummy) {}
void *wavm_wasi_unstable_sock_shutdown(void *dummy) {}

#endif /* WAVM_POSIX_X86_64_WASI_H */
