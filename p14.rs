impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return "".to_string();
        }
        if strs.len() == 1 {
            return strs[0].to_string();
        }
        let mut prefixs: Vec<char> = Vec::new();
        for ch in strs[0].chars() {
            prefixs.push(ch);
        }
        for s in &strs {
            let mut index = 0;
            for ch in s.chars() {
                if let Some(p) = prefixs.get(index) {
                    if ch != prefixs[index] {
                        for i in (0..prefixs.len() - index) {
                            prefixs.pop();
                        }
                        break;
                    }
                    index = index + 1;
                }
            }
            if let Some(p) = prefixs.get(s.chars().count()) {
                for i in (0..prefixs.len()-s.chars().count()) {
                    prefixs.pop();
                }
            }
        }
        let mut res: String = String::new();
        for ch in prefixs {
            res.push(ch);
        }
        return res;
    }
}
