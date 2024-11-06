pub fn str_str(haystack: String, needle: String) -> i32 {
    let mut left: usize = 0;
    let mut right: usize = 0;
    let mut temp = 0;
    let str_length = haystack.len();
    let pattern_length = needle.len();
    if str_length==0||pattern_length>str_length {
        return -1;
    }

    let str_array: Vec<char> = haystack.chars().collect();
    let needle_array: Vec<char> = needle.chars().collect();
    for _ in 0..str_length - pattern_length + 1 {
        if str_array[left] == needle_array[right] {
            right += 1;
            left+=1;
            if right==pattern_length-1 {
                return temp as i32;
            }
        } else {
            left += 1;
            temp = left;
            right = 0
        }
    }
    return -1 as i32;
}
