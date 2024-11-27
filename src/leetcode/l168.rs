

pub fn convert_to_title(column_number: i32) -> String {
    let mut column_number = column_number;
    let mut shang = -1;
    let mut yushu = 0;
    let mut yushu_arr: Vec<i32> = vec![];
    // 余数为零的情况代表进位
    while column_number!=0 {
        yushu = column_number % 26;
        if yushu==0 {
            yushu=26;
            column_number-=1;
        }
        yushu_arr.push(yushu);
        column_number = column_number / 26
    }
    yushu_arr.reverse();
    let mut c:String=yushu_arr.iter().map(|&n|(n as u8 +b'A'-1) as char).collect();
    println!("{:?}",yushu_arr);
    println!("{:?}",c);
    c
}
