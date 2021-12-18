with open("inputs/day17input.txt") as file:
    nums = [int(x) for x in ''.join((char if char in '0123456789-' else ' '
                                     for char in file.read())).split()]
leftx, rightx, downy, upy = min(nums[0], nums[1]), max(nums[0], nums[1]), \
                            min(nums[2], nums[3]), max(nums[2], nums[3])

def checkHits(x, y):
    posx, posy = 0, 0
    while posy>=downy:
        posx+=x
        posy+=y
        x+= 0 if x==0 else (1 if x<0 else -1)
        y-=1
        if leftx <= posx <= rightx and downy <= posy <= upy: return True
    return False

# best possible value y is such that after n steps y=downy and, necessarily, at n-1 steps y=0
# with this we simply derive the maximum y using the triangular number series ((y-1)*y)/2
x, y = 0, abs(downy)-1

# this loop is simply here for posterity, it is not needed on sample input however
# special cases (e.g. target higher than 0,0) require verification of an existing x
while not checkHits(x, y):
    if x>rightx:
        x=0
        y-=1
    else: x+=1

print(int((y*(y+1))/2))

sum = 0
for y in range(int(abs(downy)), downy-1, -1):
    x = 1
    while x<=rightx:
        if checkHits(x, y): sum+=1
        x+=1

print(sum)