# QuackQuack ISA Reference

This documents every opcode currently defined in `quackk.c`, what it does, how it's encoded, and how to write it in `quassembler.py` syntax. It reflects the emulator's actual behavior, not just the intent in the comments — a few places where those disagree are called out explicitly.

## Instruction format

Every instruction is **6 bytes**, fetched as a fixed-width record:

```
byte:  0    1    2    3    4    5
field: op   ra   b2   b3   b4   b5
```

- **op** — the opcode (see table below)
- **ra** — almost always a register index; meaning depends on the instruction
- **b2, b3, b4, b5** — operand bytes. For 16-bit values they hold `lo, hi`. For 32-bit values they hold all four bytes little-endian (`b2`=byte 0 ... `b5`=byte 3).

`pc` always advances by 6 after a non-branching instruction. Branches/calls overwrite `pc` directly with an absolute 16-bit address built from `b2` (low) and `b3` (high) — there's no relative addressing.

## Registers and flags

- 6 general-purpose 32-bit registers: `R0`–`R5`. Any instruction touching a register calls `check_reg`, which halts the program if the index is outside `0..5`.
- 4 single-bit flags on the CPU: `ZF` (zero), `SF` (sign), `OF` (overflow), `CF` (carry). `CMP` is the main place these get set deliberately; a few arithmetic/logical ops also update `ZF` as a side effect (noted per-instruction below).
- `sp` starts at `0x2000` and grows downward as you `PUSHW`.

## Opcode table

| Mnemonic | Opcode | Category |
|---|---|---|
| `rrmovb` | `0x01` | Register move |
| `rrmovw` | `0x02` | Register move |
| `rrmovd` | `0x03` | Register move |
| `irmovb` | `0x04` | Immediate load |
| `irmovw` | `0x05` | Immediate load |
| `irmovd` | `0x06` | Immediate load |
| `mrmovb` | `0x07` | Memory load |
| `mrmovw` | `0x08` | Memory load |
| `mrmovd` | `0x09` | Memory load |
| `rmmovb` | `0x0A` | Memory store |
| `rmmovw` | `0x0B` | Memory store |
| `rmmovd` | `0x0C` | Memory store |
| `add` | `0x0D` | Arithmetic |
| `sub` | `0x0E` | Arithmetic |
| `inc` | `0x0F` | Arithmetic |
| `dec` | `0x10` | Arithmetic |
| `clr` | `0x11` | Arithmetic |
| `rand` | `0x12` | Logical |
| `ror` | `0x13` | Logical |
| `rxor` | `0x14` | Logical |
| `iand` | `0x15` | Logical |
| `ior` | `0x16` | Logical |
| `ixor` | `0x17` | Logical |
| `shl` | `0x18` | Shift |
| `shr` | `0x19` | Shift |
| `sar` | `0x1A` | Shift |
| `sal` | `0x1B` | Shift |
| `cmp` | `0x1C` | Control flow |
| `jmp` | `0x1D` | Control flow |
| `je` | `0x1E` | Control flow |
| `jne` | `0x1F` | Control flow |
| `jg` | `0x20` | Control flow |
| `jl` | `0x21` | Control flow |
| `jge` | `0x22` | Control flow |
| `jle` | `0x23` | Control flow |
| `ja` | `0x24` | Control flow |
| `jb` | `0x25` | Control flow |
| `pushw` | `0x26` | Stack/procedure |
| `popw` | `0x27` | Stack/procedure |
| `call` | `0x28` | Stack/procedure |
| `ret` | `0x29` | Stack/procedure |
| `outc` | `0x2A` | Output |
| `halt` | `0x2B` | Control |

---

## Register moves

### `RRMOVB`, `RRMOVW`, `RRMOVD` — `0x01` / `0x02` / `0x03`

```asm
rrmovw rSRC, rDST
```

Copies one register's value into another: `R[ra] = R[b3]`. `ra` is the **destination**, `b3` is the **source** — the operand order is reversed from how you'd guess by reading left to right in the byte layout.

**Current behavior to be aware of:** all three variants (`B`/`W`/`D`) execute identically — each one moves the *entire* 32-bit register, regardless of suffix. The byte/word size distinction that `IRMOV`/`MRMOV`/`RMMOV` honor isn't implemented here yet.

No flags affected.

---

## Immediate loads

### `IRMOVB` — `0x04`

```asm
irmovb $imm, rDST
```

`R[ra] = b2` (zero-extended). Immediate is a single byte, so values are truncated to `0..255`.

### `IRMOVW` — `0x05`

```asm
irmovw $imm, rDST
```

`R[ra] = (b3 << 8) | b2`. 16-bit immediate, zero-extended into the 32-bit register.

### `IRMOVD` — `0x06`

```asm
irmovd $imm, rDST
```

`R[ra] = b5:b4:b3:b2` (little-endian). Full 32-bit immediate — the only load that can put an arbitrary 32-bit pattern (including negative numbers, as their two's-complement bit pattern) directly into a register.

No flags affected by any `IRMOV*` variant.

---

## Memory loads

### `MRMOVB`, `MRMOVW`, `MRMOVD` — `0x07` / `0x08` / `0x09`

```asm
mrmovb $addr, rDST
mrmovw $addr, rDST
mrmovd $addr, rDST
```

Reads from memory into a register: `R[ra] = mem[addr]`, where `addr` comes from `b2`/`b3` (low/high byte of a 16-bit address — `b4`/`b5` are present in the encoding but get truncated away since memory addresses are 16-bit). `B` reads 1 byte, `W` reads 2 bytes (little-endian), `D` reads 4 bytes (little-endian) — these *do* differ correctly by width, unlike `RRMOV`.

Bounds-checked against `MEM_SIZE` (8192); an out-of-range address halts the program with a memory error.

No flags affected.

---

## Memory stores

### `RMMOVB`, `RMMOVW`, `RMMOVD` — `0x0A` / `0x0B` / `0x0C`

```asm
rmmovb $addr, rSRC
rmmovw $addr, rSRC
rmmovd $addr, rSRC
```

Writes a register out to memory: `mem[addr] = R[ra]`, truncated to 1/2/4 bytes depending on variant. Same address encoding as the `MRMOV` family.

No flags affected.

---

## Arithmetic

### `ADD` (`add`) — `0x0D`

```asm
add rSRC, rDST
```

`R[b2] += R[ra]`. Note the destination is the **second** operand, not the first. Sets `ZF` based on whether the destination ended up zero.

### `SUB` (`sub`) — `0x0E`

```asm
subw rSRC, rDST
```

`R[b2] -= R[ra]`. Same operand order as `ADD`. Sets `ZF` on the destination's result.

### `INC` (`inc`) — `0x0F`

```asm
incw rX
```

`R[ra]++`. No flags affected.

### `DEC` (`dec`) — `0x10`

```asm
decw rX
```

`R[ra]--`. No flags affected.

### `CLR` (`clrw`) — `0x11`

```asm
clrw rX
```

`R[ra] = 0`. No flags affected.

---

## Logical operations

There are two families here: register-register (`RAND`/`ROR`/`RXOR`) and register-immediate (`IAND`/`IOR`/`IXOR`).

### `RAND` (`rand`) — `0x12`

```asm
rand rSRC, rDST
```

`R[b2] = R[ra] & R[b2]`. Sets `ZF`, but based on `R[ra]` (the source), not the destination that actually changed — worth knowing if you're relying on the flag.

### `ROR` (`ror`) — `0x13`

```asm
ror rSRC, rDST
```

`R[b2] = R[ra] | R[b2]`. Same `ZF`-on-source quirk as `RAND`. (Despite the name, this is bitwise OR, not "rotate".)

### `RXOR` (`rxor`) — `0x14`

```asm
rxor rSRC, rDST
```

`R[b2] = R[ra] ^ R[b2]`. Same `ZF`-on-source quirk.

### `IAND` (`iand`) — `0x15`

```asm
iand $imm, rX
```

`R[ra] = R[ra] & imm`, where `imm` is the full 32-bit value from `b2..b5`. Sets `ZF` based on the (correct, in this case) destination register `ra`.

### `IOR` (`ior`) — `0x16`

```asm
ior $imm, rX
```

`R[ra] = R[ra] | imm`. Sets `ZF` on `ra`.

### `IXOR` (`ixor`) — `0x17`

```asm
ixor $imm, rX
```

`R[ra] = R[ra] ^ imm`. Sets `ZF` on `ra`.

---

## Shifts

All four take a 32-bit shift amount from `b2..b5`, same encoding as the immediate logical ops.

### `SHL` (`shl`) — `0x18`

```asm
shl $amount, rX
```

`R[ra] <<= amount`. Logical left shift, zero-fills from the right.

### `SHR` (`shr`) — `0x19`

```asm
shr $amount, rX
```

`R[ra] >>= amount`, as an **unsigned** shift (zero-fills from the left) — `R[ra]` is treated as `uint32_t` here, so this is correct for unsigned values but will not sign-extend negative ones.

### `SAR` (`sar`) — `0x1A`

```asm
sar $amount, rX
```

Arithmetic right shift: the register is reinterpreted as `int32_t` before shifting, so the sign bit is preserved (sign-extends from the left). Correct two's-complement arithmetic shift, distinct from `SHR`.

### `SAL` (`sal`) — `0x1B`

```asm
sal $amount, rX
```

`R[ra] <<= amount`. **Currently identical to `SHL`** — there's no separate arithmetic-left-shift logic (left shifts don't need sign-extension the way right shifts do, so as written this isn't a bug so much as a redundant opcode, but it's worth knowing `SAL` and `SHL` aren't doing anything different from each other yet).

None of the shift ops affect any flags.

---

## Control flow

### `CMP` (`cmp`) — `0x1C`

```asm
cmp rA, rB
```

Computes `R[ra] - R[b2]` (as unsigned 32-bit subtraction) purely to set flags — it doesn't write the result anywhere. This is the instruction every conditional jump below depends on:

- `ZF` = 1 if the values are equal
- `SF` = the sign bit of the subtraction result
- `CF` = 1 if `R[ra] < R[b2]` as **unsigned** values (i.e. a borrow occurred)
- `OF` = 1 if the subtraction overflowed as a **signed** 32-bit operation

Always run `cmpw` immediately before the conditional jump that depends on it — nothing else sets these four flags together as a coherent group (though see the side-effect notes on `ADD`/`SUB`/`IAND` etc. above, which only ever touch `ZF`).

### `JMP` (`jmp`) — `0x1D`

```asm
jmp label
```

Unconditional: `pc = addr`. No flags read.

### `JE` (`je`) — `0x1E`

```asm
je label
```

Jumps if `ZF == 1` (i.e. the last `cmpw` found the two operands equal).

### `JNE` (`jne`) — `0x1F`

```asm
jne label
```

Jumps if `ZF == 0`.

### `JG` (`jg`) — `0x20`

```asm
jg label
```

**Signed** "greater than": jumps if `ZF == 0 && SF == OF`. This is why `CMP` needs `OF` — a naive sign-bit check breaks across signed overflow (e.g. comparing `INT32_MAX` against `-1`), and `SF == OF` corrects for that.

### `JL` (`jl`) — `0x21`

```asm
jl label
```

**Signed** "less than": jumps if `SF != OF`.

### `JGE` (`jge`) — `0x22`

```asm
jge label
```

**Signed** "greater than or equal": jumps if `SF == OF`.

### `JLE` (`jle`) — `0x23`

```asm
jle label
```

**Signed** "less than or equal": jumps if `ZF == 1 || SF != OF`.

### `JA` (`ja`) — `0x24`

```asm
ja label
```

**Unsigned** "above": jumps if `CF == 0 && ZF == 0`.

### `JB` (`jb`) — `0x25`

```asm
jb label
```

**Unsigned** "below": jumps if `CF == 1`.

> The signed (`JG`/`JL`/`JGE`/`JLE`) and unsigned (`JA`/`JB`) families read the same flags from the same `cmpw`, but answer different questions about the same bit pattern — see `SF`/`OF` vs `CF` above. There's no `JAE`/`JBE` (unsigned ≥/≤) defined yet, even though the flag logic for them is a one-line addition (`!CF` and `CF || ZF` respectively) if you need them.

---

## Stack and procedures

The stack starts at `sp = 0x2000` and grows downward (each push decrements `sp` by 2, each pop increments it by 2). It's word-based (16-bit) — pushing/popping always moves exactly 2 bytes, regardless of register width.

### `PUSHW` (`pushw`) — `0x26`

```asm
pushw rX
```

`sp -= 2; mem[sp] = R[ra]` (low 16 bits only — pushing a register with a value above `0xFFFF` will truncate it on the way to the stack).

### `POPW` (`popw`) — `0x27`

```asm
popw rX
```

`R[ra] = mem[sp]; sp += 2`. Also sets `ZF` based on whether the popped value was zero.

### `CALL` (`call`) — `0x28`

```asm
call label
```

Pushes the return address (`pc + 6`, i.e. the instruction right after this `call`) onto the stack, then jumps: `pc = addr`.

### `RET` (`ret`) — `0x29`

```asm
ret
```

Pops a 16-bit address off the stack into `pc`. No operands.

---

## Output

### `OUTC` (`outc`) — `0x2A`

```asm
outc rX
```

**Not yet implemented.** The opcode is reserved and the assembler will happily encode it, but `cpu_step`'s switch has no case for it — executing it currently hits `die("unknown opcode")`. Intended behavior (per the `#define` comment) is to print the low byte of `R[ra]` as an ASCII character.

---

## Halt

### `HALT` (`halt`) — `0x2B`

```asm
halt
```

Sets `cpu->halted = 1`. The run loop stops calling `cpu_step` once this is set; `pc` is left pointing at the `HALT` instruction itself (it's the only opcode that doesn't advance `pc`).

---

## Known gaps, for when you get to them

- `OUTC` has no execution logic yet (see above).
- No `JAE`/`JBE` (unsigned ≥/≤), even though `JGE`/`JLE` exist for the signed side.
