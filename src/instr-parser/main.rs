use ppc_disas::instr::{self, InstrLookup, InstrMnem};
use std::{
    collections::{hash_map::Entry, HashMap},
    fs::{self, File},
    io::Read,
    str::FromStr,
};

fn main() {
    //read in copy pasted table from pdf manual
    let filename = "./src/instr-parser/instr_by_func.txt";
    let mut f = File::open(&filename).expect("no file found");
    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("buffer overflow");

    //exclude whitespace and comments
    let lines = buf
        .lines()
        .filter(|e| !e.is_empty() && !(e[0..2] == *"//"))
        .collect::<Vec<&str>>();

    //bucket instructions by main opcode, but keep track of secondary opcode
    let mut instr_buckets: HashMap<usize, Vec<(InstrMnem, Option<usize>)>> = HashMap::new();
    let instr_raws = lines
        .iter()
        .map(|e| e.split(" ").collect::<Vec<&str>>())
        .map(|e| ((e[1], e[e.len() - 2]), e[0]))
        .map(|(opcode, mn)| {
            (
                (
                    opcode.0.parse::<usize>().unwrap(),
                    opcode.1.parse::<usize>().ok(),
                ),
                InstrMnem::from_str(mn).expect(&format!("could not parse value {}", mn)),
            )
        })
        .collect::<Vec<((usize, Option<usize>), InstrMnem)>>();

    for (opcode, inst) in instr_raws.into_iter() {
        instr_buckets
            .entry(opcode.0)
            .and_modify(|e| e.push((inst, opcode.1)))
            .or_insert(vec![(inst, opcode.1)]);
    }

    //create final 2 layer LUT
    let mut instruction_LUT: HashMap<usize, InstrLookup> = HashMap::new();

    for (opcode, v) in instr_buckets.into_iter() {
        /*if v.len() == 1 {
            match instruction_LUT.entry(opcode) {
                Entry::Occupied(_o) => panic!("debug, this should never ever hit"),
                Entry::Vacant(vac) => {
                    vac.insert(InstrLookup::Final(v[0].0));
                }
            }
            continue;
        } else {
            for secondary in v.iter() {
                instruction_LUT
                    .entry(opcode)
                    .and_modify(|e| match e {
                        InstrLookup::Final(_f) => {
                            unreachable!("debug, this should never hit");
                        }
                        InstrLookup::SecondaryLookup(hm) => {
                            match hm.entry(secondary.1.unwrap()) {
                                Entry::Occupied(_o) => {
                                    unreachable!("{}",format!("tried to rewrite a secondary lookup - original opcode {}, secondary opcode {:?}, mnemonic {}",opcode,secondary.1,secondary.0))
                                }
                                Entry::Vacant(v) => {
                                    v.insert(secondary.0);
                                }
                            }
                        }
                    })
                    .or_insert(InstrLookup::SecondaryLookup(HashMap::new()));
            }
        }*/
        match instruction_LUT.entry(opcode) {
            Entry::Occupied(_o) => {
                unreachable!("we should never hit an occupied top level LUT entry")
            }
            Entry::Vacant(vac) => match v.len() {
                1 => {
                    vac.insert(InstrLookup::Final(v[0].0));
                }
                _ => {
                    if let InstrLookup::SecondaryLookup(s) =
                        vac.insert(InstrLookup::SecondaryLookup(HashMap::new()))
                    {
                        for secondary in v {
                            //s.insert(secondary.1.unwrap(), secondary.0);
                            match s.entry(secondary.1.unwrap()) {
                                Entry::Occupied(o2) => {
                                    unreachable!("")
                                }
                                Entry::Vacant(vac2) => {
                                    vac2.insert(secondary.0);
                                }
                            }
                        }
                    }
                }
            },
        }
    }

    //debug print to check our work
    for entry in instruction_LUT.iter() {
        match entry.1 {
            InstrLookup::Final(f) => {
                println!("{}: {:?}", entry.0, f)
            }
            InstrLookup::SecondaryLookup(s) => {
                println!("{}", entry.0);
                for e in s.iter() {
                    println!("\t{}: {}", e.0, e.1)
                }
            }
        }
    }
}
