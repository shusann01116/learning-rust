use libc::c_double;

extern "C" {
    fn abs(input: i32) -> i32;
    fn sin(input: c_double) -> c_double;
}

fn main() {
    println!("abs({})={}", -5, unsafe { abs(-5) });
    println!("sin({})={}", 2.0, unsafe { sin(2.0) });
}
