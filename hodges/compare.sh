#!/bin/sh
trials=35
bpm=177
cargo build --release --example compare
for f in audio/jong/*; do
    echo "Res> $f"
    echo "Res> default: "
    ./target/release/examples/compare "$f" $trials $bpm default
    echo "Res> fine: "
    ./target/release/examples/compare "$f" $trials $bpm fine
    echo "Res> rough : "
    ./target/release/examples/compare "$f" $trials $bpm rough
    # add a bit of breathing room
    echo "Res> "
done
