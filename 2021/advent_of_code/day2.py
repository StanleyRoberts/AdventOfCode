with open("inputs/day2input.txt", "r") as file:
    instruct = [[line.strip().split(" ")[0], int(line.strip().split(" ")[1])] for line in file]

x_pos = depth = 0
for i in instruct:
    if i[0][0] == 'f':
        x_pos+=i[1]
    if i[0][0] == 'd':
        depth+=i[1]
    if i[0][0] == 'u':
        depth-=i[1]

print(x_pos*depth)

x_pos = depth = aim = 0
for i in instruct:
    if i[0][0] == 'f':
        x_pos+=i[1]
        depth+=aim*i[1]
    if i[0][0] == 'd':
        aim+=i[1]
    if i[0][0] == 'u':
        aim-=i[1]

print(x_pos*depth)