import subprocess
import os
import os.path


def test_append():
    print('test_append')
    subprocess.getoutput('rm /tmp/a')
    subprocess.getoutput('res/posix_x86_64_wasi/append /tmp/a')
    r = subprocess.getoutput('cat /tmp/a')
    assert(r == 'Hello World!')
    subprocess.getoutput('res/posix_x86_64_wasi/append /tmp/a')
    r = subprocess.getoutput('cat /tmp/a')
    assert(r == 'Hello World!\nHello World!')
    subprocess.getoutput('rm /tmp/a')


def test_args():
    print('test_args')
    r = subprocess.getoutput('res/posix_x86_64_wasi/args 1 2 3').split('\n')
    assert(r[0] == 'argc=4')
    assert(r[2] == 'argv[1]: 1')
    assert(r[3] == 'argv[2]: 2')
    assert(r[4] == 'argv[3]: 3')
    assert(r[5] == 'argv[4]: <null>')
    r = subprocess.getoutput('res/posix_x86_64_wasi/args').split('\n')
    assert(r[0] == 'argc=1')


def test_cat():
    print('test_cat')
    subprocess.getoutput('echo Hello World! > /tmp/a')
    r = subprocess.getoutput('res/posix_x86_64_wasi/cat /tmp/a')
    assert(r == 'Hello World!')
    r = subprocess.getoutput('res/posix_x86_64_wasi/cat /tmp/b')
    assert(r == 'Failed to open \'/tmp/b\' for reading: No such file or directory')
    subprocess.getoutput('rm /tmp/a /tmp/b')


def test_clock():
    print('test_clock')
    r = subprocess.getoutput('res/posix_x86_64_wasi/clock').split('\n')
    assert(r[0].startswith('CLOCK_REALTIME'))
    assert(r[1].startswith('CLOCK_MONOTONIC'))
    assert(r[2].startswith('CLOCK_PROCESS_CPUTIME_ID'))
    assert(r[3].startswith('CLOCK_THREAD_CPUTIME_ID'))


def test_exit():
    print('test_exit')
    r, _ = subprocess.getstatusoutput('res/posix_x86_64_wasi/exit')
    assert(r == 0)


def test_fd_filestat_set_size():
    print('test_fd_filestat_set_size')
    subprocess.getoutput('res/posix_x86_64_wasi/fd_filestat_set_size /tmp/a')
    r = os.path.getsize('/tmp/a')
    assert(r == 4201)
    subprocess.getoutput('rm /tmp/a')


def test_fd_filestat_set_times():
    print('tset_fd_filestat_set_times')
    subprocess.getoutput('echo Hello World! > /tmp/a')
    subprocess.getoutput('res/posix_x86_64_wasi/fd_filestat_set_times /tmp/a 100 100')
    r = subprocess.getoutput('stat /tmp/a').split('\n')
    assert(r[4] == 'Access: 1970-01-01 08:00:00.000000100 +0800')
    assert(r[5] == 'Modify: 1970-01-01 08:00:00.000000100 +0800')
    subprocess.getoutput('rm /tmp/a')


def test_fd_renumber():
    print('test_fd_renumber')
    subprocess.getoutput('res/posix_x86_64_wasi/fd_renumber /tmp/a')
    r = subprocess.getoutput('cat /tmp/a')
    assert(r == 'Hello stdout!\nHello file!')
    subprocess.getoutput('rm /tmp/a')


def test_largefile():
    print('test_largefile')
    r = subprocess.getoutput('res/posix_x86_64_wasi/largefile /tmp/a').split('\n')
    assert(r[0] == 'pread(3GB): Hello 3GB!')
    assert(r[1] == 'pread(6GB): Hello 6GB!')
    assert(r[2] == 'pread(9GB): Hello 9GB!')
    r = os.path.getsize('/tmp/a')
    assert(r == 10 * 1024 * 1024 * 1024)
    subprocess.getoutput('rm /tmp/a')


def test_ls():
    print('test_ls')
    subprocess.getoutput('res/posix_x86_64_wasi/ls /tmp/').split('\n')


def test_mkdir():
    print('test_mkdir')
    subprocess.getoutput('res/posix_x86_64_wasi/mkdir /tmp/a')
    assert(os.path.exists('/tmp/a'))
    subprocess.getoutput('res/posix_x86_64_wasi/mkdir /tmp/a/b')
    assert(os.path.exists('/tmp/a/b'))
    subprocess.getoutput('rm -rf /tmp/a')


def test_path_filestat_set_times():
    print('test_path_filestat_set_times')
    subprocess.getoutput('echo Hello World! > /tmp/a')
    subprocess.getoutput('res/posix_x86_64_wasi/fd_filestat_set_times /tmp/a 100 100')
    r = subprocess.getoutput('stat /tmp/a').split('\n')
    assert(r[4] == 'Access: 1970-01-01 08:00:00.000000100 +0800')
    assert(r[5] == 'Modify: 1970-01-01 08:00:00.000000100 +0800')
    subprocess.getoutput('rm /tmp/a')


def test_preadwrite():
    print('test_preadwrite')
    r = subprocess.getoutput('res/posix_x86_64_wasi/preadwrite /tmp/a').split('\n')
    assert(r[0] == 'pread(5000): Hello 5000!')
    assert(r[1] == 'pread(500): Hello 500!')
    subprocess.getoutput('rm /tmp/a')


def test_random():
    print('test_random')
    r = subprocess.getoutput('res/posix_x86_64_wasi/random')
    assert(r.startswith('67c6697351ff'))
    assert(len(r) == 1024 * 2)


def test_rm():
    print('test_rm')
    subprocess.getoutput('echo Hello World! > /tmp/a')
    subprocess.getoutput('res/posix_x86_64_wasi/rm /tmp/a')
    assert(not os.path.exists('/tmp/a'))


def test_stat():
    print('test_stat')
    subprocess.getoutput('echo Hello World! > /tmp/a')
    subprocess.getoutput('res/posix_x86_64_wasi/stat /tmp/a')
    subprocess.getoutput('rm /tmp/a')


def test_stdout():
    print('test_stdout')
    r = subprocess.getoutput('res/posix_x86_64_wasi/stdout')
    assert(r == 'Hello world!')


def test_write():
    print('test_write')
    subprocess.getoutput('res/posix_x86_64_wasi/write /tmp/a')
    r = subprocess.getoutput('cat /tmp/a')
    assert(r == 'Hello World!')
    subprocess.getoutput('rm /tmp/a')


test_append()
test_args()
test_cat()
test_clock()
test_exit()
test_fd_filestat_set_size()
test_fd_filestat_set_times()
test_fd_renumber()
test_largefile()
test_ls()
test_mkdir()
test_path_filestat_set_times()
test_preadwrite()
test_random()
test_rm()
test_stat()
test_stdout()
test_write()
