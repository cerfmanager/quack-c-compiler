import sys

import qAvtAsm
import qlexer
import qodeEmitter
import qparser


def testCLIArgs():
    if len(sys.argv) != 2:
        print("usage:\npython3 quassembler.py <Program.qasm>")
        sys.exit(1)


def compile():
    filePath = sys.argv[1]

    newPath = filePath.replace(
        ".c", ""
    )  # adding this so when I write to file it makes it the right format

    qlexer.lex(filePath)  # lex the code into tokens
    avt = qparser.parseProgram(qlexer.tokens)  # parse the tokens into a AVT
    avtasm = qAvtAsm.parseProgram(avt)  # parse that tree into an assembly value tree
    qodeEmitter.AsmNodesToQuack(avtasm)  # get the assembly lines from the tree
    qodeEmitter.writeTofile(newPath)  # put the lines in the final file


if __name__ == "__main__":
    testCLIArgs()
    compile()
