extern crate rssharp_sys;

pub fn main() {
    let x = unsafe { rssharp_sys::n_alm(10, 10) };
    println!("{}", x);
}
