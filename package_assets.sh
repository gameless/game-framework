#! /bin/bash
if [ -f ./target/data.zip ]; then
    rm ./target/data.zip
fi
zip ./target/data.zip assets/*

