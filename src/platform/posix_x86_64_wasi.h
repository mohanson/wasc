#include <dirent.h>
#include <fcntl.h>
#include <errno.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <sys/uio.h>
#include <time.h>
#include <unistd.h>

#include "common/wavm.h"
#include "common/wasi.h"

#ifndef WAVM_POSIX_X86_64_WASI_H
#define WAVM_POSIX_X86_64_WASI_H

#define DEBUG

extern int32_t g_argc;
extern char **g_argv;

#define WAVM_PAGE_SIZE 0x10000
#ifndef MEMORY0_MAX_PAGE
#define MEMORY0_MAX_PAGE 65536
#endif /* MEMORY0_MAX_PAGE */

extern memory_instance memoryOffset0;
extern uint8_t *memory0;

int32_t wavm_intrinsic_memory_grow(void *dummy, int32_t grow_by)
{
#ifdef DEBUG
  printf("wavm_intrinsic_memory_grow grow_by=%d\n", grow_by);
#endif
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
  memory0 = (uint8_t *)realloc(memory0, new_size);
  memset(memory0 + old_size, 0, new_size - old_size);
  memoryOffset0.base = memory0;
  memoryOffset0.num_pages = new_pages;
  return old_pages;
}

void callIndirectFail()
{
#ifdef DEBUG
  printf("callIndirectFail\n");
#endif
  exit(1);
}

void unreachableTrap()
{
#ifdef DEBUG
  printf("unreachableTrap\n");
#endif
  exit(1);
}

void divideByZeroOrIntegerOverflowTrap()
{
#ifdef DEBUG
  printf("divideByZeroOrIntegerOverflowTrap\n");
#endif
  exit(1);
}

void invalidFloatOperationTrap()
{
#ifdef DEBUG
  printf("invalidFloatOperationTrap\n");
#endif
  exit(1);
}

#define STDIO_RIGHTS (__WASI_RIGHT_FD_READ | __WASI_RIGHT_FD_FDSTAT_SET_FLAGS | __WASI_RIGHT_FD_WRITE | \
                      __WASI_RIGHT_FD_FILESTAT_GET | __WASI_RIGHT_POLL_FD_READWRITE)
#define REGULAR_FILE_RIGHTS (__WASI_RIGHT_FD_DATASYNC | __WASI_RIGHT_FD_READ | __WASI_RIGHT_FD_SEEK |         \
                             __WASI_RIGHT_FD_FDSTAT_SET_FLAGS | __WASI_RIGHT_FD_SYNC | __WASI_RIGHT_FD_TELL | \
                             __WASI_RIGHT_FD_WRITE | __WASI_RIGHT_FD_ADVISE | __WASI_RIGHT_FD_ALLOCATE |      \
                             __WASI_RIGHT_FD_FILESTAT_GET | __WASI_RIGHT_FD_FILESTAT_SET_SIZE |               \
                             __WASI_RIGHT_FD_FILESTAT_SET_TIMES | __WASI_RIGHT_POLL_FD_READWRITE)
#define DIRECTORY_RIGHTS (__WASI_RIGHT_FD_FDSTAT_SET_FLAGS | __WASI_RIGHT_FD_SYNC | __WASI_RIGHT_FD_ADVISE |       \
                          __WASI_RIGHT_PATH_CREATE_DIRECTORY | __WASI_RIGHT_PATH_CREATE_FILE |                     \
                          __WASI_RIGHT_PATH_LINK_SOURCE | __WASI_RIGHT_PATH_LINK_TARGET | __WASI_RIGHT_PATH_OPEN | \
                          __WASI_RIGHT_FD_READDIR | __WASI_RIGHT_PATH_READLINK | __WASI_RIGHT_PATH_RENAME_SOURCE | \
                          __WASI_RIGHT_PATH_RENAME_TARGET | __WASI_RIGHT_PATH_FILESTAT_GET |                       \
                          __WASI_RIGHT_PATH_FILESTAT_SET_SIZE | __WASI_RIGHT_PATH_FILESTAT_SET_TIMES |             \
                          __WASI_RIGHT_FD_FILESTAT_GET | __WASI_RIGHT_FD_FILESTAT_SET_TIMES |                      \
                          __WASI_RIGHT_PATH_SYMLINK | __WASI_RIGHT_PATH_UNLINK_FILE |                              \
                          __WASI_RIGHT_PATH_REMOVE_DIRECTORY | __WASI_RIGHT_POLL_FD_READWRITE)
#define INHERITING_DIRECTORY_RIGHTS (DIRECTORY_RIGHTS | REGULAR_FILE_RIGHTS)

typedef struct Preopen
{
  char *path;
  int32_t path_len;
} Preopen;

#define PREOPEN_CNT 7
Preopen preopen[PREOPEN_CNT] = {
    {
        .path = "<stdin>",
        .path_len = 7,
    },
    {
        .path = "<stdout>",
        .path_len = 8,
    },
    {
        .path = "<stderr>",
        .path_len = 8,
    },
    {
        .path = "./",
        .path_len = 2,
    },
    {
        .path = "../",
        .path_len = 3,
    },
    {
        .path = "/",
        .path_len = 1,
    },
    {
        .path = "/tmp",
        .path_len = 4,
    },
};

void init_wasi()
{
  for (int fd = 3; fd < PREOPEN_CNT; fd++)
  {
    if (fcntl(fd, F_GETFD, 0) >= 0)
    {
      close(fd);
    }
  }
  for (int fd = 3; fd < PREOPEN_CNT; fd++)
  {
    int tfd = open(preopen[fd].path, O_RDONLY);
    if (tfd < 0)
    {
      printf("opening '%s': %s\n", "./", strerror(errno));
      exit(1);
    }
    if (tfd != fd)
    {
      printf("fd %d could not be freed up before preopen\n", fd);
      exit(1);
    }
  }
}

#define MAX_IOV 128

struct iovec host_iov[MAX_IOV];

struct iovec *copy_iov_to_host(uint32_t iov_offset, uint32_t iovs_len)
{
  if (iovs_len > MAX_IOV)
  {
    printf("copy_iov_to_host called with iovs_len > 128\n");
    exit(1);
  }
  struct iovec *wasi_iov = (struct iovec *)&memoryOffset0.base[iov_offset];
  for (int32_t i = 0; i < iovs_len; i++)
  {
    uint32_t buffer_address = *((uint32_t *)&memoryOffset0.base[iov_offset + i * 8]);
    uint8_t *buf = &memoryOffset0.base[buffer_address];
    uint32_t buffer_length = *((uint32_t *)&memoryOffset0.base[iov_offset + i * 8 + 4]);
    host_iov[i].iov_base = buf;
    host_iov[i].iov_len = buffer_length;
  }
  return host_iov;
}

#define MAX_PATH_LENGTH 1024

__wasi_timestamp_t conv_posix_timespec_2_wasi_timestamp(struct timespec t)
{
  return t.tv_sec * 1000000000 + t.tv_nsec;
}

int32_t conv_wasi_lookupflags_2_posix_lookupflags(__wasi_lookupflags_t lookup_flags)
{
  int32_t f = 0;
  if ((lookup_flags & __WASI_LOOKUP_SYMLINK_FOLLOW) == 0)
  {
    f |= AT_SYMLINK_NOFOLLOW;
  }
  return f;
}

__wasi_errno_t conv_posix_errno_2_wasi_errno(int error)
{
  switch (error)
  {
  case EPERM:
    return __WASI_EPERM;
  case ENOENT:
    return __WASI_ENOENT;
  case ESRCH:
    return __WASI_ESRCH;
  case EINTR:
    return __WASI_EINTR;
  case EIO:
    return __WASI_EIO;
  case ENXIO:
    return __WASI_ENXIO;
  case E2BIG:
    return __WASI_E2BIG;
  case ENOEXEC:
    return __WASI_ENOEXEC;
  case EBADF:
    return __WASI_EBADF;
  case ECHILD:
    return __WASI_ECHILD;
  case EAGAIN:
    return __WASI_EAGAIN;
  case ENOMEM:
    return __WASI_ENOMEM;
  case EACCES:
    return __WASI_EACCES;
  case EFAULT:
    return __WASI_EFAULT;
  case ENOTBLK:
    return __WASI_FDFLAG_NONBLOCK;
  case EBUSY:
    return __WASI_EBUSY;
  case EEXIST:
    return __WASI_EEXIST;
  case EXDEV:
    return __WASI_EXDEV;
  case ENODEV:
    return __WASI_ENODEV;
  case ENOTDIR:
    return __WASI_ENOTDIR;
  case EISDIR:
    return __WASI_EISDIR;
  case EINVAL:
    return __WASI_EINVAL;
  case ENFILE:
    return __WASI_ENFILE;
  case EMFILE:
    return __WASI_EMFILE;
  case ENOTTY:
    return __WASI_ENOTTY;
  case ETXTBSY:
    return __WASI_ETXTBSY;
  case EFBIG:
    return __WASI_EFBIG;
  case ENOSPC:
    return __WASI_ENOSPC;
  case ESPIPE:
    return __WASI_ESPIPE;
  case EROFS:
    return __WASI_EROFS;
  case EMLINK:
    return __WASI_EMLINK;
  case EPIPE:
    return __WASI_EPIPE;
  case EDOM:
    return __WASI_EDOM;
  case ERANGE:
    return __WASI_ERANGE;
  case ENOTEMPTY:
    return __WASI_ENOTEMPTY;
  default:
    printf("unhandled posix errno=%d %s\n", errno, strerror(errno));
    exit(1);
  }
}

__wasi_filetype_t as_wasi_file_type(mode_t mode)
{
  switch (mode)
  {
  case DT_BLK:
    return __WASI_FILETYPE_BLOCK_DEVICE;
  case DT_CHR:
    return __WASI_FILETYPE_CHARACTER_DEVICE;
  case DT_DIR:
    return __WASI_FILETYPE_DIRECTORY;
  case DT_FIFO:
    return __WASI_FILETYPE_UNKNOWN;
  case DT_LNK:
    return __WASI_FILETYPE_SYMBOLIC_LINK;
  case DT_REG:
    return __WASI_FILETYPE_REGULAR_FILE;
  default:
    return __WASI_FILETYPE_UNKNOWN;
  };
}

__wasi_filetype_t get_filetype_from_mode(mode_t mode)
{
  switch (mode & S_IFMT)
  {
  case S_IFBLK:
    return __WASI_FILETYPE_BLOCK_DEVICE;
  case S_IFCHR:
    return __WASI_FILETYPE_CHARACTER_DEVICE;
  case S_IFIFO:
    return __WASI_FILETYPE_UNKNOWN;
  case S_IFREG:
    return __WASI_FILETYPE_REGULAR_FILE;
  case S_IFDIR:
    return __WASI_FILETYPE_DIRECTORY;
  case S_IFLNK:
    return __WASI_FILETYPE_SYMBOLIC_LINK;
  case S_IFSOCK:
  default:
    __WASI_FILETYPE_UNKNOWN;
  };
}

wavm_ret_int32_t pack_errno(void *dummy, int32_t value)
{
  wavm_ret_int32_t ret;
  ret.dummy = dummy;
  ret.value = value;
  return ret;
}

wavm_ret_int32_t wavm_wasi_unstable_args_sizes_get(void *dummy, int32_t argc_address, int32_t arg_buf_size_address)
{
  (void)dummy;
#ifdef DEBUG
  printf("wavm_wasi_unstable_args_sizes_get\n");
#endif
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
#ifdef DEBUG
  printf("wavm_wasi_unstable_args_get\n");
#endif
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
#ifdef DEBUG
  printf("wavm_wasi_unstable_environ_sizes_get\n");
#endif
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
#ifdef DEBUG
  printf("wavm_wasi_unstable_environ_get\n");
#endif
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

wavm_ret_int32_t wavm_wasi_unstable_clock_res_get(void *dummy, int32_t clock_id, int32_t resolution_address)
{
  (void)dummy;
#ifdef DEBUG
  printf("wavm_wasi_unstable_clock_res_get\n");
#endif
  struct timespec tp;
  if (clock_getres(clock_id, &tp))
  {
    return pack_errno(dummy, __WASI_EINVAL);
  }
  *((uint64_t *)&memoryOffset0.base[resolution_address]) = tp.tv_sec * 1000000000 + tp.tv_nsec;
  return pack_errno(dummy, 0);
}

wavm_ret_int32_t wavm_wasi_unstable_clock_time_get(void *dummy, int32_t clock_id, int64_t precision, int32_t time_address)
{
  (void)dummy;
#ifdef DEBUG
  printf("wavm_wasi_unstable_clock_time_get\n");
#endif
  struct timespec tp;
  if (clock_gettime(clock_id, &tp))
  {
    return pack_errno(dummy, __WASI_EINVAL);
  }
  *((uint64_t *)&memoryOffset0.base[time_address]) = tp.tv_sec * 1000000000 + tp.tv_nsec;
  return pack_errno(dummy, 0);
}

void *wavm_wasi_unstable_fd_advise(void *dummy) {}
void *wavm_wasi_unstable_fd_allocate(void *dummy) {}

wavm_ret_int32_t wavm_wasi_unstable_fd_close(void *dummy, int32_t fd)
{
  (void)dummy;
#ifdef DEBUG
  printf("wavm_wasi_unstable_fd_close fd=%d\n", fd);
#endif
  int32_t r = close(fd);
  if (r != 0)
  {
    return pack_errno(dummy, __WASI_EBADF);
  }
  return pack_errno(dummy, 0);
}

void *wavm_wasi_unstable_fd_datasync(void *dummy) {}

// There is a bug here, when the directory is very large, only a part will be displayed.
wavm_ret_int32_t wavm_wasi_unstable_fd_fdstat_get(void *dummy, int32_t fd, int32_t fdstat_address)
{
  (void)dummy;
#ifdef DEBUG
  printf("wavm_wasi_unstable_fd_fdstat_get fd=%d\n", fd);
#endif
  struct stat fd_stat;
  struct __wasi_fdstat_t fdstat;
  int fl = fcntl(fd, F_GETFL);
  if (fl < 0)
  {
    return pack_errno(dummy, conv_posix_errno_2_wasi_errno(errno));
  }
  fstat(fd, &fd_stat);
  int mode = fd_stat.st_mode;
  fdstat.fs_filetype = (S_ISBLK(mode) ? __WASI_FILETYPE_BLOCK_DEVICE : 0) |
                       (S_ISCHR(mode) ? __WASI_FILETYPE_CHARACTER_DEVICE : 0) |
                       (S_ISDIR(mode) ? __WASI_FILETYPE_DIRECTORY : 0) |
                       (S_ISREG(mode) ? __WASI_FILETYPE_REGULAR_FILE : 0) |
                       (S_ISSOCK(mode) ? __WASI_FILETYPE_SOCKET_STREAM : 0) |
                       (S_ISLNK(mode) ? __WASI_FILETYPE_SYMBOLIC_LINK : 0);
  fdstat.fs_flags = ((fl & O_APPEND) ? __WASI_FDFLAG_APPEND : 0) |
                    ((fl & O_DSYNC) ? __WASI_FDFLAG_DSYNC : 0) |
                    ((fl & O_NONBLOCK) ? __WASI_FDFLAG_NONBLOCK : 0) |
                    ((fl & O_RSYNC) ? __WASI_FDFLAG_RSYNC : 0) |
                    ((fl & O_SYNC) ? __WASI_FDFLAG_SYNC : 0);
  if (fd < 3)
  {
    fdstat.fs_rights_base = STDIO_RIGHTS;
    fdstat.fs_rights_inheriting = 0;
  }
  else if (S_ISREG(mode))
  {
    fdstat.fs_rights_base = REGULAR_FILE_RIGHTS;
    fdstat.fs_rights_inheriting = REGULAR_FILE_RIGHTS;
  }
  else
  {
    fdstat.fs_rights_base = DIRECTORY_RIGHTS;
    fdstat.fs_rights_inheriting = INHERITING_DIRECTORY_RIGHTS;
  }
  *((__wasi_fdstat_t *)&memoryOffset0.base[fdstat_address]) = fdstat;
  return pack_errno(dummy, 0);
}

wavm_ret_int32_t wavm_wasi_unstable_fd_fdstat_set_flags(void *dummy, int32_t fd, int32_t flags)
{
  (void)dummy;
#ifdef DEBUG
  printf("wavm_wasi_unstable_fd_fdstat_set_flags fd=%d flags=%d\n", fd, flags);
#endif
  int fd_flags = ((flags & __WASI_FDFLAG_APPEND) ? O_APPEND : 0) |
                 ((flags & __WASI_FDFLAG_DSYNC) ? O_DSYNC : 0) |
                 ((flags & __WASI_FDFLAG_NONBLOCK) ? O_NONBLOCK : 0) |
                 ((flags & __WASI_FDFLAG_RSYNC) ? O_RSYNC : 0) |
                 ((flags & __WASI_FDFLAG_SYNC) ? O_SYNC : 0);
  if (fcntl(fd, F_SETFL, fd_flags) != 0)
  {
    return pack_errno(dummy, conv_posix_errno_2_wasi_errno(errno));
  }
  return pack_errno(dummy, 0);
}

void *wavm_wasi_unstable_fd_fdstat_set_rights(void *dummy) {}

wavm_ret_int32_t wavm_wasi_unstable_fd_filestat_get(void *dummy, int32_t fd, int32_t filestat_address)
{
  (void)dummy;
#ifdef DEBUG
  printf("wavm_wasi_unstable_fd_filestat_get fd=%d\n", fd);
#endif
  struct stat posix_filestat;
  if (fstat(fd, &posix_filestat))
  {
    return pack_errno(dummy, conv_posix_errno_2_wasi_errno(errno));
  }
  __wasi_filestat_t wasi_filestat;
  wasi_filestat.st_dev = (__wasi_device_t)posix_filestat.st_dev;
  wasi_filestat.st_ino = (__wasi_inode_t)posix_filestat.st_ino;
  wasi_filestat.st_filetype = (__wasi_filetype_t)get_filetype_from_mode(posix_filestat.st_mode);
  wasi_filestat.st_nlink = (__wasi_linkcount_t)posix_filestat.st_nlink;
  wasi_filestat.st_size = (__wasi_filesize_t)posix_filestat.st_size;
  wasi_filestat.st_atim = conv_posix_timespec_2_wasi_timestamp(posix_filestat.st_atim);
  wasi_filestat.st_mtim = conv_posix_timespec_2_wasi_timestamp(posix_filestat.st_mtim);
  wasi_filestat.st_ctim = conv_posix_timespec_2_wasi_timestamp(posix_filestat.st_ctim);
  *((__wasi_filestat_t *)&memoryOffset0.base[filestat_address]) = wasi_filestat;
  return pack_errno(dummy, 0);
}

wavm_ret_int32_t wavm_wasi_unstable_fd_filestat_set_size(void *dummy, int32_t fd, int64_t num_bytes)
{
  (void)dummy;
#ifdef DEBUG
  printf("wavm_wasi_unstable_fd_filestat_set_size fd=%d num_bytes=%ld\n", fd, num_bytes);
#endif
  int result = ftruncate(fd, (off_t)num_bytes);
  if (result != 0)
  {
    return pack_errno(dummy, conv_posix_errno_2_wasi_errno(errno));
  }
  return pack_errno(dummy, 0);
}

wavm_ret_int32_t wavm_wasi_unstable_fd_filestat_set_times(void *dummy, int32_t fd, int64_t last_access_time64, int64_t last_write_time64, int32_t flags)
{
  (void)dummy;
#ifdef DEBUG
  printf("wavm_wasi_unstable_fd_filestat_set_times fd=%d last_access_time64=%ld last_write_time64=%ld flags=%d\n",
         fd, last_access_time64, last_write_time64, flags);
#endif
  struct timespec tp;
  if (clock_gettime(CLOCK_REALTIME, &tp))
  {
    return pack_errno(dummy, __WASI_EINVAL);
  }

  struct timespec timespecs[2];
  if (flags & __WASI_FILESTAT_SET_ATIM)
  {
    timespecs[0].tv_sec = last_access_time64 / 1000000000;
    timespecs[0].tv_nsec = last_access_time64 % 1000000000;
  }
  else if (flags & __WASI_FILESTAT_SET_ATIM_NOW)
  {
    timespecs[0] = tp;
  }
  else
  {
    timespecs[0].tv_nsec = UTIME_OMIT;
  }

  if (flags & __WASI_FILESTAT_SET_MTIM)
  {
    timespecs[1].tv_sec = last_write_time64 / 1000000000;
    timespecs[1].tv_nsec = last_write_time64 % 1000000000;
  }
  else if (flags & __WASI_FILESTAT_SET_MTIM_NOW)
  {
    timespecs[1] = tp;
  }
  else
  {
    timespecs[1].tv_nsec = UTIME_OMIT;
  }
  if (futimens(fd, timespecs) != 0)
  {
    return pack_errno(dummy, conv_posix_errno_2_wasi_errno(errno));
  }
  return pack_errno(dummy, 0);
}

wavm_ret_int32_t wavm_wasi_unstable_fd_pread(void *dummy, int32_t fd, int32_t iovs_address, int32_t num_iovs, int64_t offset, int32_t num_bytes_read_address)
{
  (void)dummy;
#ifdef DEBUG
  printf("wavm_wasi_unstable_fd_pread fd=%d iovs_address=%d num_iovs=%d num_bytes_read_address=%d\n",
         fd, iovs_address, num_iovs, num_bytes_read_address);
#endif
  struct iovec *iovs = copy_iov_to_host(iovs_address, num_iovs);
  size_t ret = preadv(fd, iovs, num_iovs, offset);
  if (ret < 0)
  {
    return pack_errno(dummy, conv_posix_errno_2_wasi_errno(errno));
  }
  *((uint32_t *)&memoryOffset0.base[num_bytes_read_address]) = ret;
  return pack_errno(dummy, 0);
}

wavm_ret_int32_t wavm_wasi_unstable_fd_prestat_get(void *dummy, int32_t fd, int32_t prestat_address)
{
  (void)dummy;
#ifdef DEBUG
  printf("wavm_wasi_unstable_fd_prestat_get fd=%d\n", fd);
#endif
  if (fd < 3 || fd >= PREOPEN_CNT)
  {
    return pack_errno(dummy, __WASI_EBADF);
  }
  *(uint32_t *)&memoryOffset0.base[prestat_address] = __WASI_PREOPENTYPE_DIR;
  *(uint32_t *)&memoryOffset0.base[prestat_address + 4] = preopen[fd].path_len;
  return pack_errno(dummy, 0);
}

wavm_ret_int32_t wavm_wasi_unstable_fd_prestat_dir_name(void *dummy, int32_t fd, int32_t buffer_address, int32_t buffer_length)
{
  (void)dummy;
#ifdef DEBUG
  printf("wavm_wasi_unstable_fd_prestat_dir_name fd=%d\n", fd);
#endif
  if (fd < 3 || fd >= PREOPEN_CNT)
  {
    return pack_errno(dummy, __WASI_EBADF);
  }
  int32_t l = preopen[fd].path_len <= buffer_length ? preopen[fd].path_len : buffer_length;
  memcpy((char *)&memoryOffset0.base[buffer_address], preopen[fd].path, l);
  return pack_errno(dummy, 0);
}

wavm_ret_int32_t wavm_wasi_unstable_fd_pwrite(void *dummy, int32_t fd, int32_t iovs_address, int32_t num_iovs,
                                              int64_t offset, int32_t num_bytes_written_address)
{
  (void)dummy;
#ifdef DEBUG
  printf("wavm_wasi_unstable_fd_pwrite fd=%d num_iovs=%d\n", fd, num_iovs);
#endif
  struct iovec *iovs = copy_iov_to_host(iovs_address, num_iovs);
  ssize_t ret = pwritev(fd, iovs, num_iovs, offset);
  if (ret < 0)
  {
    return pack_errno(dummy, conv_posix_errno_2_wasi_errno(errno));
  }
  *((uint32_t *)&memoryOffset0.base[num_bytes_written_address]) = ret;
  return pack_errno(dummy, 0);
}

wavm_ret_int32_t wavm_wasi_unstable_fd_read(void *dummy, int32_t fd, int32_t iovs_address, int32_t num_iovs, int32_t num_bytes_read_address)
{
  (void)dummy;
#ifdef DEBUG
  printf("wavm_wasi_unstable_fd_read fd=%d iovs_address=%d num_iovs=%d num_bytes_read_address=%d\n",
         fd, iovs_address, num_iovs, num_bytes_read_address);
#endif
  struct iovec *iovs = copy_iov_to_host(iovs_address, num_iovs);
  size_t ret = readv(fd, iovs, num_iovs);
  if (ret < 0)
  {
    return pack_errno(dummy, conv_posix_errno_2_wasi_errno(errno));
  }
  *((uint32_t *)&memoryOffset0.base[num_bytes_read_address]) = ret;
  return pack_errno(dummy, 0);
}

wavm_ret_int32_t wavm_wasi_unstable_fd_readdir(void *dummy, int32_t dir_fd, int32_t buffer_address,
                                               int32_t num_buffer_bytes, int64_t first_cookie,
                                               int32_t out_num_buffer_bytes_used_address)
{
  (void)dummy;
#ifdef DEBUG
  printf("wavm_wasi_unstable_fd_readdir dir_fd=%d buffer_address=%d num_buffer_bytes=%d first_cookie=%ld\n",
         dir_fd, buffer_address, num_buffer_bytes, first_cookie);
#endif
  DIR *dir = fdopendir(dir_fd);
  if (!dir)
  {
    return pack_errno(dummy, conv_posix_errno_2_wasi_errno(errno));
  }
  seekdir(dir, first_cookie);

  struct dirent *dirp;
  __wasi_dirent_t wasi_dirent;
  uint32_t num_buffer_bytes_used = 0;
  while (1)
  {
    dirp = readdir(dir);
    if (dirp == NULL)
    {
      break;
    }

    uint32_t cap_using = sizeof(wasi_dirent) + strlen((*dirp).d_name);
    if (num_buffer_bytes_used + cap_using > num_buffer_bytes)
    {
      break;
    }
    wasi_dirent.d_next = telldir(dir);
    wasi_dirent.d_ino = (*dirp).d_ino;
    wasi_dirent.d_namlen = strlen((*dirp).d_name);
    wasi_dirent.d_type = as_wasi_file_type((*dirp).d_type);

    memcpy(&memoryOffset0.base[buffer_address + num_buffer_bytes_used], &wasi_dirent, sizeof(wasi_dirent));
    num_buffer_bytes_used += sizeof(wasi_dirent);
    memcpy(&memoryOffset0.base[buffer_address + num_buffer_bytes_used], (*dirp).d_name, wasi_dirent.d_namlen);
    num_buffer_bytes_used += wasi_dirent.d_namlen;
  }
  *((uint32_t *)&memoryOffset0.base[out_num_buffer_bytes_used_address]) = num_buffer_bytes_used;
  return pack_errno(dummy, 0);
}

wavm_ret_int32_t wavm_wasi_unstable_fd_renumber(void *dummy, int32_t from_fd, int32_t to_fd)
{
  (void)dummy;
#ifdef DEBUG
  printf("wavm_wasi_unstable_fd_renumber from_fd=%d to_fd=%d\n", from_fd, to_fd);
#endif
  close(to_fd);
  fcntl(from_fd, F_DUPFD);
  return pack_errno(dummy, 0);
}

wavm_ret_int32_t wavm_wasi_unstable_fd_seek(void *dummy, int32_t fd, int64_t offset, int32_t whence, int32_t new_offset_address)
{
  (void)dummy;
#ifdef DEBUG
  printf("wavm_wasi_unstable_fd_seek fd=%d offset=%ld whence=%d\n", fd, offset, whence);
#endif
  int64_t result = lseek(fd, (off_t)offset, whence);
  if (result < 0)
  {
    return pack_errno(dummy, conv_posix_errno_2_wasi_errno(errno));
  }
  *((uint64_t *)&memoryOffset0.base[new_offset_address]) = result;
  return pack_errno(dummy, 0);
}

void *wavm_wasi_unstable_fd_sync(void *dummy) {}
void *wavm_wasi_unstable_fd_tell(void *dummy) {}

wavm_ret_int32_t wavm_wasi_unstable_fd_write(void *dummy, int32_t fd, int32_t iovs_address, int32_t num_iovs,
                                             int32_t num_bytes_written_address)
{
  (void)dummy;
#ifdef DEBUG
  printf("wavm_wasi_unstable_fd_write fd=%d num_iovs=%d\n", fd, num_iovs);
#endif
  struct iovec *iovs = copy_iov_to_host(iovs_address, num_iovs);
  ssize_t ret = writev(fd, iovs, num_iovs);
  if (ret < 0)
  {
    return pack_errno(dummy, conv_posix_errno_2_wasi_errno(errno));
  }
  *((uint32_t *)&memoryOffset0.base[num_bytes_written_address]) = ret;
  return pack_errno(dummy, 0);
}

wavm_ret_int32_t wavm_wasi_unstable_path_create_directory(void *dummy, int32_t dir_fd, int32_t path_address, int32_t num_path_bytes)
{
  (void)dummy;
  char path[MAX_PATH_LENGTH];
  memcpy(path, &memoryOffset0.base[path_address], num_path_bytes);
  path[num_path_bytes] = '\0';
#ifdef DEBUG
  printf("wavm_wasi_unstable_path_create_directory dir_fd=%d path_name=%s\n", dir_fd, path);
#endif
  if (mkdirat(dir_fd, (char *)&memoryOffset0.base[path_address], 0666) != 0)
  {
    return pack_errno(dummy, conv_posix_errno_2_wasi_errno(errno));
  }
  return pack_errno(dummy, 0);
}

wavm_ret_int32_t wavm_wasi_unstable_path_filestat_get(void *dummy, int32_t dir_fd, int32_t lookup_flags,
                                                      int32_t path_address, int32_t num_path_bytes,
                                                      int32_t filestat_address)
{
  (void)dummy;
  char path[MAX_PATH_LENGTH];
  memcpy(path, &memoryOffset0.base[path_address], num_path_bytes);
  path[num_path_bytes] = '\0';
#ifdef DEBUG
  printf("wavm_wasi_unstable_path_filestat_get dir_fd=%d path_name=%s lookup_flags=%d\n", dir_fd, path, lookup_flags);
#endif
  struct stat posix_filestat;
  if (fstatat(dir_fd, path, &posix_filestat, conv_wasi_lookupflags_2_posix_lookupflags(lookup_flags)) != 0)
  {
    return pack_errno(dummy, conv_posix_errno_2_wasi_errno(errno));
  }
  __wasi_filestat_t wasi_filestat;
  wasi_filestat.st_dev = (__wasi_device_t)posix_filestat.st_dev;
  wasi_filestat.st_ino = (__wasi_inode_t)posix_filestat.st_ino;
  wasi_filestat.st_filetype = (__wasi_filetype_t)get_filetype_from_mode(posix_filestat.st_mode);
  wasi_filestat.st_nlink = (__wasi_linkcount_t)posix_filestat.st_nlink;
  wasi_filestat.st_size = (__wasi_filesize_t)posix_filestat.st_size;
  wasi_filestat.st_atim = conv_posix_timespec_2_wasi_timestamp(posix_filestat.st_atim);
  wasi_filestat.st_mtim = conv_posix_timespec_2_wasi_timestamp(posix_filestat.st_mtim);
  wasi_filestat.st_ctim = conv_posix_timespec_2_wasi_timestamp(posix_filestat.st_ctim);
  *((__wasi_filestat_t *)&memoryOffset0.base[filestat_address]) = wasi_filestat;
  return pack_errno(dummy, 0);
}

wavm_ret_int32_t wavm_wasi_unstable_path_filestat_set_times(void *dummy, int32_t dir_fd, int32_t lookup_flags,
                                                            int32_t path_address, int32_t num_path_bytes,
                                                            int64_t last_access_time64, int64_t last_write_time64,
                                                            int32_t flags)
{
  (void)dummy;
  char path[MAX_PATH_LENGTH];
  memcpy(path, &memoryOffset0.base[path_address], num_path_bytes);
  path[num_path_bytes] = '\0';
#ifdef DEBUG
  printf("wavm_wasi_unstable_path_filestat_set_times path=%s\n", path);
#endif
  struct timespec tp;
  if (clock_gettime(CLOCK_REALTIME, &tp))
  {
    return pack_errno(dummy, __WASI_EINVAL);
  }
  struct timespec timespecs[2];
  if (flags & __WASI_FILESTAT_SET_ATIM)
  {
    timespecs[0].tv_sec = last_access_time64 / 1000000000;
    timespecs[0].tv_nsec = last_access_time64 % 1000000000;
  }
  else if (flags & __WASI_FILESTAT_SET_ATIM_NOW)
  {
    timespecs[0] = tp;
  }
  else
  {
    timespecs[0].tv_nsec = UTIME_OMIT;
  }
  if (flags & __WASI_FILESTAT_SET_MTIM)
  {
    timespecs[1].tv_sec = last_write_time64 / 1000000000;
    timespecs[1].tv_nsec = last_write_time64 % 1000000000;
  }
  else if (flags & __WASI_FILESTAT_SET_MTIM_NOW)
  {
    timespecs[1] = tp;
  }
  else
  {
    timespecs[1].tv_nsec = UTIME_OMIT;
  }

  int mode = 0644;
  int host_fd = openat(dir_fd, path, flags, mode);
  if (host_fd < 0)
  {
    return pack_errno(dummy, conv_posix_errno_2_wasi_errno(errno));
  }
  if (futimens(host_fd, timespecs) != 0)
  {
    close(host_fd);
    return pack_errno(dummy, conv_posix_errno_2_wasi_errno(errno));
  }
  else
  {
    close(host_fd);
    return pack_errno(dummy, 0);
  }
}

void *wavm_wasi_unstable_path_link(void *dummy) {}

wavm_ret_int32_t wavm_wasi_unstable_path_open(void *dummy, int32_t dirfd, int32_t dirflags, int32_t path_address,
                                              int32_t num_path_bytes, int32_t open_flags, int64_t requested_rights,
                                              int64_t requested_inheriting_rights, int32_t fd_flags, int32_t fd_address)
{
  (void)dummy;
  char path[MAX_PATH_LENGTH];
  memcpy(path, &memoryOffset0.base[path_address], num_path_bytes);
  path[num_path_bytes] = '\0';
#ifdef DEBUG
  printf("wavm_wasi_unstable_path_open path=%s dirflags=%d open_flags=%d requested_rights=%ld requested_inheriting_rights=%ld fd_flags=%d\n",
         path, dirflags, open_flags, requested_rights, requested_inheriting_rights, fd_flags);
#endif
  int flags = ((open_flags & __WASI_O_CREAT) ? O_CREAT : 0) |
              ((open_flags & __WASI_O_DIRECTORY) ? O_DIRECTORY : 0) |
              ((open_flags & __WASI_O_EXCL) ? O_EXCL : 0) |
              ((open_flags & __WASI_O_TRUNC) ? O_TRUNC : 0) |
              ((fd_flags & __WASI_FDFLAG_APPEND) ? O_APPEND : 0) |
              ((fd_flags & __WASI_FDFLAG_DSYNC) ? O_DSYNC : 0) |
              ((fd_flags & __WASI_FDFLAG_NONBLOCK) ? O_NONBLOCK : 0) |
              ((fd_flags & __WASI_FDFLAG_RSYNC) ? O_RSYNC : 0) |
              ((fd_flags & __WASI_FDFLAG_SYNC) ? O_SYNC : 0);
  if ((requested_rights & __WASI_RIGHT_FD_READ) &&
      (requested_rights & __WASI_RIGHT_FD_WRITE))
  {
    flags |= O_RDWR;
  }
  else if ((requested_rights & __WASI_RIGHT_FD_WRITE))
  {
    flags |= O_WRONLY;
  }
  else if ((requested_rights & __WASI_RIGHT_FD_READ))
  {
    flags |= O_RDONLY;
  }
  int mode = 0644;
  int host_fd = openat(dirfd, path, flags, mode);
  if (host_fd < 0)
  {
    return pack_errno(dummy, conv_posix_errno_2_wasi_errno(errno));
  }
  *((uint32_t *)&memoryOffset0.base[fd_address]) = host_fd;
  return pack_errno(dummy, 0);
}

void *wavm_wasi_unstable_path_readlink(void *dummy) {}

wavm_ret_int32_t wavm_wasi_unstable_path_remove_directory(void *dummy, int32_t dir_fd, int32_t path_address, int32_t num_path_bytes)
{
  (void)dummy;
  char path[MAX_PATH_LENGTH];
  memcpy(path, &memoryOffset0.base[path_address], num_path_bytes);
  path[num_path_bytes] = '\0';
#ifdef DEBUG
  printf("wavm_wasi_unstable_path_remove_directory path_name=%s\n", path);
#endif
  if (unlinkat(dir_fd, path, AT_REMOVEDIR) != 0)
  {
    return pack_errno(dummy, conv_posix_errno_2_wasi_errno(errno));
  }
  return pack_errno(dummy, 0);
}

void *wavm_wasi_unstable_path_rename(void *dummy) {}
void *wavm_wasi_unstable_path_symlink(void *dummy) {}

wavm_ret_int32_t wavm_wasi_unstable_path_unlink_file(void *dummy, int32_t dir_fd, int32_t path_address, int32_t num_path_bytes)
{
  (void)dummy;
  char path[MAX_PATH_LENGTH];
  memcpy(path, &memoryOffset0.base[path_address], num_path_bytes);
  path[num_path_bytes] = '\0';
#ifdef DEBUG
  printf("wavm_wasi_unstable_path_unlink_file path_name=%s\n", path);
#endif
  if (unlinkat(dir_fd, path, 0) != 0)
  {
    return pack_errno(dummy, conv_posix_errno_2_wasi_errno(errno));
  }
  return pack_errno(dummy, 0);
}

void *wavm_wasi_unstable_poll_oneoff(void *dummy) {}
void *wavm_wasi_unstable_proc_exit(void *dummy, int32_t code)
{
#ifdef DEBUG
  printf("wavm_wasi_unstable_proc_exit code=%d\n", code);
#endif
  (void)dummy;
  exit(code);
  return dummy;
}
void *wavm_wasi_unstable_proc_raise(void *dummy) {}
void *wavm_wasi_unstable_sched_yield(void *dummy) {}

wavm_ret_int32_t wavm_wasi_unstable_random_get(void *dummy, int32_t buffer_address, int32_t num_buffer_bytes)
{
#ifdef DEBUG
  printf("wavm_wasi_unstable_random_get buffer_address=%d num_buffer_bytes=%d\n", buffer_address, num_buffer_bytes);
#endif
  (void)dummy;
  for (int32_t i = 0; i < num_buffer_bytes; i++)
  {
    memoryOffset0.base[buffer_address + i] = rand() % 256;
  }
  return pack_errno(dummy, 0);
}

void *wavm_wasi_unstable_sock_recv(void *dummy) {}
void *wavm_wasi_unstable_sock_send(void *dummy) {}
void *wavm_wasi_unstable_sock_shutdown(void *dummy) {}

#endif /* WAVM_POSIX_X86_64_WASI_H */
