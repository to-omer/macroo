#[macroo::default_int_suffix(u64)]
fn main() {
    println!("{}", !0); // 18446744073709551615
}
