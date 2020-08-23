impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut x = s.len();
        if x == 0 {
            return 0;
        }
        let mut l = 0;
        let b = s.as_bytes();
        
        for index in (0..x).rev() {
            let tmp = b[index];
            if tmp == b' ' && l == 0 {
                continue;
            }
            else {
                if tmp == b' ' {
                    break;
                }
            }
            l = l + 1;
            if index == 0 {
                break;
            }
        }
        return l;
    }
}
