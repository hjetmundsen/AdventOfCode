from collections import defaultdict

rules = [line.split('contain') for line in open('../../inputs/day8.txt', 'r')]

nodes = dict()

outer = set()

class Node:
    def __init__(self):
        self.parents = []

graph = defaultdict(list)

for rule in rules:
    root = rule[0].split(' ')
    root = ' '.join(root[0:2])
    children = rule[1].split(',')
    children = [child.split(' ') for child in children]
    children = [' '.join(child[2:4]) for child in children]

    nodes[root] = Node()
    outer.add(nodes[root])
    for child in children:
        if child not in nodes: nodes[child] = Node()
        nodes[child].parents.append(nodes[root])

dfs = [node for node in nodes['shiny gold'].parents]
visited = set()
num_ancestors = 0

while dfs:
    temp = dfs.pop()

    if temp in outer: num_ancestors += 1

    for parent in temp.parents:
        if parent not in visited:
            visited.add(parent)
            dfs.append(parent)

print(num_ancestors)
