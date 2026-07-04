import sys

programBin = bytearray()


hexmap = {
    "rrmovb": 0x01,
    "rrmovw": 0x02,
    "rrmovd": 0x03,
    "irmovb": 0x04,
    "irmovw": 0x05,
    "irmovd": 0x06,
    "mrmovb": 0x07,
    "mrmovw": 0x08,
    "mrmovd": 0x09,
    "rmmovb": 0x0A,
    "rmmovw": 0x0B,
    "rmmovd": 0x0C,
    "add": 0x0D,
    "sub": 0x0E,
    "inc": 0x0F,
    "dec": 0x10,
    "clr": 0x11,
    "rand": 0x12,
    "ror": 0x13,
    "rxor": 0x14,
    "iand": 0x15,
    "ior": 0x16,
    "ixor": 0x17,
    "shl": 0x18,
    "shr": 0x19,
    "sar": 0x1A,
    "sal": 0x1B,
    "cmp": 0x1C,
    "jmp": 0x1D,
    "je": 0x1E,
    "jne": 0x1F,
    "jg": 0x20,
    "jl": 0x21,
    "jge": 0x22,
    "jle": 0x23,
    "ja": 0x24,
    "jb": 0x25,
    "pushw": 0x26,
    "popw": 0x27,
    "call": 0x28,
    "ret": 0x29,
    "outc": 0x2A,
    "halt": 0x2B,
    "neg": 0x2C,
    "bitcomp": 0x2D,
}

# NOTE:OP_OUTC is defined but doesnt work
# opcodes and will assemble correctly here, but quackk.c's cpu_step() switch
# has no case for them yet -- running one currently hits the default
# die("unknown opcode").

# Every instruction is fixed-width: 6 bytes (op, ra, b2, b3, b4, b5).
# This matches instr_t in quackk.c and the pc += 6 used by every opcode

INSTR_WIDTH = 6

jumpMap = {}


def testCLIArgs():
    if len(sys.argv) != 2:
        print("usage:\npython3 quassembler.py <Program.qasm>")
        sys.exit(1)


def reg(tok):
    return int(tok.replace("r", ""))


def imm(tok):
    # base=0 lets you write either decimal ($42) or hex ($0x2A) immediates
    return int(tok.replace("$", ""), 0)


def readASMFile():
    programCounter = 0
    try:
        with open(sys.argv[1], "r") as asm:
            for raw in asm:
                line = raw.strip()
                if not line or line.startswith("#"):
                    continue
                if line.endswith(":"):
                    jumpMap[line[:-1]] = programCounter
                else:
                    programCounter += INSTR_WIDTH

        with open(sys.argv[1], "r") as asm:
            for raw in asm:
                line = raw.strip()
                if not line or line.startswith("#") or line.endswith(":"):
                    continue

                parts = line.replace(",", "").split()
                mnemonic = parts[0]
                if mnemonic not in hexmap:
                    print(f"Unknown mnemonic: {mnemonic}")
                    sys.exit(4)

                op = hexmap[mnemonic]
                ra = b2 = b3 = b4 = b5 = 0x00

                match mnemonic:
                    # r -> r : rSRC rDST
                    case "rrmovb" | "rrmovw" | "rrmovd":
                        b3 = reg(parts[1])
                        ra = reg(parts[2])

                    # byte imm -> r : $val, rDST
                    case "irmovb":
                        value = imm(parts[1])
                        ra = reg(parts[2])
                        b2 = value & 0xFF
                    # word imm -> r : $val, rDST
                    case "irmovw":
                        value = imm(parts[1])
                        ra = reg(parts[2])
                        b2 = value & 0xFF
                        b3 = (value >> 8) & 0xFF
                    # double imm -> r : $val, rDST
                    case "irmovd":
                        value = imm(parts[1])
                        ra = reg(parts[2])
                        b2 = value & 0xFF
                        b3 = (value >> 8) & 0xFF
                        b4 = (value >> 16) & 0xFF
                        b5 = (value >> 24) & 0xFF

                    # mem -> r : $addr, rDST
                    case "mrmovb" | "mrmovw" | "mrmovd":
                        addr = imm(parts[1])
                        ra = reg(parts[2])
                        b2 = addr & 0xFF
                        b3 = (addr >> 8) & 0xFF

                    # r -> mem : $DSTaddr, rSRC
                    case "rmmovb" | "rmmovw" | "rmmovd":
                        addr = imm(parts[1])
                        ra = reg(parts[2])
                        b2 = addr & 0xFF
                        b3 = (addr >> 8) & 0xFF

                    # r -> r : rSRC, rDST
                    # for sub its src - dst
                    case "add" | "sub":
                        ra = reg(parts[1])
                        b2 = reg(parts[2])

                    case "inc" | "dec" | "clr":
                        ra = reg(parts[1])

                    case "rand" | "ror" | "rxor":
                        ra = reg(parts[1])
                        b2 = reg(parts[2])

                    case "iand" | "ior" | "ixor":
                        value = imm(parts[1])
                        ra = reg(parts[2])
                        b2 = value & 0xFF
                        b3 = (value >> 8) & 0xFF
                        b4 = (value >> 16) & 0xFF
                        b5 = (value >> 24) & 0xFF

                    case "neg" | "bitcomp":
                        ra = reg(parts[1])

                    case "shl" | "shr" | "sar" | "sal":
                        amount = imm(parts[1])
                        ra = reg(parts[2])
                        b2 = amount & 0xFF
                        b3 = (amount >> 8) & 0xFF
                        b4 = (amount >> 16) & 0xFF
                        b5 = (amount >> 24) & 0xFF

                    case "cmp":
                        ra = reg(parts[1])
                        b2 = reg(parts[2])

                    case (
                        "jmp" | "je" | "jne" | "jg" | "jl" | "jge" | "jle" | "ja" | "jb"
                    ):
                        addr = jumpMap[parts[1]]
                        b2 = addr & 0xFF
                        b3 = (addr >> 8) & 0xFF

                    case "pushw" | "popw":
                        ra = reg(parts[1])

                    case "call":
                        addr = jumpMap[parts[1]]
                        b2 = addr & 0xFF
                        b3 = (addr >> 8) & 0xFF

                    case "ret" | "halt":
                        pass

                    case "outc":
                        ra = reg(parts[1])

                programBin.extend([op, ra, b2, b3, b4, b5])

    except FileNotFoundError:
        print("Assembly file not found")
        sys.exit(2)

    except IOError:
        print("File could not be read")
        sys.exit(3)


def writeByteToFile():
    try:
        path = sys.argv[1].replace("qasm", "duck")
        with open(path, "wb") as f:
            f.write(programBin)
    except IOError:
        print("File could not be written to")
        sys.exit(3)


if __name__ == "__main__":
    testCLIArgs()
    readASMFile()
    writeByteToFile()
