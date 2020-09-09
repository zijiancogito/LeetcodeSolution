impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for ch in s.chars(){
           if ch == '(' || ch == '{' || ch == '['{
               stack.push(ch);
           } else {
               if ch == ')'{
                   if let Some(tmp_ch) = stack.pop(){
                       match tmp_ch{
                           '(' => continue,
                           _ => return false
                       }
                   } else {
                       return false;
                   }
               }
               else if ch == '}'{
                   if let Some(tmp_ch) = stack.pop(){
                       match tmp_ch{
                           '{' => continue,
                           _ => return false
                       }
                   } else {
                       return false;
                   }
               }
               else if ch == ']'{
                   if let Some(tmp_ch) = stack.pop(){
                       match tmp_ch{
                           '[' => continue,
                           _ => return false
                       }
                   } else {
                       return false;
                   }
               }
               
           }
        }
        if stack.len() == 0 {
            return true;
        } else{
            return false
        }
    }
}
