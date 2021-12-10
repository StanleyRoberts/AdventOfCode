with open("inputs/day10input.txt", "r") as file:
    lines = [line.strip() for line in file]

openers = {"[": "]", "{": "}", "(": ")", "<": ">", }
closers = {"]": 57, "}": 1197, ")": 3, ">": 25137}
pts = 0
cpts = []

for line in lines:
    tinput = ""
    for char in line:
        if char in openers:
            tinput = openers[char]+tinput
        else:
            if char == tinput[0]:
                tinput = tinput[1:]
            else:
                pts+=closers[char]
                break
    else:
        score = 0
        for char in tinput:
            match char:
                case ")":
                    val = 1
                case "]":
                    val = 2
                case "}":
                    val = 3
                case ">":
                    val = 4
            score = (score*5) + val
        cpts.append(score)

print(pts)
cpts = sorted(cpts)
print(cpts[int(len(cpts)/2)])
