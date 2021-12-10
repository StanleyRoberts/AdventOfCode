with open("inputs/day1input.txt", "r") as file:
    depths = [int(line.strip()) for line in file]

print(sum([x < y for x, y in zip(depths, depths[1:])]))

slide3 = [x+y+z for x,y,z in zip(depths, depths[1:], depths[2:])]
print(sum([x < y for x, y in zip(slide3, slide3[1:])]))