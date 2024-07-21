use clap::Parser;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

type Literal = i32;
type Cid = u32;

#[derive(Debug)]
struct LratLine {
    clause_id: Cid,
    lits: Vec<Literal>,
    deps: Vec<Cid>,
}

#[derive(Debug)]
struct Global {
    base_clauses: u32,
    lines: HashMap<Cid, Vec<Cid>>,
    cache: HashMap<Cid, HashSet<Cid>>,
}

#[derive(Parser, Debug)] #[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    lrat: String,

    #[arg(long, default_value_t = 0)]
    learned_gap: u32,
    #[arg(short, long, default_value_t = 1)]
    unit_count: u32,
    #[arg(short, long, default_value_t = 0)]
    cone_size: u32,
}

fn parse_lrat_line(line: &str) -> LratLine {
    let cid: u32 = (line.split(" ").collect::<Vec<_>>()[0]).parse().unwrap();
    let frags = line.split(" 0 ").collect::<Vec<_>>();
    let lits: Vec<Literal> = frags[0]
        .split(" ")
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    let mut deps: Vec<Cid> = frags[1].split(" ").map(|x| x.parse().unwrap()).collect();
    deps.pop();

    LratLine {
        clause_id: cid,
        lits,
        deps,
    }
}

fn cache_deps(data: &mut Global, cid: Cid) {
    use std::collections::hash_map::Entry;
    let mut to_visit: Vec<Cid> = Vec::new();
    let mut seen: HashSet<Cid> = HashSet::new();
    for dcid in data.lines[&cid].iter() {
        to_visit.push(*dcid);
        seen.insert(*dcid);
    }
    let mut res: HashSet<Cid> = HashSet::new();
    while !to_visit.is_empty() {
        let current_cid = to_visit.pop().unwrap();
        if current_cid <= data.base_clauses {
            res.insert(current_cid);
        } else {
            match data.cache.entry(current_cid) {
                Entry::Occupied(o) => {
                    res.extend(o.get());
                }
                Entry::Vacant(_) => {
                    for ddcid in data.lines[&current_cid].iter() {
                        if !seen.contains(&ddcid) {
                            seen.insert(*ddcid);
                            to_visit.push(*ddcid);
                        }
                    }
                }
            }
        }
    }
    data.cache.insert(cid, res);
}

fn cache_deps_rec(data: &mut Global, cid: Cid) {
    if cid <= data.base_clauses {
        let mut s = HashSet::new();
        s.insert(cid);
        data.cache.insert(cid, s);
    } else if !data.cache.contains_key(&cid) {
        let mut s = HashSet::new();
        for dcid in data.lines[&cid].clone() {
            cache_deps_rec(data, dcid);
            for ddcid in data.cache[&dcid].clone() {
                s.insert(ddcid);
            }
        }
        data.cache.insert(cid, s);
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let f = File::open(args.lrat)?;
    let reader = BufReader::new(f);
    let mut lines = reader.lines();
    let first_line = lines.next().unwrap().unwrap();

    let num_base_clauses: u32 = first_line.split(" ").next().unwrap().parse().unwrap();
    let mut glbl = Global {
        base_clauses: num_base_clauses,
        lines: HashMap::new(),
        cache: HashMap::new(),
    };
    for line in lines {
        let line = line?;
        if line.contains("d") {
            continue;
        }
        let lrat_line = parse_lrat_line(&line);
        //println!("{:?}", lrat_line);
        glbl.lines.insert(lrat_line.clause_id, lrat_line.deps);
        if lrat_line.lits.len() != 1 {
            continue;
        }
        cache_deps(&mut glbl, lrat_line.clause_id);
        let cone_size = glbl.cache[&lrat_line.clause_id].len();
        println!("{}, {}, {cone_size}", lrat_line.clause_id, lrat_line.lits[0]);
    }
    Ok(())
}
