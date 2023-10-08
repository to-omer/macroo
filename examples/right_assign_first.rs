#![allow(clippy::useless_vec)]

#[derive(Copy, Clone, Debug, PartialEq)]
struct I(i32);

impl std::ops::AddAssign for I {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

#[macroo::right_first_assign]
fn run(x: i32) {
    let mut a = vec![I(x)];
    a[0] += a[0];
    assert_eq!(a[0], I(2));
}

fn main() {
    run(1);
}
