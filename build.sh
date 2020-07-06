TOP=$(pwd)

cd third_party
./build.sh
cd $TOP

cargo build --release

rm -rf build
mkdir build
cp ./target/release/wasc ./build
cp ./third_party/WAVM/build/bin/wavm ./build
