use std::io;

pub fn p38() {
    println!("请输入矩阵3*3");
    let mut arr: [[i32; 3]; 3] = [[0; 3]; 3];
    let mut sum = 0;
    for i in 0..3 {
        for j in 0..3 {
            // 用来存储用户的输入
            let mut input = String::new();
            // 获取用户的输入
            io::stdin().read_line(&mut input).expect("读取失败");
            // 错误处理
            let input: i32 = match input.trim().parse() {
                Ok(f) => f,
                Err(v) => {
                    println!("{}，输入的不是数字", v);
                    return;
                }
            };
            // input = 12;
            arr[i][j] = input;
        }
    }
    println!("");
    // 计算
    for i in 0..3 {
        sum += arr[i][i];
    }
    // 输出结果
    println!("你输的总和为{}", sum);
}
