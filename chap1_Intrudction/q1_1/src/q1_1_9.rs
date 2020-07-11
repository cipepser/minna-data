use std::{
    fs::File,
    io::{
        self,
        prelude::*,
        BufReader,
        BufWriter,
    },
};
use rand::thread_rng;
use rand::seq::SliceRandom;

#[test]
fn q1_1_9() {
    let f = File::open("./data6.txt").unwrap();
    let f = BufReader::new(f);

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut stack = vec![];
    for line in f.lines() {
        stack.push(line.unwrap());
    }

    let mut rng = thread_rng();
    for elem in stack.choose_multiple(&mut rng, stack.len()) {
        writeln!(stdout, "{:?}", elem).unwrap();
    }
    stdout.flush().unwrap();
}