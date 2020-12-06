result = 0
yes = set()

for line in open('input1.txt', 'r'):

    line = line.strip()
    if line == '':
        result += len(yes)
        yes.clear()
    else:
        for i in line: yes.add(i)

print(result)
