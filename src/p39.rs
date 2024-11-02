use std::io;
fn sort(array: &mut [i32])->&mut [i32] {
    let mut temp = 0;
    // 最大索引为10
    for max_index in (0..=array.len() - 1).rev() {
        for i in 0..max_index {
            if array[i] > array[i + 1] {
                temp = array[i];
                array[i] = array[i + 1];
                array[i + 1] = temp;
            }
        }
    }
    array
}
pub fn p39() {
    let mut arr: [i32; 11] = [11, 23, 34, 45, 3, 23, 1, 23, 23, 23, 0];
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input_number: i32 = input.trim().parse().expect("输入错误");
    // 获取输入的数字
    let arr_length = arr.len();
    // 将最后一个元素变为输入的数字
    arr[arr_length - 1] = input_number;
    println!("未排序{:?}",arr);
    let arr=sort(&mut arr);
    println!("{:?}",arr)
    // 进行排序
}
