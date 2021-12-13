import matplotlib.pyplot as plt

with open("inputs/day13input.txt", "r") as file:
    sdots, sfolds = file.read().strip().split('\n\n')
    dots = [[int(i) for i in line.split(',')] for line in ''.join(sdots).split('\n')]
    folds = [[pair for pair in last.split()[2].split('=')] for last in ''.join(sfolds).split('\n')]

for pair in folds:
    axis, val = pair[0], int(pair[1])
    for point in dots:
        if axis=='x' and point[0] > val:
            point[0] -= 2*(point[0]-val)
        elif axis=='y' and point[1] > val:
            point[1] -= 2*(point[1]-val)
    if pair==folds[0]:
        print(len(set([tuple(i) for i in dots])))

plt.scatter(*zip(*dots))
plt.gca().invert_yaxis()
plt.show()