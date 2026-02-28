# Quompiler
![Static Badge](https://img.shields.io/badge/python-blue)

# Author
Alexandre Kozlowski DACS Computer Science 

## Description
This is a small c compiler written currently in python , it compiles standard C to a cpu emulator that reads DUCK bin files , the compiler turns C into QASM which can be then translated to DUCK by the Quassembler , which can then be read by the cpu emulator

## requirements 

python 3.13.5 (this is installed by default on mac and linux and is easily downloadable on the python website for windows)

## Installation
Install all the python files 

## Usage

for windows run 
```
python qompiler.py [name of your file].c
python quassembler.py [same file name].qasm
```
for the assembler you will find your file in the same folder as where the compiler sits so if you dont move it you will have to refence the correct folder 
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
