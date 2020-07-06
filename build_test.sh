./build.sh

cargo test -- --nocapture
python3 tests/test_wasi.py
