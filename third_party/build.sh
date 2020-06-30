TOP=$(pwd)

if [ ! -d "WAVM" ]; then
    git clone --depth=1 --branch master https://github.com/WAVM/WAVM
    cd WAVM
    git checkout 725da9066f915de26aee8b557fa239eeff8b87be
    mkdir build
    cd build
    cmake .. -DLLVM_DIR=/usr/lib/llvm-9/lib/cmake/llvm
    cmake --build .
    cd $TOP
fi

if [ ! -d "wasi-sdk-11.0" ]; then
    wget https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-11/wasi-sdk-11.0-linux.tar.gz
    tar xvf wasi-sdk-11.0-linux.tar.gz
    ln -s wasi-sdk-11.0 wasi-sdk
    rm wasi-sdk-11.0-linux.tar.gz
fi
