pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    let mut v: Vec<i32> = vec![];
    let mut num_split_array:Vec<i32>=vec![];
    let mut temp = 0;
    let mut flag: bool=false;
    for i in left..=right {
        let mut num = i;
        while num != 0 {
            temp=num%10;
            num/=10;
            num_split_array.push(temp);
            // temp=0;
        }
        flag=true;
        for j in num_split_array {
           
            if j==0||i%j!=0 {
                flag=false;
                break;
            }
        }
        if flag {
            v.push(i);
        }
        num_split_array=vec![];
    }
    v
}
