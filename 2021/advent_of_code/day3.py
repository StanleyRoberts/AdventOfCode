with open("inputs/day3input.txt", "r") as file:
    diag = [line.strip() for line in file]

gamma= [0] * len(diag[0])

for i in range(0, len(diag[0])):
    gamma[i] = 0 if sum(int(j[i]) for j in diag) < len(diag) / 2 else 1

btd = lambda x: sum(val*(2**i) for i, val in enumerate(reversed(x)))
print(btd(gamma) * btd([0 if i==1 else 1 for i in gamma]))

def filteredlist(list, n, o):
    if len(list) == 1: return list[0]
    inv = 1 - o
    comp = o if sum(int(j[n]) for j in list) < len(list) / 2 else inv
    return filteredlist([val for val in list if int(val[n]) == comp], n+1, o)

oxygen = int(filteredlist(diag, 0, 0), 2)
co2 = int(filteredlist(diag, 0, 1), 2)
print(oxygen*co2)