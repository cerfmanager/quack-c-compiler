# Quompiler
![Static Badge](https://img.shields.io/badge/python-blue)
![Static Badge](https://img.shields.io/badge/rust-orange)
# Author
Alexandre Kozlowski DACS Computer Science 

## Description
This is a small c compiler written in rust, it compiles standard C to a cpu emulator that reads DUCK bin files , the compiler turns C into QASM which can be then translated to DUCK by the Quassembler , which can then be read by the cpu emulator

## requirements 

lastest rust version

## Usage

the assembler and compiler are part of a bigger general package, the ability to compile and assemble the file in one pass will arrive in a future version for now use 

```
cargo run -p rust_compiler --release <yourfile>.c
```
to compile 
note that currently changing the name of the output file is not supported since the name of the file doenst change the assembly generation, renaming the file manually doesn't break anything 

```
cargo run -p rust_assembler --release <yourfile>.qasm
```
to assemble 
then compile the quackk emulator using a standard C compiler
this compilation currently only works for unix based machines as stdio and stdlib are linux packages, windows support might will arrive much much later

## Roadmap
  -floating point number support
  -linking
  -binary operators
  -variables
  -branching and loops
  -functions
  -stdio 
  -pointers

and as much of C99 im motivated to add 
## License
will prob add gpl when I have time

## Project status
In developement
