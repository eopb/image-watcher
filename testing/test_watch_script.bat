:start
cargo build --release
del image-watcher.exe
copy ..\target\release\image-watcher.exe image-watcher.exe
image-watcher.exe -w
timeout 10
goto start