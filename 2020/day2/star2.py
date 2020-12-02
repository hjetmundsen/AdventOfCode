result = 0

for line in open('input1.txt', 'r'):
    ct, ch, pswd = line.split(' ')
    ct = ct.split('-')
    ch = ch[0]
    if (pswd[int(ct[0]) - 1] == ch) ^ (pswd[int(ct[1]) - 1] == ch): result += 1

print(result)
