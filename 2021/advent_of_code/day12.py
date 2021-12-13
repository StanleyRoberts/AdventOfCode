with open("inputs/day12input.txt", "r") as file:
    edges = [line.strip().split('-') for line in file]

count = 0
def pathgen(word, path, double, skip):
    global count
    if word=='end': count+=1
    else:
        for edge in edges:
            if word in edge:
                new = edge[0] if word==edge[1] else edge[1]
                if (path.count(new)==1 and double==None
                        and new.upper() != new and new!='start') and skip:
                    pathgen(new, path + [new], 1, skip)
                if new not in path or new.upper() == new:
                    pathgen(new, path + [new], double, skip)

pathgen('start', ['start'], None, False)
print(count)
count=0
pathgen('start', ['start'], None, True)
print(count)