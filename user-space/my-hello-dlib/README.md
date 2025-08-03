## Build
```
cargo build
cargo build --release
```
# Testing
```
cargo test
```
# Analysis
* Verify generated .so file using `file`, `objdump`, `readelf` commands
```
file target/release/libsample.so
file target/debug/libsample.so

readelf -h target/release/libsample.so
readelf -h target/debug/libsample.so

objdump -t target/release/libsample.so | grep xadd
objdump -t target/release/libsample.so | grep yadd

```
* Need for `#[no_mangle]`

# Interoperability with C

## Invoke rust functions from C
```
gcc test.c -c
gcc -Ltarget/release test.o -o test -lsimple # link with libsample.so
```