use proconio::{
    input,
    source::{
        line::LineSource,
        once::OnceSource,
    },
    marker::Bytes,
};
use std::{
    fs::File,
    io::{self, BufReader, BufWriter, Write as _},
};

#[test]
#[proconio::fastout]
fn q1_1_1() {
    let f = File::open("./data.txt").unwrap();
    let mut input_steam = BufReader::new(f);
    let source = LineSource::new(input_steam);

    // let stdout = io::stdout();
    // let mut stdout = BufWriter::new(stdout.lock());

    input! {
        from source,
        lines: [Bytes; 30],
    }
    println!("{:?}", lines);

    for line in lines.iter().rev() {
        let s = String::from_utf8(line.to_vec()).unwrap();
        println!("{}", s);
        // writeln!(stdout, "{:?}", line).unwrap();
    }
    // stdout.flush().unwrap();
}