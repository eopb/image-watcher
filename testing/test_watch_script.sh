cargo build --release
rm image-watcher
cp ../target/release/image-watcher image-watcher
./image-watcher -w