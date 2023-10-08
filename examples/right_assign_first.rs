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
    assert_eq!(a[0], I(2));
}
