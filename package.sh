#! /bin/bash
cargo build --release
./package_assets.sh

rm game-framework.zip
zip -j game-framework.zip ./target/release/framework data.zip
