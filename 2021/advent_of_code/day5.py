with open("inputs/day5input.txt", "r") as file:
    lines = [[tuple(int(i) for i in x.split(',')), tuple(int(j) for j in y.split(','))]
             for x, y in [line.strip().split(" -> ") for line in file]]

cross = max(val for line in lines for pair in line for val in pair)+1

def inter(diag=False):
    plane = [[0] * cross for t in range(cross)]
    for line in lines:
        p1, p2 = line[0], line[1]
        miny, maxy = min(p1[1], p2[1]), max(p1[1], p2[1])
        if p1[0] == p2[0]:
            for y in range(miny, maxy+1): plane[p1[0]][y] += 1
        minx, maxx = min(p1[0], p2[0]), max(p1[0], p2[0])
        if p1[1] == p2[1]:
            for x in range(minx, maxx+1): plane[x][p1[1]] += 1
        if diag:
            if maxy-miny == (diff:=maxx-minx):
                for i in range(diff+1):
                    if p1[0]<p2[0] and p1[1]>p2[1]: plane[p1[0]+i][p1[1]-i] += 1
                    elif p1[0]>p2[0] and p1[1]<p2[1]: plane[p1[0]-i][p1[1]+i] += 1
                    else: plane[minx+i][miny+i] += 1
    count = 0
    for row in plane:
        for ele in row:
            if ele>=2: count+=1
    return count

print(inter())
print(inter(True))