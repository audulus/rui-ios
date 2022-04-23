# rui-ios
Demo of rui embedded on iOS

To run:

1. `rustup target add aarch64-apple-ios x86_64-apple-ios`
1. `cargo install cargo-lipo`
1. `cargo lipo`
1. Open Xcode project
1. Run

re-generate the rui-ios.h with

`cbindgen --config cbindgen.toml --crate rui-ios --output rui-ios.h`