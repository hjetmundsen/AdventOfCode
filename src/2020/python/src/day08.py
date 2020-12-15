instructions = [line for line in open('../../inputs/day8.txt')]
visited = set()

i = 0
acc = 0

signs = { '+': 1, '-': -1 }

while True:
    if i in visited:
        print(acc)
        break

    visited.add(i)

    code, val = instructions[i].split(' ')
    
    if code == 'nop':
        i += 1
        continue
    if code == 'acc':
        acc += int(val[1:]) * signs[val[0]]
        i += 1
        continue
    else:
        i += int(val[1:]) * signs[val[0]]
        continue

