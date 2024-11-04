use leetcode::l20;

mod leetcode{
     pub  mod l7;
     pub mod l1;
     pub mod l13;
     pub mod l14;
     pub mod l20;
}
fn main() {
    // println!("Hello, world!");
    let result=l20::is_vaild("()(){}{".to_string());
    println!("{:?}",result);

}
