use leetcode::{l20, l26, l28, l283, l633, l7};

mod leetcode {
    pub mod l1;
    pub mod l13;
    pub mod l14;
    pub mod l20;
    pub mod l26;
    pub mod l27;
    pub mod l283;
    pub mod l3222;
    pub mod l633;
    pub mod l7;
    pub mod l9;
    pub mod l28;
}
fn main() {
   
    let mut array = vec![0, 1, 0, 1, 12];
    let result = l28::str_str("leetcode".to_string(), "leeto".to_string());
    println!("{:?}", result);
}
