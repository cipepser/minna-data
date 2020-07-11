use std::{
    fs::File,
    io::{
        self,
        prelude::*,
        BufReader,
        BufWriter,
    },
    collections::HashSet
};

#[test]
fn q1_1_5() {
    let f = File::open("./data4.txt").unwrap();
    let f = BufReader::new(f);

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    let mut set: HashSet<String> = HashSet::new();

    for line in f.lines() {
        let s = &line.unwrap();
        match set.get(s) {
            None =>  {
                set.insert(s.to_string());
            },
            Some(_) => {
                writeln!(stdout, "{:?}", s).unwrap();
            },
        }
    }
    stdout.flush().unwrap();
}

