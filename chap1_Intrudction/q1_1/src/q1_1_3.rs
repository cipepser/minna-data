use ringbuf::RingBuffer;

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
fn q1_1_3() {
    let f = File::open("./data3.txt").unwrap();
    let mut f = BufReader::new(f);

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    let rb = RingBuffer::<String>::new(42);
    let (mut prod, mut cons) = rb.split();
    loop {
        let mut buf = String::new();
        let num_bytes = f.read_line(&mut buf).unwrap();
        if num_bytes == 0 || &*buf == "\n".to_string() {
            break;
        }
        if prod.is_full() {
            cons.pop().unwrap();
        }
        prod.push(buf.trim().to_string()).unwrap();
    }
    let elem = cons.pop().unwrap();
    assert_eq!(elem, "199".to_string());
    writeln!(stdout, "{:?}", elem).unwrap();
    stdout.flush().unwrap();
}

