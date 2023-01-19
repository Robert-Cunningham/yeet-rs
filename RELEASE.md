# How to generate a release of yeet-rs

On a linux box:
```
cargo build --release --target=x86_64-unknown-linux-gnu
cp target/x86_64-unknown-linux-gnu/release/yeet yeet-linux-x64
```

On a MacOS box:
```
cargo build --release --target=aarch64-apple-darwin
mv target/aarch64-apple-darwin/release/yeet yeet-darwin-aarch64
cargo build --release --target=x86_64-apple-darwin
mv target/x86_64-apple-darwin/release/yeet yeet-darwin-x64
```