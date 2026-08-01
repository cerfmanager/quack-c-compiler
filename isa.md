# QuackQuack ISA Reference

Complete documentation of every opcode in `quackk.c`. This reference reflects the **actual emulator behavior** — discrepancies between documentation and implementation are called out explicitly.

## Instruction format

Every instruction is **6 bytes**, fetched as a fixed-width record:

```
byte:  0    1    2    3    4    5
field: op   ra   b2   b3   b4   b5
```

- **op** — the opcode (see table below)
- **ra** — almost always a register index; exact meaning depends on the instruction
- **b2, b3, b4, b5** — operand bytes. For 16-bit values they hold `lo, hi`. For 32-bit values they hold all four bytes little-endian (`b2`=byte 0 ... `b5`=byte 3).

Non-branching instructions advance `pc` by 6. Branches and calls overwrite `pc` with an absolute 16-bit address built from `b2` (low) and `b3` (high) — no relative addressing.

## Registers and flags

- 6 general-purpose 32-bit registers: `R0`–`R5`. Any instruction touching a register calls `check_reg`, halting if the index is outside `0..5`.
- 4 single-bit flags: `ZF` (zero), `SF` (sign), `OF` (overflow), `CF` (carry). Primarily set by `CMP`; some arithmetic/logical ops also update `ZF` (noted per-instruction).
- `sp` starts at `0x2000` and grows downward as you `PUSHW`.

## Opcode table

| Mnemonic | Opcode | Category |
|---|---|---|
| `rrmovb` / `rrmovw` / `rrmovd` | `0x01` / `0x02` / `0x03` | Register move |
| `irmovb` / `irmovw` / `irmovd` | `0x04` / `0x05` / `0x06` | Immediate load |
| `mrmovb` / `mrmovw` / `mrmovd` | `0x07` / `0x08` / `0x09` | Memory load |
| `rmmovb` / `rmmovw` / `rmmovd` | `0x0A` / `0x0B` / `0x0C` | Memory store |
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
| `pushw` | `0x26` | Stack |
| `popw` | `0x27` | Stack |
| `call` | `0x28` | Stack |
| `ret` | `0x29` | Stack |
| `outc` | `0x2A` | Output |
| `halt` | `0x2B` | Control |
| `neg` | `0x2C` | Unary arithmetic |
| `bitcomp` | `0x2D` | Unary logical |

---

## Register moves

### `RRMOVB`, `RRMOVW`, `RRMOVD` — `0x01` / `0x02` / `0x03`

```asm
rrmovw rSRC, rDST
```

Copies one register into another: `R[ra] = R[b3]`. 
- **ra** is the destination
- **b3** is the source (operand order is reversed from syntax)

**⚠️ Implementation note:** All three variants (`B`/`W`/`D`) execute identically — each moves the *entire* 32-bit register, regardless of suffix. Byte/word size distinction is not implemented.

No flags affected.

---

## Immediate loads

### `IRMOVB` — `0x04`

```asm
irmovb $imm, rDST
```

`R[ra] = b2` (zero-extended). Immediate is a single byte (`0..255`).

### `IRMOVW` — `0x05`

```asm
irmovw $imm, rDST
```

`R[ra] = (b3 << 8) | b2`. 16-bit immediate, zero-extended to 32-bit.

### `IRMOVD` — `0x06`

```asm
irmovd $imm, rDST
```

`R[ra] = b5:b4:b3:b2` (little-endian). Full 32-bit immediate — the only load supporting arbitrary bit patterns including negative numbers as two's-complement.

No flags affected by any `IRMOV*` variant.

---

## Memory loads

### `MRMOVB`, `MRMOVW`, `MRMOVD` — `0x07` / `0x08` / `0x09`

```asm
mrmovb $addr, rDST
mrmovw $addr, rDST
mrmovd $addr, rDST
```

Read from memory into a register: `R[ra] = mem[addr]`. Address comes from `b2`/`b3` (low/high of 16-bit address; `b4`/`b5` present but unused).
- **B** reads 1 byte
- **W** reads 2 bytes (little-endian)
- **D** reads 4 bytes (little-endian)

These variants *do* differ correctly by width, unlike `RRMOV`.

Bounds-checked against `MEM_SIZE` (8192); out-of-range address halts with memory error.

No flags affected.

---

## Memory stores

### `RMMOVB`, `RMMOVW`, `RMMOVD` — `0x0A` / `0x0B` / `0x0C`

```asm
rmmovb $addr, rSRC
rmmovw $addr, rSRC
rmmovd $addr, rSRC
```

Write from register to memory: `mem[addr] = R[ra]`, truncated to 1/2/4 bytes per variant. Same address encoding as `MRMOV`.

No flags affected.

---

## Arithmetic

### `ADD` — `0x0D`

```asm
add rSRC, rDST
```

`R[b2] += R[ra]`. **Destination is the second operand** (not first). Sets `ZF` based on whether destination result is zero.

### `SUB` — `0x0E`

```asm
sub rSRC, rDST
```

`R[b2] -= R[ra]`. Same operand order as `ADD`. Sets `ZF` on destination result.

### `INC` — `0x0F`

```asm
inc rX
```

`R[ra]++`. No flags affected.

### `DEC` — `0x10`

```asm
dec rX
```

`R[ra]--`. No flags affected.

### `CLR` — `0x11`

```asm
clr rX
```

`R[ra] = 0`. No flags affected.

### `NEG` — `0x2C`

```asm
neg rX
```

`R[ra] = -R[ra]` (two's-complement negation). No flags affected.

---

## Logical operations

Two families: register-register (`RAND`/`ROR`/`RXOR`) and register-immediate (`IAND`/`IOR`/`IXOR`).

### `RAND` — `0x12`

```asm
rand rSRC, rDST
```

`R[b2] = R[ra] & R[b2]`. Sets `ZF` based on the **destination** result.

### `ROR` — `0x13`

```asm
ror rSRC, rDST
```

`R[b2] = R[ra] | R[b2]`. Sets `ZF` based on destination result. (Despite the name, this is bitwise OR, not rotate.)

### `RXOR` — `0x14`

```asm
rxor rSRC, rDST
```

`R[b2] = R[ra] ^ R[b2]`. Sets `ZF` based on destination result.

### `IAND` — `0x15`

```asm
iand $imm, rX
```

`R[ra] = R[ra] & imm` (32-bit immediate from `b2..b5`). Sets `ZF` based on destination result.

### `IOR` — `0x16`

```asm
ior $imm, rX
```

`R[ra] = R[ra] | imm`. Sets `ZF` based on destination result.

### `IXOR` — `0x17`

```asm
ixor $imm, rX
```

`R[ra] = R[ra] ^ imm`. Sets `ZF` based on destination result.

### `BITCOMP` — `0x2D`

```asm
bitcomp rX
```

`R[ra] = ~R[ra]` (bitwise complement). No flags affected.

---

## Shifts

All take a 32-bit shift amount from `b2..b5`.

### `SHL` — `0x18`

```asm
shl $amount, rX
```

`R[ra] <<= amount`. Logical left shift, zero-fills from the right.

### `SHR` — `0x19`

```asm
shr $amount, rX
```

`R[ra] >>= amount` as **unsigned** (zero-fills from left). `R[ra]` treated as `uint32_t`.

### `SAR` — `0x1A`

```asm
sar $amount, rX
```

Arithmetic right shift: `R[ra]` reinterpreted as `int32_t` before shifting, preserving the sign bit. Correct two's-complement behavior, distinct from `SHR`.

### `SAL` — `0x1B`

```asm
sal $amount, rX
```

`R[ra] <<= amount`. **Currently identical to `SHL`** — no separate arithmetic-left-shift logic (left shifts don't require sign-extension). Redundant opcode but harmless.

No shift operations affect flags.

---

## Control flow

### `CMP` — `0x1C`

```asm
cmp rA, rB
```

Computes `R[ra] - R[b2]` (unsigned 32-bit subtraction) **for flags only** — result is discarded. Sets:

- `ZF = 1` if values equal
- `SF` = sign bit of subtraction result
- `CF = 1` if `R[ra] < R[b2]` as unsigned (borrow occurred)
- `OF = 1` if subtraction overflowed as signed 32-bit operation

**Always precede conditional jumps with `cmp`.** Nothing else sets all four flags coherently; arithmetic ops only touch `ZF`.

### `JMP` — `0x1D`

```asm
jmp label
```

Unconditional jump: `pc = addr`. No flags read.

### `JE` — `0x1E`

```asm
je label
```

Jump if `ZF == 1` (operands equal after most recent `cmp`).

### `JNE` — `0x1F`

```asm
jne label
```

Jump if `ZF == 0`.

### `JG` — `0x20`

```asm
jg label
```

**Signed** greater-than: jump if `ZF == 0 && SF == OF`. The `OF` correction handles overflow cases (e.g., `INT32_MAX` vs `-1`).

### `JL` — `0x21`

```asm
jl label
```

**Signed** less-than: jump if `SF != OF`.

### `JGE` — `0x22`

```asm
jge label
```

**Signed** greater-than-or-equal: jump if `SF == OF`.

### `JLE` — `0x23`

```asm
jle label
```

**Signed** less-than-or-equal: jump if `ZF == 1 || SF != OF`.

### `JA` — `0x24`

```asm
ja label
```

**Unsigned** above: jump if `CF == 0 && ZF == 0`.

### `JB` — `0x25`

```asm
jb label
```

**Unsigned** below: jump if `CF == 1`.

**Note:** Signed (`JG`/`JL`/`JGE`/`JLE`) and unsigned (`JA`/`JB`) families read the same flags but answer different questions about the bit pattern. No `JAE`/`JBE` (unsigned ≥/≤) defined yet; would need `!CF` and `CF || ZF` respectively.

---

## Stack and procedures

The stack starts at `sp = 0x2000` and grows downward. Operations are **word-based (16-bit values pushed/popped)**, but the implementation actually uses **32-bit operations**. Each push decrements `sp` by 4; each pop increments `sp` by 4.

**⚠️ Important discrepancy:** Documentation claimed word-based (2 bytes), but implementation uses 4-byte moves. This means stack frames are 32-bit, not 16-bit.

### `PUSHW` — `0x26`

```asm
pushw rX
```

```
sp -= 4
mem[sp] = R[ra] (32-bit write)
```

Pushes the full 32-bit register value. `pc` advances normally (+6).

### `POPW` — `0x27`

```asm
popw rX
```

```
R[ra] = mem[sp] (32-bit read)
sp += 4
```

Pops 32 bits into register. Sets `ZF` based on whether popped value was zero. `pc` advances normally (+6).

### `CALL` — `0x28`

```asm
call label
```

```
sp -= 4
mem[sp] = pc + 4  (return address: 16-bit write of next instruction address)
pc = addr
```

**⚠️ Implementation note:** Return address written is `pc + 4`, not `pc + 6`. This is a bug — should be `pc + 6` to point to the instruction after the `call`. Only 16 bits written despite 32-bit stack slots.

### `RET` — `0x29`

```asm
ret
```

```
pc = mem[sp] (16-bit read)
sp += 4
```

Pops return address from stack. No operands. Increments `sp` by 4 even though only 16 bits read — leaves 16-bit gap on stack (mismatched with `CALL`'s 16-bit write).

**⚠️ Stack alignment bug:** `CALL` writes 16 bits; `RET` increments by 4. `PUSHW`/`POPW` use 4-byte operations. This causes stack misalignment. Likely needs refactor to be consistent (either all 16-bit or all 32-bit).

---

## Output

### `OUTC` — `0x2A`

```asm
outc rX
```

**Not yet implemented.** Opcode is reserved and assembler encodes it, but `cpu_step` has no case — executing it calls `die("unknown opcode")`. Intended: print low byte of `R[ra]` as ASCII character.

---

## Halt

### `HALT` — `0x2B`

```asm
halt
```

Sets `cpu->halted = 1`. The run loop stops; `pc` stays pointing at the `HALT` instruction (only opcode that doesn't advance `pc`).

---

## Known issues and gaps

### Bugs/inconsistencies in current implementation

1. **Stack width mismatch:** `PUSHW`/`POPW` use 32-bit operations, but `CALL` writes 16-bit return address and `RET` reads 16-bit. Leaves 16-bit uninitialized gap on stack.
2. **CALL return address off by 2:** Writes `pc + 4` instead of `pc + 6`.
3. **RRMOV* width ignored:** All three variants move full 32-bit register regardless of suffix.
4. **OUTC unimplemented:** Has opcode but no execution.

### Unimplemented features

- **Floating-point:** No float registers or operations.
- **JAE/JBE:** Unsigned ≥/≤ jumps (only have JA, JB, JGE, JLE).
- **Shift by register:** All shifts require immediate 32-bit amount; can't shift by register value.
- **I/O:** `OUTC` reserved but no input mechanism.

---

## Assembly syntax notes

- **Register operands:** `rX` where X ∈ {0..5}
- **Immediates:** `$value` (decimal or hex `0x...`)
- **Memory addresses:** `$addr` (same format as immediates)
- **Labels:** Resolved to 16-bit addresses by assembler
- **Operand order:** Varies by instruction family — always check syntax above
  - Arithmetic/logical: `op rSRC, rDST` (destination is often second)
  - Memory: `op $addr, rX` or `op $imm, rX`
