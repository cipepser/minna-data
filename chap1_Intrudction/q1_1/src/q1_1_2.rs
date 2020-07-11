use std::{
    fs::File,
    io::{
        self,
        prelude::*,
        BufReader,
        BufWriter,
    },
    fmt,
};

#[test]
fn q1_1_2() {
    let f = File::open("./data.txt").unwrap();
    let mut f = BufReader::new(f);

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    'outer: loop {
        let mut stack: Vec<String> = vec![];
        for _ in 0..50 {
            let mut buf = String::new();
            let num_bytes = f.read_line(&mut buf).unwrap();
            if num_bytes == 0 {
                out_rev(&mut stdout, &mut stack);
                break 'outer;
            } else {
                stack.push(buf.trim().to_string());
            }
        }
        out_rev(&mut stdout, &mut stack);
    }
    stdout.flush().unwrap();
}

fn out_rev<W, T>(w: &mut W, stack: &mut Vec<T>)
    where W: io::Write, T: Sized + fmt::Debug
{
    for elem in stack.iter().rev() {
        writeln!(w, "{:?}", elem).unwrap();
    }
}