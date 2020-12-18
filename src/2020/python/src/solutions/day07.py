import re
from collections import defaultdict

bags = defaultdict(dict)
for l in open("../../inputs/day7.txt", "r"):
    bag = re.match(r"(.*) bags contain", l).groups()[0]
    for count, b in re.findall(r"(\d+) (\w+ \w+) bag", l):
        bags[bag][b] = int(count)


def _star1():
    answer = set()

    def search(color):
        for b in bags:
            if color in bags[b]:
                answer.add(b)
                search(b)

    search("shiny gold")
    return len(answer)


def _star2():
    def search(bag):
        count = 1
        for s in bags[bag]:
            multiplier = bags[bag][s]
            count += multiplier * search(s)
        return count

    return search("shiny gold") - 1  # Rm one for shiny gold itself


def day07():
    print("DAY 7\n=====\nSTAR 1: {}\nSTAR 2: {}\n".format(_star1(), _star2()))
