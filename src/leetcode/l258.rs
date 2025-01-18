pub fn add_digits(num: i32) -> i32 {
    let mut num = num;
    let mut v: Vec<i32> = vec![0];
    // 存放最后一个数
    let mut temp = 0;
    // 将数字转换为数字数组
    while num != 0 {
        temp = num % 10;
        num /= 10;
        v.push(temp);
    }
    // 只要长度大于1,就加起来然后重复上述步骤
    while v.len() > 1 {
        for i in v {
            num += i;
        }
        v = vec![];
        while num != 0 {
            temp = num % 10;
            num /= 10;
            v.push(temp);
        }

    }
    v[0]

    // println!("{:?}", v);
    
}
