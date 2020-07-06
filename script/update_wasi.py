import shutil

try:
    shutil.rmtree('res/wasi')
except:
    pass
shutil.copytree('third_party/WAVM/Test/wasi', 'res/wasi')
