def star1():
    result = 0

    for line in open('input.txt', 'r'):
        ct, ch, pswd = line.split(' ')
        ct = ct.split('-')
        ch = ch[0]
        count = pswd.count(ch)
        if int(ct[0]) <= count <= int(ct[1]):
            result += 1

    return result


def star2():
    result = 0

    for line in open('input.txt', 'r'):
        ct, ch, pswd = line.split(' ')
        ct = ct.split('-')
        ch = ch[0]
        if (pswd[int(ct[0]) - 1] == ch) ^ (pswd[int(ct[1]) - 1] == ch):
            result += 1

    return result


print('STAR 1: {}\nSTAR 2: {}'.format(star1(), star2()))
