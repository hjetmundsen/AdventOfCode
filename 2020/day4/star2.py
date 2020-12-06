def validate_fields(fields):
    eye_colors = set(('amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'))
    hex_chars = set(('a', 'b', 'c', 'd', 'e', 'f'))
    byr = fields['byr']
    iyr = fields['iyr']
    eyr = fields['eyr']
    hgt = fields['hgt']
    hcl = fields['hcl']
    ecl = fields['ecl']
    pid = fields['pid']

    return (
            byr.isdigit() and int(byr) >= 1920 and int(byr) <= 2002 and
            iyr.isdigit() and int(iyr) >= 2010 and int(iyr) <= 2020 and
            eyr.isdigit() and int(eyr) >= 2020 and int(eyr) <= 2030 and
            (
                (hgt[-2:] == 'cm' and hgt[0:-2].isdigit() and int(hgt[0:-2]) >= 150 and int(hgt[0:-2]) <= 193) or
                (hgt[-2:] == 'in' and hgt[0:-2].isdigit() and int(hgt[0:-2]) >= 59 and int(hgt[0:-2]) <= 76)
            ) and
            hcl[0] == '#' and all([(x.isdigit() or x in hex_chars) for x in hcl[1:]]) and
            ecl in eye_colors and
            len(pid) == 9 and pid.isdigit()
           )

def solve():
    required = ['hcl', 'iyr', 'eyr', 'ecl', 'pid', 'byr', 'hgt']
    found = dict()
    num_valid = 0

    for line in open('input1.txt', 'r'):
        line = line.strip()
        if line == '':
            if all([x in found for x in required]) and validate_fields(found): num_valid += 1
            found.clear()
        else:
            for field in line.split(' '): found[field.split(':')[0]] = field.split(':')[1]
    
    return num_valid

print(solve())
