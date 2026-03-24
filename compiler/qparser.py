class Program:
    def __init__(self, functionDefinition):
        self.functionDefinition = functionDefinition


class Return:
    def __init__(self, exp):
        self.expression = exp


class Function:
    def __init__(
        self,
        identifier: str,
        body,
    ):
        self.identifier = identifier
        self.body = body


class Constant:
    def __init__(self, value: int):
        self.value = value


def parseProgram(tokens):
    function, newTokens = parseFunction(tokens)
    if len(newTokens) != 0:
        raise Exception("Syntax error , code found after Program")
    return Program(function)


def parseFunction(tokens):
    newTokens = expect("Int", tokens)
    id, newTokens = expect("Identifier", newTokens)
    newTokens = expect("Open_parenthesis", newTokens)
    newTokens = expect("Close_parenthesis", newTokens)
    newTokens = expect("Open_brace", newTokens)
    body, newTokens = parseStatement(newTokens)
    newTokens = expect("Close_brace", newTokens)
    return Function(id, body), newTokens


def parseStatement(tokens):
    newTokens = expect("Return", tokens)
    return_val, newTokens = parseExp(newTokens)
    newTokens = expect("Semicolon", newTokens)
    return Return(return_val), newTokens


def parseExp(tokens):
    nextToken, innerExp = takeToken(tokens)
    if nextToken.keyword == "Open_parenthesis":
        ExpVal = parseExp(innerExp)
        newTokens = expect("Close_parenthesis", tokens)
        return ExpVal, newTokens
    else:
        newTokens = expect("Constant", tokens)
        return Constant(tokens[0].value), newTokens


def expect(expected, tokens):
    actual, newTokens = takeToken(tokens)
    if actual.keyword != expected:
        raise Exception(f"Syntax error {expected} expected , found {actual.keyword}")
    if expected == "Identifier":
        return actual.value, newTokens
    return newTokens


def takeToken(tokens):
    token = tokens[0]
    tokens = tokens[1:]
    return token, tokens


def prettyPrinter(obj, indent):

    space = "  " * indent  # 2 spaces per level
    match obj:
        case Program(functionDefinition=fd):
            print(f"{space}Program(")
            prettyPrinter(fd, indent + 1)
            print(f"{space})")

        case Function(identifier=name, body=body):
            print(f"{space}Function(")
            print(f"{space}  name={name}")
            print(f"{space}  body=")
            prettyPrinter(body, indent + 2)
            print(f"{space})")

        case Return(expression=expr):
            print(f"{space}Return(")
            prettyPrinter(expr, indent + 1)
            print(f"{space})")

        case Constant(value=v):
            print(f"{space}Constant({v})")

        case _:
            raise TypeError(f"Unknown node type: {type(obj)}")
