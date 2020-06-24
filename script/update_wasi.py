import os.path
import shutil
import glob
import subprocess

wasi_test = 'third_party/WAVM/Test/wasi'

for e in glob.glob(f'{wasi_test}/*.wasm'):
    shutil.copy(e, f'res/wasi')

for e in glob.glob('res/wasi/*.wasm'):
    f = e.replace('wasm', 'wat')
    subprocess.call(f'wasm2wat -o {f} {e}', shell=True)
    subprocess.call(f'sed -i \'s/wasi_snapshot_preview1/wasi_unstable/g\' {f}', shell=True)
    subprocess.call(f'wat2wasm -o {e} {f}', shell=True)
