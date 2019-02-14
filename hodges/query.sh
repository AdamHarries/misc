#!/bin/sh
cargo build --release --example optimise

./target/release/examples/optimise audio/jong/iar.aiff 50 177 | tee dat.csv

cat dat.csv | q -H -d , -b -O "select MeanSqErr, MeanTime, Interval, Steps, Samples from - where MeanSqErr < 10.0 order by MeanTime"