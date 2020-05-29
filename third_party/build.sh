TOP=$(pwd)

git clone --depth=1 --branch master https://github.com/WAVM/WAVM
cd WAVM
git checkout 725da9066f915de26aee8b557fa239eeff8b87be
mkdir build
cd build
cmake .. -DLLVM_DIR=/usr/lib/llvm-9/lib/cmake/llvm
cmake --build .
cd $TOP
