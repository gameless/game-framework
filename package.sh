#! /bin/bash
cargo build --release
./package_assets.sh

if [ -f ./target/game-framework.zip ]; then
    rm ./target/game-framework.zip
fi
zip -j ./target/game-framework.zip ./target/release/framework ./target/data.zip
