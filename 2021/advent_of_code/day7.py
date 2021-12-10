with open("inputs/day7input.txt", "r") as file:
    crabs = [int(i) for i in file.read().split(',')]

crabs.sort()
median = crabs[int(len(crabs)/2)]
print(sum(abs(i-median) for i in crabs))

mean = int(sum(crabs)/len(crabs))
val = sum((i*(i+1))/2 for i in map(lambda x: abs(x-mean), crabs))
print(int(val))
