with open("inputs/day6input.txt", "r") as file:
    lf = [int(i) for i in file.read().split(',')]

list = [0]*9
for i in lf:
    list[i]+=1

def rot(list):
    temp = list[0]
    for i in range(8):
        list[i]=list[i+1]
    list[6]+=temp
    list[8]=temp
    return list

for i in range(256):
    list = rot(list)
    if i==79: print(sum(list))

print(sum(list))
