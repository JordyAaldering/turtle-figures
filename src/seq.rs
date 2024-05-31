/// Pure morphic sequence
pub struct Seq<T> {
    seq: Vec<T>,
    f: fn(T) -> Vec<T>,
}

impl<T: Copy> Seq<T> {
    pub fn new(init: T, f: fn(T) -> Vec<T>) -> Self {
        Seq { seq: vec![init], f }
    }

    pub fn take(&mut self, n: usize) -> &Vec<T> {
        while n > self.seq.len() {
            self.apply();
        }
        &self.seq
    }

    fn apply(&mut self) {
        let n = self.seq.len();
        for i in (0..n).rev() {
            let xs = (self.f)(self.seq[i]);
            self.seq[i] = xs[0];
            for (j, &x) in xs.iter().skip(1).enumerate() {
                self.seq.insert(i + j + 1, x);
            }
        }
    }
}

/// Binary fibonacci sequence
/// ```
/// f(0) = 01
/// f(1) = 0
/// ```
#[allow(dead_code)]
pub fn fibonacci(x: i32) -> Vec<i32> {
    match x {
        0 => vec![0, 1],
        1 => vec![0],
        _ => unreachable!(),
    }
}

/// Period doubling sequence
/// ```
/// f(0) = 01
/// f(1) = 00
/// ```
#[allow(dead_code)]
pub fn doubling(x: i32) -> Vec<i32> {
    match x {
        0 => vec![0, 1],
        1 => vec![0, 0],
        _ => unreachable!(),
    }
}
