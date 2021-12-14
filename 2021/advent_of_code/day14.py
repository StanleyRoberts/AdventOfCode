import numpy as np

with open("inputs/day14input.txt", "r") as file:
    poly = [i for i in file.readline().strip()]
    file.readline()
    pins = [[line[0], line[1]] for line in [t.strip().split(' -> ') for t in file]]

def count(val):
    alph = dict([(chr(i), 0) for i in range(65, 91)])

    for letter in alph:
        for i in range(len(val)):
            if letter in pins[i][0]:
                alph[letter] += val[i]
                if pins[i][0][0] == pins[i][0][1]: alph[letter] += val[i]
        if letter == poly[0] or letter == poly[len(poly) - 1]: alph[letter] += 1
        alph[letter] /= 2
    return alph

def iter(num):
    val = np.linalg.matrix_power(mat.T, num).dot(vec)
    counts = {k: v for k, v in count(val).items() if v}.values()
    return int(max(counts) - min(counts))

mat = np.zeros((len(pins), len(pins)))
idxmap = {pins[i][0]: i for i in range(len(list(pins)))}

for div in pins:
    mat[idxmap[div[0]], idxmap[div[0][0]+div[1]]] = 1
    mat[idxmap[div[0]], idxmap[div[1]+div[0][1]]] = 1

vec = np.zeros(len(idxmap))
for i in range(len(vec)-1):
    for j in [poly[k]+poly[k+1] for k in range(len(poly)-1)]:
        if i==idxmap[j]:
            vec[i]+=1

print(iter(10))
print(iter(40))
