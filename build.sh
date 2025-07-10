#!/bin/sh
cargo build --release
mkdir -p bin
mv target/release/zsh-histdb-skim bin

cp bin/zsh-histdb-skim "$XDG_DATA_HOME/zsh-histdb-skim/"
