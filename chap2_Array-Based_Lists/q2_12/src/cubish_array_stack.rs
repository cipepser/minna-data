use common::traits::List;
use basic::array_stack::ArrayStack;
use rug::{Float, Assign};

#[derive(Debug, Clone, Default)]
pub struct CubishArrayStack<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> {
    cubes: ArrayStack<ArrayStack<T>>,
    n: usize,
}

pub const PRECISION: u32 = 200;

impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> List<T> for CubishArrayStack<T> {
    fn size(&self) -> usize { self.n }

    fn get(&self, i: usize) -> Option<T> {
        let c = self.i2c(i);
        let j = i - cubic_total(c);
        self.cubes.get(c).unwrap().get(j)
    }

    fn set(&mut self, i: usize, x: T) -> Option<T> {
        // println!("i: {}", i);
        let c = self.i2c(i);
        // println!("c: {}", c);
        let j = i - cubic_total(c);

        let mut cube = self.cubes.get(c).unwrap().clone();
        // println!("cube: {:?}", cube);
        let y = cube.get(j).clone();
        if cube.set(j, x.clone()).is_none() {
            cube.add(j, x.clone());
        };
        self.cubes.set(c, cube.clone());
        y
    }

    fn add(&mut self, i: usize, x: T) {
        let r = self.cubes.size();
        if cubic_total(r) < self.size() + 1 {
            self.grow();
        }
        self.n += 1;
        for j in (i + 1..self.size()).rev() {
            self.set(j, self.get(j - 1).unwrap().clone());
        }
        self.set(i, x.clone());
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        let x = self.get(i).unwrap().clone();
        for j in i..self.size() - 1 {
            self.set(j, self.get(j + 1).unwrap().clone());
        }
        self.n -= 1;

        let r = self.cubes.size();
        if cubic_total(r - 1) >= self.size() {
            self.shrink();
        }
        Some(x)
    }
}

impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> CubishArrayStack<T> {
    pub fn new() -> Self {
        let inner: ArrayStack<T> = ArrayStack::with_capacity(1);
        let mut buf = ArrayStack::with_capacity(1);
        buf.add(0, inner);
        CubishArrayStack {
            cubes: buf,
            n: 0,
        }
    }

    pub fn with_capacity(len: usize) -> Self {
        let mut ras = Self::new();
        while ras.length() < len {
            ras.grow();
        }
        ras
    }

    fn length(&self) -> usize {
        let r = self.cubes.size();
        cubic_total(r)
    }

    fn i2c(&self, i: usize) -> usize {
        // let rt = 2.0 * 3.0 * (729.0 * (4.0 * i as f64 - 9.0) * (4.0 * i as f64 - 9.0) - 3.0).sqrt();
        // let mut cr = rt + 2.0 * 81.0 * (4.0 * i as f64 - 9.0);
        // cr.cbrt();
        //
        // let dc = cr / (6.0 * (2.0 as f64).cbrt()) + (2.0 as f64).cbrt() / (2.0 * cr) - 1.5;
        // dc.ceil() as usize

        // let six = Float::with_val(PRECISION, 6);
        // let rt = Float::with_val(PRECISION, 729 * (16 * i * i + 81 - 72 * i) - 3).sqrt();
        // let tmp = Float::with_val(PRECISION, 162.0 * (4.0 * i as f64 - 9.0));
        // let cr = &six * &rt + &tmp;
        //
        // let mut buf = Float::new(PRECISION);
        // buf.assign(cr);
        // let cr = buf.cbrt();
        //
        // let cb2 = Float::with_val(PRECISION, 2.0).cbrt();
        // let tmp = &cb2 * &six;
        // let mut deno = Float::new(PRECISION);
        // deno.assign(tmp);
        // let first = &cr / deno;
        //
        // let tmp = &cb2 / &cr;
        // let mut second = Float::new(PRECISION);
        // second.assign(tmp);
        // let second = second / &Float::with_val(PRECISION, 2);
        //
        // let minus_3 = Float::with_val(PRECISION, -3);
        // let two = Float::with_val(PRECISION, 2);
        // let third = &minus_3 / &two;
        //
        // let tmp = &first + &second;
        // let mut acc = Float::new(PRECISION);
        // acc.assign(tmp);
        // let mut third_buf = Float::new(PRECISION);
        // third_buf.assign(third);
        // let res_tmp = &acc + &third_buf;
        //
        // let mut res = Float::new(PRECISION);
        // res.assign(res_tmp);
        // let res = res.ceil();
        // println!("{:?}", res);

        let mut dc = 1;
        while cubic_total(dc) <= i {
            dc += 1;
        }
        return dc - 1;
    }

    pub fn grow(&mut self) {
        let cap = (self.cubes.size() + 1) * (self.cubes.size() + 1);
        let new_block = ArrayStack::with_capacity(cap);
        self.cubes.add(self.cubes.size(), new_block);
    }

    fn shrink(&mut self) {
        let mut r = self.cubes.size();
        while r > 0 && cubic_total(r - 1) >= self.size() {
            self.cubes.remove(self.cubes.size() - 1);
            r -= 1;
        }
    }
}

fn cubic_total(r: usize) -> usize {
    r * (r + 1) * (2 * r + 1) / 6
}


#[cfg(test)]
#[test]
fn i2c_test() {
    let a: CubishArrayStack<char> = CubishArrayStack::with_capacity(10);

    assert_eq!(a.i2c(0), 0);
    assert_eq!(a.i2c(1), 1);
    assert_eq!(a.i2c(2), 1);
    assert_eq!(a.i2c(3), 1);
    assert_eq!(a.i2c(4), 1);
    assert_eq!(a.i2c(5), 2);
    assert_eq!(a.i2c(13), 2);
    assert_eq!(a.i2c(14), 3);
}

#[cfg(test)]
#[test]
fn cubish_array_stack() {
    let mut a: CubishArrayStack<char> = CubishArrayStack::with_capacity(10);

    assert_eq!(a.size(), 0);
    a.add(0, 'a');
    a.add(1, 'b');
    a.add(2, 'c');
    a.add(3, 'd');
    a.add(4, 'e');
    a.add(5, 'f');
    a.add(6, 'g');
    a.add(7, 'h');

    assert_eq!(a.size(), 8);
    assert_eq!(a.cubes.size(), 3);
    assert_eq!(a.get(0), Some('a'));
    assert_eq!(a.get(1), Some('b'));
    assert_eq!(a.get(2), Some('c'));
    assert_eq!(a.get(3), Some('d'));
    assert_eq!(a.get(4), Some('e'));
    assert_eq!(a.get(5), Some('f'));
    assert_eq!(a.get(6), Some('g'));
    assert_eq!(a.get(7), Some('h'));

    assert_eq!(a.remove(7), Some('h'));
    assert_eq!(a.remove(6), Some('g'));
    assert_eq!(a.cubes.size(), 3);
    assert_eq!(a.remove(5), Some('f'));
    assert_eq!(a.cubes.size(), 2);
}
