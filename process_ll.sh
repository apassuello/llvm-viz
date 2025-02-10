Copy#!/bin/bash
for file in samples/lua/src/*.ll; do
    echo "Processing $file"
    opt --load-pass-plugin=target/debug/libllvm_viz.so --passes=hello-world -disable-output "$file"
done