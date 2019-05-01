# codevs-reborn

# codevs rebornのAI

# 方針
* 連鎖
* スキル使用
* 


* `cargo build --features lambda`
* `cargo build`


# test ai
* cp target/release/test-ai ./ai/rensa_ai_v0.1
* /Users/user/work/src/codevs-reborn/ai/rensa_ai_v0.1
* /Users/user/work/src/codevs-reborn/ai/best_ai_v0.3_fixed
* /Users/user/work/src/codevs-reborn/ai/best_ai_v0.4
* /Users/user/work/src/codevs-reborn/ai/best_ai_v0.7
* /Users/user/work/src/codevs-reborn/ai/best_ai_v0.8
* /Users/user/work/src/codevs-reborn/ai/best_ai_v0.8-40
* /Users/user/work/src/codevs-reborn/target/release/test-ai
* /Users/user/work/src/codevs-reborn/ai/test-ai-wip

RUSTFLAGS='-C overflow-checks=n' cargo run

* pass instance: mHfapR8gvBarpinnrg

* cp /Users/user/work/src/codevs-reborn/target/release/test-ai /Users/user/work/src/codevs-reborn/ai/test-ai-wip

# libraryを分離して、AIとLambdaはライブラリに依存するようにする
* 4bit



# 提出
```bash

cd /mnt/d/d/work/src/codevs_reborn/codevs-reborn

x86_64-unknown-linux-gnu

cargo install xargo

rustup install nightly-x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-gnu

WSLの場合、

rustup install nightly-x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-gnu
rustup install nightly-x86_64-unknown-linux-musl
rustup target add x86_64-unknown-linux-musl

in mac
brew install FiloSottile/musl-cross/musl-cross

cargo build --release

xargo build --target x86_64-unknown-linux-gnu
xargo build --release --target x86_64-unknown-linux-gnu
xargo build --release --target x86_64-unknown-linux-musl
cross build --release --target x86_64-unknown-linux-gnu
cross build --release --target x86_64-unknown-linux-musl

CROSS_COMPILE=x86_64-linux-musl- xargo build --release --target x86_64-unknown-linux-musl

RUSTFLAGS='-C target-feature=+sse,+sse2,+sse3,+sse3,+sse4.1,+sse4.2,+avx,+avx2,+fma -C target-cpu=cortex-a75' xargo build --release --target x86_64-unknown-linux-gnu

RUSTFLAGS = "-C target-feature=+sse -C target-feature=+sse2 -C target-feature=+sse3 -C target-feature=+sse4.1 -C target-feature=+sse4.2 -C target-feature=+popcnt -C target-feature=+lzcnt"


RUSTFLAGS='-C target-feature=+sse,+sse2,+sse3,+sse3,+sse4.1,+sse4.2,+avx,+avx2,+fma -C target-cpu=skylake-avx512' xargo build --release --target x86_64-unknown-linux-gnu
Illegal instruction

RUSTFLAGS='-C target-feature=+sse,+sse2,+sse3,+sse3,+sse4.1,+sse4.2,+avx,+avx2,+fma -C target-cpu=skylake' xargo build --release --target x86_64-unknown-linux-gnu

RUSTFLAGS='-C target-feature=+sse,+sse2,+sse3,+sse3,+sse4.1,+sse4.2,+avx,+avx2,+fma -C target-cpu=skylake' xargo build --release --target x86_64-unknown-linux-musl

RUSTFLAGS='-C target-feature=+sse,+sse2,+sse3,+sse3,+sse4.1,+sse4.2,+avx,+avx2,+fma -C target-cpu=skylake' cross build --release --target x86_64-unknown-linux-musl

ok, ng in mac


cargo clean
RUSTFLAGS='-C target-feature=+sse,+sse2,+sse3,+sse3,+sse4.1,+sse4.2,+avx,+avx2,+fma -C target-cpu=skylake' CROSS_COMPILE=x86_64-linux-musl- cargo build --release --target=x86_64-unknown-linux-musl
ok

# RUSTFLAGS='-C target-feature=+sse,+sse2,+sse3,+sse3,+sse4.1,+sse4.2,+avx,+avx2,+fma -C target-cpu=skylake' CROSS_COMPILE=x86_64-linux-musl- cross build --release --target=x86_64-unknown-linux-musl


target cpu
https://github.com/llvm-mirror/clang/blob/master/test/Frontend/x86-target-cpu.c



### save new ver
cargo clean && cargo build --release
cp ./target/release/test-ai ./ai/best_ai_v0.8
cp ./target/release/test-ai ./ai/best_ai_v0.8-40

### make submit ai
RUSTFLAGS='-C target-feature=+sse,+sse2,+sse3,+sse3,+sse4.1,+sse4.2,+avx,+avx2,+fma -C target-cpu=skylake' CROSS_COMPILE=x86_64-linux-musl- cargo build --release --target=x86_64-unknown-linux-musl
# cp ./target/x86_64-unknown-linux-gnu/release/test-ai ./test-ai
cp ./target/x86_64-unknown-linux-musl/release/test-ai ./test-ai
cp /Users/user/work/src/codevs-reborn/ai/best_ai_v0.3_fixed ./test-ai
rm codevs-reborn.zip & zip codevs-reborn test-ai run.sh

```

