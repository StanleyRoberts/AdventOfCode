with open("inputs/day11input.txt", "r") as file:
    octs = [[int(char) for char in line.strip()] for line in file]


def flash(i, j, flashes):
    if triggered[i][j]==1:
        return flashes
    octs[i][j] += 1
    if octs[i][j] >= 10:
        octs[i][j], triggered[i][j] = 0, 1
        flashes+=1
        for x in [i-1, i, i+1]:
            for y in [j-1, j, j+1]:
                if x >= 0 and y >= 0 and x < len(octs) and y<len(octs[0]) and (x!=i or y!=j):
                    flashes = flash(x, y, flashes)
    return flashes

flashes, step, ctr = 0, 0, 0
while step==0:
    ctr+=1
    triggered = [[0] * len(octs[0]) for t in range(len(octs))]
    for i in range(len(octs)):
        for j in range(len(octs[0])):
            flashes = flash(i, j, flashes)
    if ctr==100:
        fflash = flashes
    if sum(sum(l) for l in triggered)==len(octs)*len(octs[0]):
        step=ctr

print(fflash)
print(step)
