import ast
import math

with open("inputs/day18input.txt") as file:
    equations = [line.strip() for line in file]

def explode(eq):
    neweq=eq.replace(',', ' , ').replace('[', ' [ ').replace(']', ' ] ').split()
    ctr, pot = 0, None
    for i in range(len(neweq)):
        if (char:=neweq[i]).isdigit(): pot=i
        ctr+= 1 if char=="[" else (-1 if char=="]" else 0)
        if ctr==5:
            if pot: neweq[pot] = str(int(neweq[pot]) + int(neweq[i + 1]))
            temp = int(neweq[i + 3])
            neweq[i:i + 5]= "0"
            i+=1
            while i<len(neweq):
                if neweq[i].isdigit():
                    neweq[i] = str(int(neweq[i]) + temp)
                    break
                i+=1
            return True, ''.join(neweq)
    return False, eq

def split(eq):
    neweq=eq.replace(',', ' , ').replace('[', ' [ ').replace(']', ' ] ').split()
    for i in range(len(neweq)):
        if neweq[i].isdigit() and (val:=int(neweq[i]))>9:
            neweq[i] = "[" + str(math.floor(val / 2)) + "," + str(math.ceil(val / 2)) + "]"
            return True, ''.join(neweq)
    return False, eq

def sum(val1, val2):
    return reduce(str("["+str(val1)+","+str(val2)+"]"))

def _reduceOp(eq):
    cont, newstring = explode(eq)
    while cont: cont, newstring = explode(newstring)
    return split(newstring)

def reduce(eq):
    cont=True
    while cont: cont, eq = _reduceOp(eq)
    return eq

def _pairmag(eqval):
    if type(eqval)==int: return eqval
    else: return 3 * _pairmag(eqval[0]) + 2 * _pairmag(eqval[1])

def mag(string):
    eqlist = ast.literal_eval(string.strip().replace(",", ", "))
    return _pairmag(eqlist)

runtot = equations[0]
for eq in equations[1:]:
    runtot = sum(runtot, eq)
print(mag(runtot))

maxmag = 0
for eq1 in equations:
    for eq2 in equations:
        if eq1!=eq2: maxmag = max(maxmag, mag(sum(eq1, eq2)))
print(maxmag)