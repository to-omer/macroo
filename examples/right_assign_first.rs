#![allow(clippy::useless_vec)]

#[derive(Copy, Clone, Debug, PartialEq)]
struct I(i32);

impl std::ops::AddAssign for I {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

#[macroo::right_first_assign]
fn main() {
    let mut a = vec![I(1)];
    a[0] += a[0];
    assert_eq!(a, vec![I(2)]);
}

/*
fn main() {
    let mut a = vec![I(1)];
    a[0] += a[0]; // error[E0502]: cannot borrow `a` as immutable because it is also borrowed as mutable
    assert_eq!(a, vec![I(2)]);
}
*/
