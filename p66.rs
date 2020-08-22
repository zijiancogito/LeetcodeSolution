impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut flag = 1;
        for index in (0..digits.len()).rev() {
            let mut x: i32 = digits[index] + flag;
            if x > 9 {
                x = 0;
                flag = 1;
                result.push(x);
            }
            else {
                flag = 0;
                result.push(x);
            }
        }
        if flag != 0 {
            result.push(flag);
        }
        let mut res_rev: Vec<i32> = Vec::new();
        for index in (0..result.len()).rev() {
            res_rev.push(result[index]);
        }
        return res_rev;
    }
}
