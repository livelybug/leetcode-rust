impl Solution {
    pub fn reverse(x: i32) -> i32 {
        // if negative integer
        
        // if overflow
        
        // 
        let mut x_cl = x;
        let mut nums: Vec<i32> = Vec::new();
        
        while x_cl != 0 {
            let s_val = (x_cl % 10);
            nums.push(s_val as i32);
            x_cl = x_cl / 10;
        }
        
        let mut re = 0 as i32;
        let len = nums.len() - 1;
        
        for (idx, s_val) in nums.iter().enumerate() {
            // println!("{:?}", (len - idx, s_val));
            let _tmp = s_val.overflowing_mul(10_i32.pow((len - idx) as u32));
            if(_tmp.1 == true) { 
                return 0; 
            }
            
            let _tmp2 = re.overflowing_add(_tmp.0);
            if _tmp2.1 == true {
                return 0;
            }
            re = _tmp2.0;
        }
        
        // println!{"{:?}", nums};
        // println!{"{:?}", re};
        re
    }
}
