import qparser as qp


class Program:
    def __init__(self, functionDefinition):
        self.functionDefinition = functionDefinition


class Function:
    def __init__(self, name, instructions):
        self.identifier = name
        self.instructions = instructions


class Mov:
    def __init__(self, src, dst):
        self.src = src
        self.dst = dst


class Ret:
    def __init__(self):
        pass


class Imm:
    def __init__(self, int):
        self.int = int


class Register:
    def __init__(self, adr):
        self.adr = adr


def parseFunction(Node):
    instructions = parseInstructions(Node.body)
    return Function(Node.identifier, instructions)


def parseProgram(Node):
    result = parseFunction(Node.functionDefinition)
    return Program(result)


def parseMov(src, adr):
    return Mov(src, adr)


def parseImm(immVal):
    return Imm(immVal)


def parseRegister(adr):
    return Register(adr)


def parseInstructions(Node):
    instructions = []
    print(type(Node))
    if isinstance(Node, qp.Return):
        print(type(Node.expression))
        if isinstance(Node.expression, qp.Constant):
            immediate = parseImm(Node.expression.value)
            register = parseRegister("r0")
            mov = parseMov(immediate, register)
            returnSt = Ret()
            instructions.append(mov)
            instructions.append(returnSt)

    return instructions


def prettyPrint(obj, indent=0):
    space = "  " * indent
    result = []

    match obj:
        case Program(functionDefinition=fd):
            result.append(f"{space}Program(")
            result.append(prettyPrint(fd, indent + 1))
            result.append(f"{space})")

        case Function(identifier=name, instructions=instrs):
            result.append(f"{space}Function(")
            result.append(f"{space}  name={name}")
            result.append(f"{space}  instructions=[")
            for instr in instrs:
                result.append(prettyPrint(instr, indent + 2))
            result.append(f"{space}  ]")
            result.append(f"{space})")

        case Mov(src=s, dst=d):
            result.append(f"{space}Mov(")
            result.append(prettyPrint(s, indent + 1))
            result.append(f"{space}  ,")
            result.append(prettyPrint(d, indent + 1))
            result.append(f"{space})")

        case Ret():
            result.append(f"{space}Ret()")

        case Imm(int=value):
            result.append(f"{space}Imm({value})")

        case Register(adr=adr):
            result.append(f"{space}Register({adr})")

        case _:
            raise TypeError(f"Unknown node type: {type(obj)}")

    return "\n".join(result) if result else ""
