impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut y: i32 = x % 10;
        let mut xx = x / 10;
        let mut res = loop {
            if xx == 0 {
                break y;
            }
            let i = xx % 10;
            if let Some(tmp) = y.checked_mul(10i32) {
                if let Some(tmp) = y.checked_add(i) {
                    y = y * 10 + i;
                    xx = xx / 10;
                }
                else{
                    break 0i32;
                }
            }
            else {
                break 0i32;
            }
        };
        return res;
    }
}
