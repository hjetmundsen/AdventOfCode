def solve():
    required = ['hcl', 'iyr', 'eyr', 'ecl', 'pid', 'byr', 'hgt']
    found = set()
    num_valid = 0

    for line in open('input1.txt', 'r'):
        line = line.strip()
        if line == '':
            if all([x in found for x in required]): num_valid += 1
            found.clear()
        else:
            for field in line.split(' '): found.add(field.split(':')[0])
    
    return num_valid

print(solve())
