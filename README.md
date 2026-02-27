# Quompiler
![Static Badge](https://img.shields.io/badge/python-blue)

# Author
Alexandre Kozlowski DACS Computer Science 

## Description
This is a small c compiler written currently in c , it compiles standard C to a cpu emulator that reads DUCK bin files , the compiler turns C into QASM which can be then translated to DUCK by the Quassembler , which can then be read by the cpu emulator

## Installation
Install all the python files 

## Usage
currently incorrect as it only works for the test.q file and the commands you must run are qlexer.py and quassembler.py ./test.qasm

for windows run 
```
python quompiler.py [name of your file].c
python quassembler.py [same file name].qasm
```
then run it in the duck.c bin

for macos and linux
```
python3 quompiler.py [name of your file].c
python3 quassembler.py [same file name].qasm
```


## Roadmap
I will soon finish the basic compiler for basic instructions, if I still have motivation after that I might rewrite everything in zig for the type safety


## Authors and acknowledgment


## License
For open source projects, say how it is licensed.

## Project status
In developement
