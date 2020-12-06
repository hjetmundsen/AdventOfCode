result = 0
responses = []

for line in open('input1.txt', 'r'):
    line = line.strip()

    if line == '':
        if responses:
            temp = responses[0]
            for r in responses[1:]:
                temp = temp.intersection(r)
            result += len(temp)
            responses.clear()
    else:
        temp = set()
        for i in line: temp.add(i)
        responses.append(temp)

print(result)
