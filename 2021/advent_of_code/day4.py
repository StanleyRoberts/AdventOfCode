with open("inputs/day4input.txt", "r") as file:
    call = [int(x) for x in file.readline().strip().split(",")]
    file.readline()
    boards = [[[int(val) for val in line.split()]
               for line in matrix.split("\n")]
              for matrix in file.read().split("\n\n")]

def win(board, val):
    if board[0][0]==-2: return
    global first, last
    calc = sum([elem for line in board for elem in line if elem!=-1])*val
    board[0][0]=-2
    if first:
        if calc!=0: last = calc
    else: first = calc

first = last = None
for cur in call:
    for board in boards:
        for row in board:
            for val in range(len(row)):
                row[val] = -1 if row[val]==cur else row[val]
            if sum(row) == -5:
                win(board, cur)
        for i in range(len(board[0])):
            if sum([row[i] for row in board])== -5:
                win(board, cur)

print(first)
print(last)