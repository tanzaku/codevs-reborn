# codevs-reborn

# codevs rebornのAI

# 方針
* 連鎖
* スキル使用
* 


* `cargo build --features lambda`
* `cargo build`


# libraryを分離して、AIとLambdaはライブラリに依存するようにする

# 提出
```bash

cd /mnt/d/d/work/src/codevs_reborn/codevs-reborn

x86_64-unknown-linux-gnu

xargo 

xargo build --target x86_64-unknown-linux-gnu
xargo build --release --target x86_64-unknown-linux-gnu

RUSTFLAGS='-C target-feature=+sse,+sse2,+sse3,+sse3,+sse4.1,+sse4.2,+avx,+avx2,+fma -C target-cpu=cortex-a75' xargo build --release --target x86_64-unknown-linux-gnu

RUSTFLAGS = "-C target-feature=+sse -C target-feature=+sse2 -C target-feature=+sse3 -C target-feature=+sse4.1 -C target-feature=+sse4.2 -C target-feature=+popcnt -C target-feature=+lzcnt"


RUSTFLAGS='-C target-feature=+sse,+sse2,+sse3,+sse3,+sse4.1,+sse4.2,+avx,+avx2,+fma -C target-cpu=skylake-avx512' xargo build --release --target x86_64-unknown-linux-gnu
Illegal instruction

RUSTFLAGS='-C target-feature=+sse,+sse2,+sse3,+sse3,+sse4.1,+sse4.2,+avx,+avx2,+fma -C target-cpu=skylake' xargo build --release --target x86_64-unknown-linux-gnu
ok

target cpu
https://github.com/llvm-mirror/clang/blob/master/test/Frontend/x86-target-cpu.c


cp ./target/x86_64-unknown-linux-gnu/release/test-ai ./test-ai
zip codevs-reborn test-ai run.sh
```


