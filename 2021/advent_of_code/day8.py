with open("inputs/day8input.txt", "r") as file:
    display = [line.strip().split("|") for line in file]
display = [[i[0].split(), i[1].split()] for i in display]

s = lambda x: sum(1 for i in display for n in range(len(i[1])) if len(i[1][n]) == x)
print(s(2) + s(4) + s(3) + s(7))

sum = 0
for pairs in display:
    v = lambda x: [pairs[0][n] for n in range(len(pairs[0])) if len(pairs[0][n]) == x][0]
    dict = {1: v(2), 4: v(4), 7: v(3), 8: v(7)}
    for t in pairs[0]:
        if len(t)==6:
            if len(set(t).intersection(dict[1])) != 2: dict[6] = t
            elif len(set(t).intersection(dict[4])) == 4: dict[9] = t
            elif len(set(t).intersection(dict[4].translate({ord(i): None for i in dict[1]}))) != 2: dict[0] = t
        if len(t)==5:
            if len(set(t).intersection(dict[1])) == 2: dict[3] = t
            elif len(set(t).intersection(dict[4].translate({ord(i): None for i in dict[1]}))) == 2: dict[5] = t
            elif len(set(dict[4]).intersection(dict[8].translate({ord(i): None for i in t}))) == 2: dict[2] = t

    num = ""
    for i in pairs[1]:
        for k, v in dict.items():
            if sorted(i)==sorted(v):
                num+=str(k)
    sum += int(num)

print(sum)

