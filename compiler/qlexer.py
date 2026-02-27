import re

import qAvtAsm
import qparser

tokens = []

regexes = {
    "Int": r"int\b",
    "Void": r"void\b",
    "Return": r"return\b",
    "Open_parenthesis": r"\(",
    "Close_parenthesis": r"\)",
    "Open_brace": r"\{",
    "Close_brace": r"\}",
    "Semicolon": r";",
    "Constant": r"[0-9]+\b",
    "Identifier": r"[a-zA-Z_]\w*\b",
    "SKIP": r"\s+",
}


master_pattern = re.compile(
    "|".join(f"(?P<{name}>{pattern})" for name, pattern in regexes.items())
)


class Token:
    def __init__(self, keyword, value=None):
        self.keyword = keyword
        self.value = value


with open("test.q") as file:
    code = file.read()

for match in master_pattern.finditer(code):
    token_type = match.lastgroup
    value = match.group()

    if token_type == "SKIP":
        continue
    elif token_type == "Constant":
        tokens.append(Token("Constant", int(value)))
    elif token_type == "Identifier":
        tokens.append(Token("Identifier", value))
    else:
        tokens.append(Token(token_type))

if not tokens:
    raise Exception("No tokens found in this file")

for thingy in tokens:
    print(f"{thingy.keyword} {thingy.value}")


AVT = qparser.parseProgram(tokens)
AVTASM = qAvtAsm.parseProgram(AVT)
print(qAvtAsm.prettyPrint(AVTASM))
