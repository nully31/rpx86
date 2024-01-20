# RPX86

A toy x86 emulator capable of running a few binary programs written in Rust for studying purposes.
At the time of writing, it only has a minimal amount of basic x86 instructions implemented, and can call a function, input/output some ASCII characters.

I may/may not improve its functionality and/or code structure in future.

## Run

Invoke a binary file inside of the `bin` folder.
If it runs correctly, the program shows the register states as an output.

```
cargo run bin/helloworld.bin
```

If you want to see what each binary does/is compiled from, the source code are inside of `bin/src`.

Note that the binary files only contain the core portion of the code, and newer compilers/assemblers might emit a different code. The binaries in this project are compiled with `gcc 4.9.2` and `nasm 2.11.08`.

## Reference

This is made with the reference of this [book](https://book.mynavi.jp/ec/products/detail/id=41347).