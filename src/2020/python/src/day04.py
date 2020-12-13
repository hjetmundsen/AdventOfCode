def _validate_fields(fields, counter):
    eye_colors = {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"}
    hex_chars = {"a", "b", "c", "d", "e", "f"}
    byr = fields["byr"]
    iyr = fields["iyr"]
    eyr = fields["eyr"]
    hgt = fields["hgt"]
    hcl = fields["hcl"]
    ecl = fields["ecl"]
    pid = fields["pid"]

    if not (
        byr.isdigit()
        and 1920 <= int(byr) <= 2002
        and iyr.isdigit()
        and 2010 <= int(iyr) <= 2020
        and eyr.isdigit()
        and 2020 <= int(eyr) <= 2030
        and (
            (hgt[-2:] == "cm" and hgt[0:-2].isdigit() and 150 <= int(hgt[0:-2]) <= 193)
            or (hgt[-2:] == "in" and hgt[0:-2].isdigit() and 59 <= int(hgt[0:-2]) <= 76)
        )
        and hcl[0] == "#"
        and all([(x.isdigit() or x in hex_chars) for x in hcl[1:]])
        and ecl in eye_colors
        and len(pid) == 9
        and pid.isdigit()
    ):
        print(counter)

    return (
        byr.isdigit()
        and 1920 <= int(byr) <= 2002
        and iyr.isdigit()
        and 2010 <= int(iyr) <= 2020
        and eyr.isdigit()
        and 2020 <= int(eyr) <= 2030
        and (
            (hgt[-2:] == "cm" and hgt[0:-2].isdigit() and 150 <= int(hgt[0:-2]) <= 193)
            or (hgt[-2:] == "in" and hgt[0:-2].isdigit() and 59 <= int(hgt[0:-2]) <= 76)
        )
        and hcl[0] == "#"
        and all([(x.isdigit() or x in hex_chars) for x in hcl[1:]])
        and ecl in eye_colors
        and len(pid) == 9
        and pid.isdigit()
    )


def star1():
    required = ["hcl", "iyr", "eyr", "ecl", "pid", "byr", "hgt"]
    found = set()
    num_valid = 0

    for line in open("../../inputs/day4.txt", "r"):
        line = line.strip()
        if line == "":
            if all([x in found for x in required]):
                num_valid += 1
            found.clear()
        else:
            for field in line.split(" "):
                found.add(field.split(":")[0])

    return num_valid


def star2():
    required = ["hcl", "iyr", "eyr", "ecl", "pid", "byr", "hgt"]
    found = dict()
    num_valid = 0
    counter = 0

    for line in open("../../inputs/day4.txt", "r"):
        line = line.strip()
        if line == "":
            if all([x in found for x in required]) and _validate_fields(found, counter):
                num_valid += 1
            found.clear()
        else:
            for field in line.split(" "):
                found[field.split(":")[0]] = field.split(":")[1]
        counter += 1

    return num_valid


print("STAR 1: {}\nSTAR 2: {}".format(star1(), star2()))
