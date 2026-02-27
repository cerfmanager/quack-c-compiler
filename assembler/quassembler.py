import sys

programBin = bytearray()

hexmap = {
    "irmovb": 0x02,
    "rrmovw": 0x01,
    "mrmovw": 0x03,
    "rmmovw": 0x04,
    "irmovw": 0x05,
    "mrmovb": 0x06,
    "rmmovb": 0x07,
    "mrmovbr": 0x08,
    "rmmovbr": 0x09,
    "addw": 0x10,
    "subw": 0x11,
    "cmpw": 0x15,
    "jmp": 0x20,
    "je": 0x21,
    "jne": 0x22,
    "halt": 0x23,
    "call": 0x32,
    "ret": 0x33,
    "pushw": 0x30,
    "popw": 0x31,
}

jumpMap = {}


def testCLIArgs():
    if len(sys.argv) != 2:
        print("usage:\npython3 quassembler.py <Program.qasm>")
        sys.exit(1)


def readASMFile():
    programCounter: int = 0
    try:
        with open(sys.argv[1], "r") as asm:
            for raw in asm:
                line = raw.strip()
                if not line or line.startswith("#"):
                    continue
                if line.endswith(":"):
                    jumpMap[line[:-1]] = programCounter

                else:
                    programCounter += 4

        with open(sys.argv[1], "r") as asm:
            for raw in asm:
                line = raw.strip()
                if not line or line.startswith("#") or line.endswith(":"):
                    continue

                parts = line.replace(",", "").split()
                op = hexmap[parts[0]]
                ra = b2 = b3 = 0x00

                match parts[0]:
                    case "irmovw":
                        imm = int(parts[1].replace("$", ""))
                        ra = int(parts[2].replace("r", ""))
                        b2 = imm & 0xFF
                        b3 = (imm >> 8) & 0xFF

                    case "rrmovw":
                        ra = int(parts[1].replace("r", ""))
                        b2 = int(parts[2].replace("r", ""))

                    case "irmovb":
                        imm = int(parts[1].replace("$", ""))
                        ra = int(parts[2].replace("r", ""))
                        b3 = imm & 0xFF

                    case "mrmovw":
                        ra = int(parts[1].replace("r", ""))
                        adrr = int(parts[2].replace("$", ""))
                        b2 = adrr & 0xFF
                        b3 = (adrr >> 8) & 0xFF

                    case "rmmovw":
                        adrr = int(parts[1].replace("$", ""))
                        ra = int(parts[2].replace("r", ""))
                        b2 = adrr & 0xFF
                        b3 = (adrr >> 8) & 0xFF

                    case "irmovb":
                        imm = int(parts[1].replace("$", ""))
                        ra = int(parts[2].replace("r", ""))
                        b3 = imm & 0xFF

                    case "addw":
                        ra = int(parts[1].replace("r", ""))
                        b2 = int(parts[2].replace("r", ""))

                    case "subw":
                        ra = int(parts[1].replace("r", ""))
                        b2 = int(parts[2].replace("r", ""))

                    case "cmpw":
                        ra = int(parts[1].replace("r", ""))
                        b2 = int(parts[2].replace("r", ""))

                    case "popw":
                        ra = int(parts[1].replace("r", ""))

                    case "pushw":
                        ra = int(parts[1].replace("r", ""))

                    case "ret":
                        pass

                    case "call":
                        addr = jumpMap[parts[1]]
                        b2 = addr & 0xFF
                        b3 = (addr >> 8) & 0xFF

                    case "jmp" | "je" | "jne":
                        addr = jumpMap[parts[1]]
                        b2 = addr & 0xFF
                        b3 = (addr >> 8) & 0xFF

                    case "halt":
                        pass
                programBin.extend([op, ra, b2, b3])

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
