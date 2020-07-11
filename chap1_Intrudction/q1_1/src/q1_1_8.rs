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
fn q1_1_8() {
    let f = File::open("./data.txt").unwrap();
    let f = BufReader::new(f);

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut stack_even = vec![];
    let mut stack_odd = vec![];
    for (i, line) in f.lines().enumerate() {
        if i % 2 == 0 {
            stack_even.push(line.unwrap());
        } else {
            stack_odd.push(line.unwrap());
        }
    }

    for elem in stack_even.iter() {
        writeln!(stdout, "{:?}", elem).unwrap();
    }
    for elem in stack_odd.iter() {
        writeln!(stdout, "{:?}", elem).unwrap();
    }
    stdout.flush().unwrap();
}