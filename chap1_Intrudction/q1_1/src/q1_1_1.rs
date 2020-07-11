use std::{
    fs::File,
    io::{
        self,
        prelude::*,
        BufReader,
        BufWriter,
    },
};

#[test]
fn q1_1_1() {
    let f = File::open("./data.txt").unwrap();
    let f = BufReader::new(f);

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut stack = vec![];
    for line in f.lines() {
        stack.push(line.unwrap());
    }
    for elem in stack.iter().rev() {
        writeln!(stdout, "{:?}", elem).unwrap();
    }
    stdout.flush().unwrap();
}