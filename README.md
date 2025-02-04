# LLVM VIZ

## Pitfalls

- Be careful no functions are tagged `optnone` !
- Build from sources: Dont forget this flag! `BUILD_SHARED_LIBS`


## Getting Started

1) Compile files using **clang** and generate all *.ll* files
```bash
cd samples/two-sources-files
clang -S -emit-llvm *.c
sed -i 's/optnone//g' *.ll
```
2)  go to top directory and generate json graph from *.ll* files
```bash
cd ../..
opt --load-pass-plugin=target/debug/libllvm_viz.so --passes=hello-world -disable-output samples/two-source-files/main.ll
```
    *Troubleshoot: if the command *opt* is not found, use *opt-XX* - where XX is your LLVM version or create a simlink as follows 
    ```bash
    sudo ln -s /usr/lib/llvm-XX/bin/opt /usr/bin/opt
    ```
3) generate and run the **viz** binary to generate the visual grah
```bash
cargo r --bin viz
```

## Troubleshooting

In case of error from wgpu looking like this:
`The selected version doesn't support Features(DYNAMIC_ARRAY_SIZE)`

Export this:
```
export WGPU_BACKEND=vulkan
```
