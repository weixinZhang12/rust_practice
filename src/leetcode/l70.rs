pub fn L(n: i32,v:& mut Vec<i32>) -> i32 {
    if n == 1 {
        return 1;
    }
    if n==2 {
        return 2;
    }
    if v[n as  usize]!=0 {
        return v[n as usize];
    }
    v[n as usize]= L(n - 1,v) + L(n - 2,v);
    v[n as usize]
}
pub fn climb_stairs(n: i32) -> i32 {
    let mut v:Vec<i32>=vec![0;(n+1) as usize];
    L(n,&mut v)
}
