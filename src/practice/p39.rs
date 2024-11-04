use std::io;

use colored::Colorize;
fn sort(array: &mut [i32])->&mut [i32] {
    let mut _temp = 0;
    // 最大索引为10
    // 第一次包括最大索引
    // 第二次不包括最大索引
    for max_index in (0..=array.len() - 1).rev() {
        for i in 0..max_index {
            if array[i] > array[i + 1] {
                _temp = array[i];
                array[i] = array[i + 1];
                array[i + 1] = _temp;
            }
        }
    }
    array
}
pub fn p39() {
    let mut arr: [i32; 11] = [11, 23, 34, 45, 3, 23, 1, 23, 23, 23, 0];
    let mut input = String::new();
    println!("当前的数组为{:?}",arr);
    println!("请输入一个数字：");
    io::stdin().read_line(&mut input).unwrap();
    let input_number: i32 = input.trim().parse().expect(&format!("{}","输入错误".red().bold()));
    // 获取输入的数字
    let arr_length = arr.len();
    // 将最后一个元素变为输入的数字
    arr[arr_length - 1] = input_number;
    println!("未排序\t{:?}",arr);
    let arr=sort(&mut arr);
    println!("排序完毕{:?}",arr)
    // 进行排序
}
