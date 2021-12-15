from queue import PriorityQueue

def dijkstra(graph):
    pq = PriorityQueue()
    visited = {(0, 0)}
    pq.put((0, (0, 0)))
    while pq:
        dist, node = pq.get()
        (x, y) = node
        if (x, y) == (len(graph)-1, len(graph[0])-1): return dist

        for (a, b) in [(x, y-1), (x, y+1), (x-1, y), (x+1, y)]:
            if 0 <= a < len(graph) and 0 <= b < len(graph[0]) and (a, b) not in visited:
                visited.add((a, b))
                # pq will always grab least node and will never revisit marked node thanks to visited set
                # i.e. equivalent to replacing node priority
                pq.put((dist+graph[a][b], (a,b)))

def extend(vals):
    for i in range(len(vals)):
        temp = []
        for j in range(5):
            temp += [(x + j - 1) % 9 + 1 for x in vals[i]]
        vals[i] = temp

    new = []
    for i in range(5):
        for l in vals:
            new.append([(x + i - 1) % 9 + 1 for x in l])
    return new

with open("inputs/day15input.txt", "r") as file:
    vals = [[int(char) for char in line.strip()] for line in file]

print(dijkstra(vals))
vals = extend(vals)
print(dijkstra(vals))