from dataclasses import dataclass
from tqdm import tqdm
from functools import lru_cache
import sys
import argparse

calls = 0
cache = {}

@dataclass
class LratLine:
    clause_id: int
    lits: list[int]
    deps: list[int]


def parse_lrat_line(line):
    cid = int(line.split(" ")[0])
    frags = line.split(" 0 ")
    lits = list(map(int, frags[0].split(" ")[1:]))
    deps = list(map(int, frags[1].split(" ")[:-1]))
    return LratLine(cid, lits, deps)

@lru_cache()
def get_deps_ref(cid):
    if cid < num_base_clauses:
        s = set()
        s.add(cid)
        return s
    else:
        s = set()
        for dcid in lrat_lines[cid]:
            s = s.union(get_deps_ref(dcid))
        return s

def get_deps(cid):
    to_visit = set()
    seen = set()
    for dcid in lrat_lines[cid]:
        to_visit.add(dcid)
    res = set()
    while len(to_visit) != 0:
        current = to_visit.pop()
        if current < num_base_clauses:
            res.add(current)
        elif current in cache:
            res = res.union(cache[current])
        else:
            for ddcid in lrat_lines[current]:
                if ddcid not in seen:
                    to_visit.add(ddcid)
                    seen.add(ddcid)
    cache[cid] = res
    return res

            


if __name__ == "__main__":

    parser = argparse.ArgumentParser()
    parser.add_argument("--lrat", dest="lrat", required=True)
    parser.add_argument("--learned-gap", dest="lgap", type=int, default=0)
    parser.add_argument("--unit-gap", dest="ugap", type=int, default=0)
    parser.add_argument("--unit-count", dest="ucount", type=int, default=1)
    parser.add_argument("--cone-size", dest="csize", type=int, default=0)
    args = parser.parse_args()

    f = open(args.lrat, "r")
    lines = f.readlines()
    num_base_clauses = int(lines[0].split(" ")[0])
    lrat_lines = {}
    last_printed = 0
    units_printed = 0
    units_seen_since_last_print = 0
    proof_steps_since_last_print = 0

    for i, line in enumerate(tqdm(lines)):
        if "d" in line:
            continue
        lrat_line = parse_lrat_line(line)
        lrat_lines[lrat_line.clause_id] = lrat_line.deps

        proof_steps_since_last_print += 1
        if len(lrat_line.lits) != 1:
            continue
        else:
            calls = 0
            cone_size = len(get_deps(lrat_line.clause_id))
            units_seen_since_last_print += 1
            if len(lrat_line.lits) == 1 and cone_size >= args.csize and i - last_printed >= args.lgap and units_seen_since_last_print >= args.ugap:
                print("unit: {}, cone size: {}, proof steps since previous print: {}".format(lrat_line.lits[0], cone_size, proof_steps_since_last_print))
                units_seen_since_last_print = 0
                proof_steps_since_last_print = 0
                last_printed = i
                units_printed += 1
                if units_printed >= args.ucount:
                    exit(0)
