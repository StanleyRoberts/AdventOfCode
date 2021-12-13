with open("inputs/day9input.txt", "r") as file:
    hmap = [[int(char) for char in line.strip()] for line in file]

def basin(i, j, dive=False):
    global lows
    adj = [x for x in [(i, j-1) if j>0 else None,
           (i, j+1) if j<len(hmap[0])-1 else None,
           (i-1, j) if i>0 else None,
           (i+1, j) if i<len(hmap)-1 else None] if x is not None]

    if all([hmap[i][j] < hmap[x][y] for x, y in adj]):
        lows += 1 + hmap[i][j]
        dive = True
    size = {(i, j)}
    if dive:
        for x,y in adj:
            if hmap[x][y] > hmap[i][j] and hmap[x][y]<9:
                size = size.union(basin(x,y, True))
    return size


lows, size = 0,  []
for i in range(len(hmap)):
    for j in range(len(hmap[0])):
        size.append(len(basin(i, j)))

prod=1
for i in sorted(size)[-3:]:
    prod*=i

print(lows)
print(prod)
