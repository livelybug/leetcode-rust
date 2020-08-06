impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashSet;
        
        let mut current_sub: Vec<char> = Vec::new();
        let mut max_len = 0;
        
        for (idx, val) in s.chars().enumerate() {
            // println!("{:?}", (idx, val));
            if(current_sub.contains(&val)) {
                // println!("dup = {}", val);
                if(current_sub.len() > max_len) {max_len = current_sub.len()}
                let index = current_sub.iter().position(|&r| r == val).unwrap();
                current_sub= current_sub.split_at(index + 1).1.to_vec();
                current_sub.push(val);
            } else {
                current_sub.push(val);
            }
        }

        if(current_sub.len() > max_len) {max_len = current_sub.len()}
        max_len as i32
    }
}
