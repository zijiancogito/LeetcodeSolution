impl Solution {
    pub fn getValue(c: u8) -> i32 {
        match c {
            b'I'=> return 1,
            b'V'=> return 5,
            b'X'=> return 10,
            b'L'=> return 50,
            b'C'=> return 100,
            b'D'=> return 500,
            b'M'=> return 1000,
            _ => return 0
        };
    }
    pub fn roman_to_int(s: String) -> i32 {
        let mut sum = 0;
        let bytes = s.as_bytes();
        for (i, &c) in bytes.iter().enumerate() {
            let mut next_c_value = 0;
            if i < s.len() - 1 {
                let next_c = bytes[i+1];
                next_c_value = Self::getValue(next_c);
            }
            let c_value = Self::getValue(c);
            
            if c_value < next_c_value {
                sum = sum - c_value;
            }
            else {
                sum = sum + c_value;
            }
            let i = i + 1;
        }
        return sum;
    }
}
