pub fn reverse(x: i32) -> i32 {
    let mut x = x;
    let mut length: usize = 0;
    if x > 0 {
        length = x.to_string().len();
    } else {
        length = x.to_string().len() - 1;
    }
    let mut result: i32 = 0;
    let mut _reserved_num = 0;
    for i in 1..=length {
        let temp = x % 10;
        // result += temp * 10_i32.pow((length - i) as u32);

        if let Some(new_result) =
            result.checked_add(
                
                if let Some(v)=temp.checked_mul(10_i32.pow((length - i) as u32))    {
                    v
                }
                else {
                    return 0;
                }
            )
        {
            result = new_result;
        }else {
            return 0;
        }
        x /= 10;
    }
    return result;
}
