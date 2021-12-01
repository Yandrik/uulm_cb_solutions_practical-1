
class MyBuffer:
    def __init__(self, input : str):
        self._input = input
        self._pos = 0
    
    def peek(self) -> str:
        if self._pos > len(self._input):
            raise 
        return '#' if self._pos == len(self._input) else self._input[self._pos]
    
    def skip(self) -> 'MyBuffer':
        self._pos +=1
        return self
    
    def pop(self) -> str:
        r = self.peek()
        self._pos += 1
        return r
    
    @property
    def input(self) -> str:
        return self._input
    
    @property
    def pos(self) -> int:
        return self._pos

class TRSyntaxException(Exception):
    def __init__(self, input : MyBuffer, expected : str):
        self._input = input.input
        self._pos = input.pos
        self._expected = expected
        super().__init__(self._shortMessage)
    
    @property
    def _shortMessage(self) -> str:
        if self._pos ==len(self._input):
            return "Error: Missing character at end of input, expected: " + self._expected
        else:
            return "Error at column " + str(self._pos) + ": Got '" + self._input[self._pos] + "', expected: " + self._expected

    @property
    def errorString(self) -> str:
        return self._input + "\n" + " " * self._pos + "^\n" + self._shortMessage

def stateS(accum : float, input : MyBuffer) -> float:
    accum = stateE(accum, input)

    if input.peek() == '#':
        return accum
    else:
        raise TRSyntaxException(input, "<EOF>")

def stateE(accum : float, input : MyBuffer) -> float:
    accum = stateP(accum, input)
    return stateE2(accum, input)

def stateE2(accum : float, input : MyBuffer) -> float:
    if input.peek() == "+":
        accum += stateP(accum, input.skip())
        return stateE2(accum, input)
    elif input.peek() == "-":
        accum -= stateP(accum, input.skip())
        return stateE2(accum, input)
    else:
        return accum
    
def stateP(accum : float, input : MyBuffer) -> float:
    accum = stateT(accum, input)
    return stateP2(accum, input)
    
def stateP2(accum : float, input : MyBuffer) -> float:
    if input.peek() == '*':
        accum *= stateT(accum, input.skip())
        return stateP2(accum, input)
    elif input.peek() == '/':
        accum /= stateT(accum, input.skip())
        return stateP2(accum, input)
    else:
        return accum

def stateT(accum : float, input : MyBuffer) -> float:
    if input.peek() == '-':
        return -stateT(accum, input.skip())
    else:
        accum = stateF(accum, input)
        return stateT2(accum, input)

def stateT2(accum : float, input : MyBuffer) -> float:
    if input.peek() == '^':
        return accum ** stateT(accum, input.skip())
    else:
        return accum

def stateF(accum : float, input : MyBuffer) -> float:
    if input.peek() == '(':
        accum = stateE(accum, input.skip())

        if input.peek() != ')':
            raise TRSyntaxException(input, ")")
        else:
            input.skip()
        
        return accum
    else:
        return stateZ(accum, input)

def stateZ(accum : float, input : MyBuffer) -> float:
    if input.peek() != '.' and (input.peek() < '1' or input.peek() > '9'):
        raise TRSyntaxException(input, "<NON-ZERO-DIGIT>")
    
    number = 0
    frac = 0

    while input.peek() >= '0' and input.peek() <= '9':
        number = number * 10 + int(input.pop())
    
    if input.peek() == '.':
        div = 10

        if input.peek() < '0' or input.peek > '9':
            raise TRSyntaxException(input, "<DIGIT>")
        
        while input.peek() >= '0' and input.peek() <= '9':
            frac = frac + int(input.pop()) / div
            div *= 10
    
    return number + frac

def parse(s : str) -> float:
    # Geht auch sauberer via RegEx
    s = s.replace(" ", "").replace("\t", "").replace("\n", "").replace("\r", "")
    return stateS(0, MyBuffer(s))

if __name__ == '__main__':
    import sys

    if len(sys.argv) <= 1:
        print("Missing input", file=sys.stderr)
        exit(1)
    else:
        try:
            print(parse(sys.argv[1]))
        except TRSyntaxException as e:
            print(e.errorString, file=sys.stderr)
            exit(-1)
