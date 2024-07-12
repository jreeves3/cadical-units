import sys
from time import sleep

f = open(sys.argv[1], "r")
lines = f.readlines()
weights = {}

for i, line in enumerate(lines):
    cid = int(line.split(" ")[0])
    cid_weight = 0
    for id in line.split(" 0 ")[-1].split(" ")[:-1]:
        if id == "d":
            continue
        id = int(id)
        if id in weights:
            cid_weight += weights[id]
        else:
            cid_weight += 1
    weights[cid] = cid_weight

    if cid_weight > 2 ** 64:
        print(i, cid_weight)
        sys.exit(1)
