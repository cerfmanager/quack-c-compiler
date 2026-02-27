import qAvtAsm as qavt

lines = []


def AsmNodesToQuack(Node):
    match Node:
        case qavt.Program():
            AsmNodesToQuack(Node.functionDefinition)
        case qavt.Function():
            signature = f"{Node.identifier}:\n"
            lines.append(signature)
            AsmNodesToQuack(Node.instructions)
        case list():
            for instruction in Node:
                AsmNodesToQuack(instruction)
        case qavt.Mov():
            if isinstance(Node.src, qavt.Imm):
                lines.append(f"    irmovw ${Node.src.int}, {Node.dst.adr}\n")
            else:
                lines.append(f"    rrmovw ${Node.src.adr}, {Node.dst.adr}\n")
        case qavt.Ret():
            lines.append("    halt\n")


def writeTofile(file):
    with open(f"{file}.qasm", "w") as file:
        file.writelines(lines)
