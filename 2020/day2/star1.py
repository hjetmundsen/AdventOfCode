result = 0

for line in open('input1.txt', 'r'):
    ct, ch, pswd = line.split(' ')
    ct = ct.split('-')
    ch = ch[0]
    count = pswd.count(ch)
    if count >= int(ct[0]) and count <= int(ct[1]): result += 1

print(result)
