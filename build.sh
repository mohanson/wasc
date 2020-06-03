TOP=$(pwd)

cd third_party
./build.sh
cd $TOP

cargo build --release

rm -rf bin
mkdir bin
cp third_party/WAVM/build/bin/wavm bin/wavm
cp target/release/wasc bin/wasc
cp -R abi bin/abi
