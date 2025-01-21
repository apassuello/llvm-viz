Basic example compiling 2 files ensuring we get the graph if compiling multiple
files at the same time.

```bash
clang -S -emit-llvm *.c
sed -i 's/optnone//g' *.ll
cd ../..
opt --load-pass-plugin=target/debug/libllvm_viz.so --passes=hello-world -disable-output samples/two-source-files/main.ll
```
