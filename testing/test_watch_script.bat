:start
cargo build --release
del image-watcher.exe
copy ..\target\release\image-watcher.exe image-watcher.exe
set RUST_BACKTRACE=1
image-watcher.exe -w
timeout 10
goto start