
#![allow(dead_code)]
#![allow(unused_variables)]


use crate::leetcode::{l171, l389};

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
    pub mod l58;
    pub mod l66;
    pub mod l69;
    pub mod l540;
    pub mod l67;
    pub mod l136;
    pub mod l168;
    pub mod l35;
    pub mod l88;
    pub mod l83;
    pub mod l118;
    pub mod l217;
    pub mod l290;
    pub mod l414;
    pub mod l441;
    pub mod l70;
    pub mod l268;
    pub mod l258;
    pub mod l728;
    pub mod l709;
    pub mod l125;
    pub mod l171;
    pub mod l344;
    pub mod l389;
}

fn main() {
    let result =l389::find_the_difference("ab".to_string(),"acb".to_string());
    println!("{:?}", result);
}
