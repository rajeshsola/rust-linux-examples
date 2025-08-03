## Build
```
cargo build            # dev build, unoptimized + debuginfo
cargo build --release  # release build, optimized and no debuginfo
RUSTC=rustc-1.80 cargo  build # build with different version of rustc
```
## Run
```
cargo run
```
# Analysis
* Verify generated executable using `file` command
```
file target/debug/myhello
file target/release/myhello
```
# Cross Compiling

## In .cargo/config.toml
```
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabi-gcc"
```
## Add the target and cross compile for the target
```
rustup target list
rustup target add armv7-unknown-linux-gnueabihf
cargo build --release --target=armv7-unknown-linux-gnueabihf
```


