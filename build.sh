TOP=$(pwd)

cd third_party
./build.sh
cd $TOP

cargo build --release
