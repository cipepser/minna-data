use std::{
    fs::File,
    io::{
        self,
        prelude::*,
        BufReader,
        BufWriter,
    },
    collections::{BTreeMap, BTreeSet},
};

#[test]
fn q1_1_6() {
    let f = File::open("./data6.txt").unwrap();
    let f = BufReader::new(f);

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    let mut btreemap: BTreeMap<u32, BTreeSet<String>> = BTreeMap::new();

    for line in f.lines() {
        let s = line.unwrap();
        btreemap.entry(s.len() as u32).or_insert_with(|| BTreeSet::new()).insert(s);
    }

    for (_, inner) in btreemap {
        for s in inner {
            writeln!(stdout, "{:?}", s).unwrap();
        }
    }
    stdout.flush().unwrap();
}

