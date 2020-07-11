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
fn q1_1_4() {
    let f = File::open("./data4.txt").unwrap();
    let f = BufReader::new(f);

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    let mut set: HashSet<String> = HashSet::new();

    for line in f.lines() {
        let s = &line.unwrap();
        match set.get(s) {
            None =>  {
                writeln!(stdout, "{:?}", s).unwrap();
                set.insert(s.to_string());
            },
            Some(_) => {},
        }
    }
    stdout.flush().unwrap();
}

