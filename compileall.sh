#!/bin/bash

cargo build --release --bins -Z unstable-options --out-dir out
cp p4gradingscript.sh out
cd out
