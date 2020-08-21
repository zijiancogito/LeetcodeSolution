impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        match x {
            0 => return true,
            1 => return true,
            2 => return true,
            3 => return true,
            4 => return true,
            5 => return true,
            6 => return true,
            7 => return true,
            8 => return true,
            9 => return true,
            10 => return false,
            _ => ()
        }
        if x % 10 == 0 {
            return false;
        }
        let mut xx = x;
        let mut y = 0;
        loop{
            y = y * 10 + xx % 10;
            // println!("y: {}", y);
            let tmp = xx;
            // println!("tmp: {}", tmp);
            xx = xx / 10;
            // println!("xx: {}", xx);
            if xx < y {
                if tmp == y {
                    return true;
                }
                else{
                    return false;
                }
            }
            if xx == y {
                return true;
            }
        }
    }
}
