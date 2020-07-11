use std::{
    fs::File,
    io::{
        self,
        prelude::*,
        BufReader,
        BufWriter,
    },
    collections::BTreeMap,
};

#[derive(Debug, Clone)]
struct ValueWithCount {
    inner: BTreeMap<String, u32>,
}

impl ValueWithCount {
    fn new() -> ValueWithCount {
        ValueWithCount {
            inner: BTreeMap::new(),
        }
    }

    fn inclement(&mut self, key: String) -> &mut Self {
        let count = self.inner.entry(key)
            .or_insert_with(|| 0);
        *count += 1;
        self
    }
}

#[test]
fn q1_1_7() {
    let f = File::open("./data6.txt").unwrap();
    let f = BufReader::new(f);

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    let mut btreemap: BTreeMap<u32, ValueWithCount> = BTreeMap::new();

    for line in f.lines() {
        let s = line.unwrap();
        btreemap.entry(s.len() as u32)
            .or_insert_with(|| ValueWithCount::new())
            .inclement(s);
    }

    for (_, vwc) in btreemap {
        for (s, count) in vwc.inner {
            if count > 1 {
                writeln!(stdout, "{:?} (counts: {:?})", s, count).unwrap();
                // writeln!(stdout, "{:?}", count).unwrap();
            } else {
                writeln!(stdout, "{:?}", s).unwrap();
            }
        }
    }
    stdout.flush().unwrap();
}

