with open("inputs/day16input.txt", "r") as file:
    full = file.read()
    binary = bin(int(full, 16))[2:].zfill(len(full)*4)

def readPacket(string):
    ver, ID, stripped = _getHeader(string)
    if ID == 4: val, stripped = readLiteral(stripped)
    else: val, stripped = readOperator(stripped, ID)
    VERSION[0]+=ver
    return val, stripped

def _getHeader(string):
    ver = int(string[:3], 2)
    ID = int(string[3:6], 2)
    return ver, ID, string[6:]

def readLiteral(string):
    i, lit = 0, ""
    while True:
        lit += string[i + 1:i + 5]
        if string[i]=="0": break
        i+=5
    i+=5
    return int(lit, 2), string[i:]

def _getPacketValues(string):
    vals = []
    if string[0]=="0":
        length = len(string[16:]) - int(string[1:16], 2)
        string = string[16:]
        while len(string)>length:
            val, string = readPacket(string)
            vals.append(val)
    else:
        qty = int(string[1:12], 2)
        string = string[12:]
        for i in range(qty):
            val, string = readPacket(string)
            vals.append(val)
    return vals, string

def readOperator(string, ID):
    vals, stripped = _getPacketValues(string)
    value = 1
    match ID:
        case 0: value = sum(vals)
        case 1:
            for i in vals:
                value *= i
        case 2: value = min(vals)
        case 3: value = max(vals)
        case 5: value = 1 if vals[0] > vals[1] else 0
        case 6: value = 1 if vals[0] < vals[1] else 0
        case 7: value = 1 if vals[0]==vals[1] else 0
    return value, stripped

VERSION = [0]
val, _ = readPacket(binary)
print(VERSION[0])
print(val)