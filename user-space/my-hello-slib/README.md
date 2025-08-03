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
* Verify generated .a file using `file`, `objdump`, `readelf` commands
```
file target/release/libsimple.a
file target/debug/libsimple.a

readelf -h target/release/libsimple.a
readelf -h target/debug/libsimple.a

objdump -t target/release/libsimple.a | grep xadd
objdump -t target/release/libsimple.a | grep yadd

```
* Need for `#[no_mangle]`

# Interoperability with C

## Invoke rust functions from C
```
gcc test.c -c
gcc -Ltarget/release test.o -o test -lsimple
```