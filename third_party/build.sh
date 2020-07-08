TOP=$(pwd)

if [ ! -d "WAVM" ]; then
    git clone --depth=1 --branch master https://github.com/libraries/WAVM/
    cd WAVM
    git checkout 3a07076227663c8c87d10925d3194b1272bf64e7
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
