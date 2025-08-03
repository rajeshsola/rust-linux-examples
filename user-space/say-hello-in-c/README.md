> Invoking C functions from RUST

# Invoking from static library
* In `slibs` subdir
```
gcc hello.c -c
ar rcs libhello.a hello.o
```
* In build.rs
```
fn main() {
    println!("cargo:rustc-link-search=native=./slibs");
    println!("cargo:rustc-link-lib=static=hello");
}
```
* Build rust code using cargo

# Invoking from dynamic library
* In `dlibs` subdir
```
gcc hello.c -c
gcc -shared -o libhello.so hello.o
```
* In build.rs
```
fn main() {
    println!("cargo:rustc-link-search=native=./dlibs");
    println!("cargo:rustc-link-lib=dylib=hello");
}
```
* Build rust code using cargo




