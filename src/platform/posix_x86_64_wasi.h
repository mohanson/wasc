#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <stddef.h>
#include <stdint.h>
#include <unistd.h>
#include <string.h>
#include <time.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <sys/uio.h>

#ifndef WAVM_POSIX_X86_64_WASI_H
#define WAVM_POSIX_X86_64_WASI_H

#define DEBUG 1

#define WAVM_PAGE_SIZE 0x10000
#ifndef MEMORY0_MAX_PAGE
#define MEMORY0_MAX_PAGE 65536
#endif /* MEMORY0_MAX_PAGE */

#ifdef MEMORY0_DEFINED
int32_t wavm_intrinsic_memory_grow(void *dummy, int32_t grow_by)
{
#ifdef DEBUG
  printf("wavm_intrinsic_memory_grow\n");
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
  memory0 = realloc(memory0, new_size);
  memset(memory0 + old_size, 0, new_size - old_size);
  memoryOffset0.base = memory0;
  memoryOffset0.num_pages = new_pages;
  return old_pages;
}
#else
int32_t wavm_intrinsic_memory_grow(void *dummy, int32_t grow_by)
{
#ifdef DEBUG
  printf("wavm_intrinsic_memory_grow\n");
#endif
  return 1;
}
#endif

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
  printf("wavm_wasi_unstable_args_get\n");
#endif
  exit(1);
}

// WASI syscall API definitions
// Derived from wasi-sysroot:
// https://github.com/CraneStation/wasi-sysroot/blob/320054e84f8f2440def3b1c8700cedb8fd697bf8/libc-bottom-half/headers/public/wasi/core.h
// The wasi-sysroot code is licensed under the terms of the CC0 1.0 Universal license:
// https://github.com/CraneStation/wasi-sysroot/blob/320054e84f8f2440def3b1c8700cedb8fd697bf8/libc-bottom-half/headers/LICENSE
// That license has been mirrored in WASITypes.LICENSE for posterity.
// Thank you to the wasi-sysroot developers for their contributions to the public domain.

typedef int32_t __wasi_intptr_t;
typedef uint32_t __wasi_uintptr_t;
typedef uint32_t __wasi_size_t;
typedef uint32_t __wasi_void_ptr_t;

typedef uint8_t __wasi_advice_t;
#define __WASI_ADVICE_NORMAL (UINT8_C(0))
#define __WASI_ADVICE_SEQUENTIAL (UINT8_C(1))
#define __WASI_ADVICE_RANDOM (UINT8_C(2))
#define __WASI_ADVICE_WILLNEED (UINT8_C(3))
#define __WASI_ADVICE_DONTNEED (UINT8_C(4))
#define __WASI_ADVICE_NOREUSE (UINT8_C(5))

typedef uint32_t __wasi_clockid_t;
#define __WASI_CLOCK_REALTIME (UINT32_C(0))
#define __WASI_CLOCK_MONOTONIC (UINT32_C(1))
#define __WASI_CLOCK_PROCESS_CPUTIME_ID (UINT32_C(2))
#define __WASI_CLOCK_THREAD_CPUTIME_ID (UINT32_C(3))

typedef uint64_t __wasi_device_t;

typedef uint64_t __wasi_dircookie_t;
#define __WASI_DIRCOOKIE_START (UINT64_C(0))

typedef uint32_t __wasi_dirnamlen_t;

typedef uint16_t __wasi_errno_t;
#define __WASI_ESUCCESS (UINT16_C(0))
#define __WASI_E2BIG (UINT16_C(1))
#define __WASI_EACCES (UINT16_C(2))
#define __WASI_EADDRINUSE (UINT16_C(3))
#define __WASI_EADDRNOTAVAIL (UINT16_C(4))
#define __WASI_EAFNOSUPPORT (UINT16_C(5))
#define __WASI_EAGAIN (UINT16_C(6))
#define __WASI_EALREADY (UINT16_C(7))
#define __WASI_EBADF (UINT16_C(8))
#define __WASI_EBADMSG (UINT16_C(9))
#define __WASI_EBUSY (UINT16_C(10))
#define __WASI_ECANCELED (UINT16_C(11))
#define __WASI_ECHILD (UINT16_C(12))
#define __WASI_ECONNABORTED (UINT16_C(13))
#define __WASI_ECONNREFUSED (UINT16_C(14))
#define __WASI_ECONNRESET (UINT16_C(15))
#define __WASI_EDEADLK (UINT16_C(16))
#define __WASI_EDESTADDRREQ (UINT16_C(17))
#define __WASI_EDOM (UINT16_C(18))
#define __WASI_EDQUOT (UINT16_C(19))
#define __WASI_EEXIST (UINT16_C(20))
#define __WASI_EFAULT (UINT16_C(21))
#define __WASI_EFBIG (UINT16_C(22))
#define __WASI_EHOSTUNREACH (UINT16_C(23))
#define __WASI_EIDRM (UINT16_C(24))
#define __WASI_EILSEQ (UINT16_C(25))
#define __WASI_EINPROGRESS (UINT16_C(26))
#define __WASI_EINTR (UINT16_C(27))
#define __WASI_EINVAL (UINT16_C(28))
#define __WASI_EIO (UINT16_C(29))
#define __WASI_EISCONN (UINT16_C(30))
#define __WASI_EISDIR (UINT16_C(31))
#define __WASI_ELOOP (UINT16_C(32))
#define __WASI_EMFILE (UINT16_C(33))
#define __WASI_EMLINK (UINT16_C(34))
#define __WASI_EMSGSIZE (UINT16_C(35))
#define __WASI_EMULTIHOP (UINT16_C(36))
#define __WASI_ENAMETOOLONG (UINT16_C(37))
#define __WASI_ENETDOWN (UINT16_C(38))
#define __WASI_ENETRESET (UINT16_C(39))
#define __WASI_ENETUNREACH (UINT16_C(40))
#define __WASI_ENFILE (UINT16_C(41))
#define __WASI_ENOBUFS (UINT16_C(42))
#define __WASI_ENODEV (UINT16_C(43))
#define __WASI_ENOENT (UINT16_C(44))
#define __WASI_ENOEXEC (UINT16_C(45))
#define __WASI_ENOLCK (UINT16_C(46))
#define __WASI_ENOLINK (UINT16_C(47))
#define __WASI_ENOMEM (UINT16_C(48))
#define __WASI_ENOMSG (UINT16_C(49))
#define __WASI_ENOPROTOOPT (UINT16_C(50))
#define __WASI_ENOSPC (UINT16_C(51))
#define __WASI_ENOSYS (UINT16_C(52))
#define __WASI_ENOTCONN (UINT16_C(53))
#define __WASI_ENOTDIR (UINT16_C(54))
#define __WASI_ENOTEMPTY (UINT16_C(55))
#define __WASI_ENOTRECOVERABLE (UINT16_C(56))
#define __WASI_ENOTSOCK (UINT16_C(57))
#define __WASI_ENOTSUP (UINT16_C(58))
#define __WASI_ENOTTY (UINT16_C(59))
#define __WASI_ENXIO (UINT16_C(60))
#define __WASI_EOVERFLOW (UINT16_C(61))
#define __WASI_EOWNERDEAD (UINT16_C(62))
#define __WASI_EPERM (UINT16_C(63))
#define __WASI_EPIPE (UINT16_C(64))
#define __WASI_EPROTO (UINT16_C(65))
#define __WASI_EPROTONOSUPPORT (UINT16_C(66))
#define __WASI_EPROTOTYPE (UINT16_C(67))
#define __WASI_ERANGE (UINT16_C(68))
#define __WASI_EROFS (UINT16_C(69))
#define __WASI_ESPIPE (UINT16_C(70))
#define __WASI_ESRCH (UINT16_C(71))
#define __WASI_ESTALE (UINT16_C(72))
#define __WASI_ETIMEDOUT (UINT16_C(73))
#define __WASI_ETXTBSY (UINT16_C(74))
#define __WASI_EXDEV (UINT16_C(75))
#define __WASI_ENOTCAPABLE (UINT16_C(76))

typedef uint16_t __wasi_eventrwflags_t;
#define __WASI_EVENT_FD_READWRITE_HANGUP (UINT16_C(0x0001))

typedef uint8_t __wasi_eventtype_t;
#define __WASI_EVENTTYPE_CLOCK (UINT8_C(0))
#define __WASI_EVENTTYPE_FD_READ (UINT8_C(1))
#define __WASI_EVENTTYPE_FD_WRITE (UINT8_C(2))

typedef uint32_t __wasi_exitcode_t;

typedef uint32_t __wasi_fd_t;

typedef uint16_t __wasi_fdflags_t;
#define __WASI_FDFLAG_APPEND (UINT16_C(0x0001))
#define __WASI_FDFLAG_DSYNC (UINT16_C(0x0002))
#define __WASI_FDFLAG_NONBLOCK (UINT16_C(0x0004))
#define __WASI_FDFLAG_RSYNC (UINT16_C(0x0008))
#define __WASI_FDFLAG_SYNC (UINT16_C(0x0010))

typedef int64_t __wasi_filedelta_t;

typedef uint64_t __wasi_filesize_t;

typedef uint8_t __wasi_filetype_t;
#define __WASI_FILETYPE_UNKNOWN (UINT8_C(0))
#define __WASI_FILETYPE_BLOCK_DEVICE (UINT8_C(1))
#define __WASI_FILETYPE_CHARACTER_DEVICE (UINT8_C(2))
#define __WASI_FILETYPE_DIRECTORY (UINT8_C(3))
#define __WASI_FILETYPE_REGULAR_FILE (UINT8_C(4))
#define __WASI_FILETYPE_SOCKET_DGRAM (UINT8_C(5))
#define __WASI_FILETYPE_SOCKET_STREAM (UINT8_C(6))
#define __WASI_FILETYPE_SYMBOLIC_LINK (UINT8_C(7))

typedef uint16_t __wasi_fstflags_t;
#define __WASI_FILESTAT_SET_ATIM (UINT16_C(0x0001))
#define __WASI_FILESTAT_SET_ATIM_NOW (UINT16_C(0x0002))
#define __WASI_FILESTAT_SET_MTIM (UINT16_C(0x0004))
#define __WASI_FILESTAT_SET_MTIM_NOW (UINT16_C(0x0008))

typedef uint64_t __wasi_inode_t;

typedef uint64_t __wasi_linkcount_t;

typedef uint32_t __wasi_lookupflags_t;
#define __WASI_LOOKUP_SYMLINK_FOLLOW (UINT32_C(0x00000001))

typedef uint16_t __wasi_oflags_t;
#define __WASI_O_CREAT (UINT16_C(0x0001))
#define __WASI_O_DIRECTORY (UINT16_C(0x0002))
#define __WASI_O_EXCL (UINT16_C(0x0004))
#define __WASI_O_TRUNC (UINT16_C(0x0008))

typedef uint16_t __wasi_riflags_t;
#define __WASI_SOCK_RECV_PEEK (UINT16_C(0x0001))
#define __WASI_SOCK_RECV_WAITALL (UINT16_C(0x0002))

typedef uint64_t __wasi_rights_t;
#define __WASI_RIGHT_FD_DATASYNC (UINT64_C(0x0000000000000001))
#define __WASI_RIGHT_FD_READ (UINT64_C(0x0000000000000002))
#define __WASI_RIGHT_FD_SEEK (UINT64_C(0x0000000000000004))
#define __WASI_RIGHT_FD_FDSTAT_SET_FLAGS (UINT64_C(0x0000000000000008))
#define __WASI_RIGHT_FD_SYNC (UINT64_C(0x0000000000000010))
#define __WASI_RIGHT_FD_TELL (UINT64_C(0x0000000000000020))
#define __WASI_RIGHT_FD_WRITE (UINT64_C(0x0000000000000040))
#define __WASI_RIGHT_FD_ADVISE (UINT64_C(0x0000000000000080))
#define __WASI_RIGHT_FD_ALLOCATE (UINT64_C(0x0000000000000100))
#define __WASI_RIGHT_PATH_CREATE_DIRECTORY (UINT64_C(0x0000000000000200))
#define __WASI_RIGHT_PATH_CREATE_FILE (UINT64_C(0x0000000000000400))
#define __WASI_RIGHT_PATH_LINK_SOURCE (UINT64_C(0x0000000000000800))
#define __WASI_RIGHT_PATH_LINK_TARGET (UINT64_C(0x0000000000001000))
#define __WASI_RIGHT_PATH_OPEN (UINT64_C(0x0000000000002000))
#define __WASI_RIGHT_FD_READDIR (UINT64_C(0x0000000000004000))
#define __WASI_RIGHT_PATH_READLINK (UINT64_C(0x0000000000008000))
#define __WASI_RIGHT_PATH_RENAME_SOURCE (UINT64_C(0x0000000000010000))
#define __WASI_RIGHT_PATH_RENAME_TARGET (UINT64_C(0x0000000000020000))
#define __WASI_RIGHT_PATH_FILESTAT_GET (UINT64_C(0x0000000000040000))
#define __WASI_RIGHT_PATH_FILESTAT_SET_SIZE (UINT64_C(0x0000000000080000))
#define __WASI_RIGHT_PATH_FILESTAT_SET_TIMES (UINT64_C(0x0000000000100000))
#define __WASI_RIGHT_FD_FILESTAT_GET (UINT64_C(0x0000000000200000))
#define __WASI_RIGHT_FD_FILESTAT_SET_SIZE (UINT64_C(0x0000000000400000))
#define __WASI_RIGHT_FD_FILESTAT_SET_TIMES (UINT64_C(0x0000000000800000))
#define __WASI_RIGHT_PATH_SYMLINK (UINT64_C(0x0000000001000000))
#define __WASI_RIGHT_PATH_REMOVE_DIRECTORY (UINT64_C(0x0000000002000000))
#define __WASI_RIGHT_PATH_UNLINK_FILE (UINT64_C(0x0000000004000000))
#define __WASI_RIGHT_POLL_FD_READWRITE (UINT64_C(0x0000000008000000))
#define __WASI_RIGHT_SOCK_SHUTDOWN (UINT64_C(0x0000000010000000))

typedef uint16_t __wasi_roflags_t;
#define __WASI_SOCK_RECV_DATA_TRUNCATED (UINT16_C(0x0001))

typedef uint8_t __wasi_sdflags_t;
#define __WASI_SHUT_RD (UINT8_C(0x01))
#define __WASI_SHUT_WR (UINT8_C(0x02))

typedef uint16_t __wasi_siflags_t;

typedef uint8_t __wasi_signal_t;
/* UINT8_C(0) is reserved; POSIX has special semantics for kill(pid, 0). */
#define __WASI_SIGHUP (UINT8_C(1))
#define __WASI_SIGINT (UINT8_C(2))
#define __WASI_SIGQUIT (UINT8_C(3))
#define __WASI_SIGILL (UINT8_C(4))
#define __WASI_SIGTRAP (UINT8_C(5))
#define __WASI_SIGABRT (UINT8_C(6))
#define __WASI_SIGBUS (UINT8_C(7))
#define __WASI_SIGFPE (UINT8_C(8))
#define __WASI_SIGKILL (UINT8_C(9))
#define __WASI_SIGUSR1 (UINT8_C(10))
#define __WASI_SIGSEGV (UINT8_C(11))
#define __WASI_SIGUSR2 (UINT8_C(12))
#define __WASI_SIGPIPE (UINT8_C(13))
#define __WASI_SIGALRM (UINT8_C(14))
#define __WASI_SIGTERM (UINT8_C(15))
#define __WASI_SIGCHLD (UINT8_C(16))
#define __WASI_SIGCONT (UINT8_C(17))
#define __WASI_SIGSTOP (UINT8_C(18))
#define __WASI_SIGTSTP (UINT8_C(19))
#define __WASI_SIGTTIN (UINT8_C(20))
#define __WASI_SIGTTOU (UINT8_C(21))
#define __WASI_SIGURG (UINT8_C(22))
#define __WASI_SIGXCPU (UINT8_C(23))
#define __WASI_SIGXFSZ (UINT8_C(24))
#define __WASI_SIGVTALRM (UINT8_C(25))
#define __WASI_SIGPROF (UINT8_C(26))
#define __WASI_SIGWINCH (UINT8_C(27))
#define __WASI_SIGPOLL (UINT8_C(28))
#define __WASI_SIGPWR (UINT8_C(29))
#define __WASI_SIGSYS (UINT8_C(30))

typedef uint16_t __wasi_subclockflags_t;
#define __WASI_SUBSCRIPTION_CLOCK_ABSTIME (UINT16_C(0x0001))

typedef uint64_t __wasi_timestamp_t;

typedef uint64_t __wasi_userdata_t;

typedef uint8_t __wasi_whence_t;
#define __WASI_WHENCE_SET (UINT8_C(0))
#define __WASI_WHENCE_CUR (UINT8_C(1))
#define __WASI_WHENCE_END (UINT8_C(2))

typedef uint8_t __wasi_preopentype_t;
#define __WASI_PREOPENTYPE_DIR (UINT8_C(0))

typedef struct __wasi_dirent_t
{
  __wasi_dircookie_t d_next;
  __wasi_inode_t d_ino;
  __wasi_dirnamlen_t d_namlen;
  __wasi_filetype_t d_type;
} __wasi_dirent_t;

typedef struct __wasi_event_t
{
  __wasi_userdata_t userdata;
  __wasi_errno_t error;
  __wasi_eventtype_t type;
  union __wasi_event_u
  {
    struct __wasi_event_u_fd_readwrite_t
    {
      __wasi_filesize_t nbytes;
      __wasi_eventrwflags_t flags;
    } fd_readwrite;
  } u;
} __wasi_event_t;

typedef struct __wasi_prestat_t
{
  __wasi_preopentype_t pr_type;
  union __wasi_prestat_u
  {
    struct __wasi_prestat_u_dir_t
    {
      __wasi_size_t pr_name_len;
    } dir;
  } u;
} __wasi_prestat_t;

typedef struct __wasi_fdstat_t
{
  __wasi_filetype_t fs_filetype;
  __wasi_fdflags_t fs_flags;
  __wasi_rights_t fs_rights_base;
  __wasi_rights_t fs_rights_inheriting;
} __wasi_fdstat_t;

typedef struct __wasi_filestat_t
{
  __wasi_device_t st_dev;
  __wasi_inode_t st_ino;
  __wasi_filetype_t st_filetype;
  __wasi_linkcount_t st_nlink;
  __wasi_filesize_t st_size;
  __wasi_timestamp_t st_atim;
  __wasi_timestamp_t st_mtim;
  __wasi_timestamp_t st_ctim;
} __wasi_filestat_t;

typedef struct __wasi_ciovec_t
{
  __wasi_void_ptr_t buf;
  __wasi_size_t buf_len;
} __wasi_ciovec_t;

typedef struct __wasi_iovec_t
{
  __wasi_void_ptr_t buf;
  __wasi_size_t buf_len;
} __wasi_iovec_t;

typedef struct __wasi_subscription_t
{
  __wasi_userdata_t userdata;
  __wasi_eventtype_t type;
  union __wasi_subscription_u
  {
    struct __wasi_subscription_u_clock_t
    {
      __wasi_clockid_t clock_id;
      __wasi_timestamp_t timeout;
      __wasi_timestamp_t precision;
      __wasi_subclockflags_t flags;
    } clock;
    struct __wasi_subscription_u_fd_readwrite_t
    {
      __wasi_fd_t fd;
    } fd_readwrite;
  } u;
} __wasi_subscription_t;

// Inspried by https://github.com/kanaka/wac/blob/master/wasi.c#L32
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
  struct iovec *wasi_iov = &memoryOffset0.base[iov_offset];
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

__wasi_errno_t as_wasi_errno(int error)
{
  switch (error)
  {

  case EPERM:
    return __WASI_EPERM;
    break;
  case ENOENT:
    return __WASI_ENOENT;
    break;
  case ESRCH:
    return __WASI_ESRCH;
    break;
  case EINTR:
    return __WASI_EINTR;
    break;
  case EIO:
    return __WASI_EIO;
    break;
  case ENXIO:
    return __WASI_ENXIO;
    break;
  case E2BIG:
    return __WASI_E2BIG;
    break;
  case ENOEXEC:
    return __WASI_ENOEXEC;
    break;
  case EBADF:
    return __WASI_EBADF;
    break;
  case ECHILD:
    return __WASI_ECHILD;
    break;
  case EAGAIN:
    return __WASI_EAGAIN;
    break;
  case ENOMEM:
    return __WASI_ENOMEM;
    break;
  case EACCES:
    return __WASI_EACCES;
    break;
  case EFAULT:
    return __WASI_EFAULT;
    break;
  case EBUSY:
    return __WASI_EBUSY;
    break;
  case EEXIST:
    return __WASI_EEXIST;
    break;
  case EXDEV:
    return __WASI_EXDEV;
    break;
  case ENODEV:
    return __WASI_ENODEV;
    break;
  case ENOTDIR:
    return __WASI_ENOTDIR;
    break;
  case EISDIR:
    return __WASI_EISDIR;
    break;
  case EINVAL:
    return __WASI_EINVAL;
    break;
  case ENFILE:
    return __WASI_ENFILE;
    break;
  case EMFILE:
    return __WASI_EMFILE;
    break;
  case ENOTTY:
    return __WASI_ENOTTY;
    break;
  case ETXTBSY:
    return __WASI_ETXTBSY;
    break;
  case EFBIG:
    return __WASI_EFBIG;
    break;
  case ENOSPC:
    return __WASI_ENOSPC;
    break;
  case ESPIPE:
    return __WASI_ESPIPE;
    break;
  case EROFS:
    return __WASI_EROFS;
    break;
  case EMLINK:
    return __WASI_EMLINK;
    break;
  case EPIPE:
    return __WASI_EPIPE;
    break;
  case EDOM:
    return __WASI_EDOM;
    break;
  case ERANGE:
    return __WASI_ERANGE;
    break;
  default:
    return errno;
  }
}

__wasi_filetype_t as_wasi_file_type(mode_t mode)
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
    return __WASI_FILETYPE_UNKNOWN;
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
  printf("wavm_wasi_unstable_fd_close\n");
#endif
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
  (void)dummy;
#ifdef DEBUG
  printf("wavm_wasi_unstable_fd_fdstat_get fd=%d\n", fd);
#endif
  struct stat fd_stat;
  struct __wasi_fdstat_t fdstat;
  int fl = fcntl(fd, F_GETFL);
  if (fl < 0)
  {
    return pack_errno(dummy, as_wasi_errno(errno));
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
  fdstat.fs_rights_base = (uint64_t)-1;       // all rights
  fdstat.fs_rights_inheriting = (uint64_t)-1; // all rights
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
    return pack_errno(dummy, as_wasi_errno(errno));
  }
  return pack_errno(dummy, 0);
}

void *wavm_wasi_unstable_fd_fdstat_set_rights(void *dummy) {}
void *wavm_wasi_unstable_fd_filestat_get(void *dummy) {}

wavm_ret_int32_t wavm_wasi_unstable_fd_filestat_set_size(void *dummy, int32_t fd, int64_t num_bytes)
{
  (void)dummy;
#ifdef DEBUG
  printf("wavm_wasi_unstable_fd_filestat_set_size fd=%d num_bytes=%ld\n", fd, num_bytes);
#endif
  int result = ftruncate(fd, (off_t)num_bytes);
  if (result != 0)
  {
    return pack_errno(dummy, as_wasi_errno(errno));
  }
  return pack_errno(dummy, 0);
}

void *wavm_wasi_unstable_fd_filestat_set_times(void *dummy) {}
void *wavm_wasi_unstable_fd_pread(void *dummy) {}

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

void *wavm_wasi_unstable_fd_pwrite(void *dummy) {}

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
    return pack_errno(dummy, as_wasi_errno(errno));
  }
  *((uint32_t *)&memoryOffset0.base[num_bytes_read_address]) = ret;
  return pack_errno(dummy, 0);
}

void *wavm_wasi_unstable_fd_readdir(void *dummy) {}
void *wavm_wasi_unstable_fd_renumber(void *dummy) {}

wavm_ret_int32_t wavm_wasi_unstable_fd_seek(void *dummy, int32_t fd, int64_t offset, int32_t whence, int32_t new_offset_address)
{
  (void)dummy;
#ifdef DEBUG
  printf("wavm_wasi_unstable_fd_seek\n");
#endif
  int result = lseek(fd, (off_t)offset, whence);
  if (result == -1)
  {
    if (errno == EINVAL)
    {
      return pack_errno(dummy, __WASI_EINVAL);
    }
    return pack_errno(dummy, as_wasi_errno(errno));
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
    return pack_errno(dummy, as_wasi_errno(errno));
  }
  *((uint32_t *)&memoryOffset0.base[num_bytes_written_address]) = ret;
  return pack_errno(dummy, 0);
}

void *wavm_wasi_unstable_path_create_directory(void *dummy) {}
void *wavm_wasi_unstable_path_filestat_get(void *dummy) {}
void *wavm_wasi_unstable_path_filestat_set_times(void *dummy) {}
void *wavm_wasi_unstable_path_link(void *dummy) {}

wavm_ret_int32_t wavm_wasi_unstable_path_open(void *dummy, int32_t dirfd, int32_t dirflags, int32_t path_address,
                                              int32_t num_path_bytes, int32_t open_flags, int64_t requested_rights,
                                              int64_t requested_inheriting_rights, int32_t fd_flags, int32_t fd_address)
{
  (void)dummy;
  char *path = (char *)&memoryOffset0.base[path_address];
  int32_t *fd = (int32_t *)&memoryOffset0.base[fd_address];
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
    return pack_errno(dummy, as_wasi_errno(errno));
  }
  *((uint32_t *)&memoryOffset0.base[fd_address]) = host_fd;
  return pack_errno(dummy, 0);
}

void *wavm_wasi_unstable_path_readlink(void *dummy) {}
void *wavm_wasi_unstable_path_remove_directory(void *dummy) {}
void *wavm_wasi_unstable_path_rename(void *dummy) {}
void *wavm_wasi_unstable_path_symlink(void *dummy) {}
void *wavm_wasi_unstable_path_unlink_file(void *dummy) {}

void *wavm_wasi_unstable_poll_oneoff(void *dummy) {}
void *wavm_wasi_unstable_proc_exit(void *dummy, int32_t code)
{
#ifdef DEBUG
  printf("wavm_wasi_unstable_proc_exit code=%d\n", code);
#endif
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
