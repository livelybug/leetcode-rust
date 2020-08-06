fn push_space(mut line: &mut Vec<char>, times: i32) -> & mut Vec<char> {
    if(times > 0) {
        line.push(' ');
        return push_space(line, times -1);         
    } else {
        return line;
    }    
}


impl Solution {    
    
    pub fn convert(s: String, num_rows: i32) -> String {
        use std::collections::HashMap;

        if(num_rows == 1) { return s; }
        
        let mut row_num: i32 = 0;
        let mut step: i32 = 0;
        let mut lines = HashMap::new();
        
        for (idx, char) in s.chars().enumerate() {
            // println!("char = {:?}, row_num = {}, step = {}", (idx,char), row_num, step);
            
            if lines.contains_key(&row_num) == false {
                lines.insert(row_num, Vec::new());
            }

            let line = lines.get_mut(&row_num).unwrap();            
            
            if step <= num_rows -1 {
                if ((idx - step as usize) as usize)  % ((2 * num_rows - 2) as usize) == 0 {
                    line.push(char);
                    push_space(line, num_rows - 2);
                }                
            } else {
                let _tmp = step - (num_rows -1);
                let _len = num_rows - 2;
                // println!("tmp = {} ,len = {}", _tmp, _len);
                
                // if _tmp == 1 { line.push(char); push_space(line, _len - _tmp); }
                // else if _tmp == _len { push_space(line, _len - 1); line.push(char); }
                // else { push_space(line, _tmp - 1); line.push(char);  push_space(line, _len - _tmp); }
                push_space(line, _tmp - 1); line.push(char);  push_space(line, _len - _tmp);
            }

            step = step + 1;                        
            if step == 2*num_rows - 2 {step = 0;}

            if step > (num_rows -1) {row_num = num_rows -1 - (step - (num_rows -1))}
            else {row_num = step;}
                        
            // println!("lines - {:?}", lines);
            
        }
                    
        // println!("lines - {:?}", lines);
        // println!("line2 - {:?}", line2);            
        
        let mut re = String::from("");        
        for idx in 0..num_rows {
            if let Some(val) = lines.get(&idx) {
                // println!("{:?}", val);
                for _val in val {
                    if !_val.is_whitespace() {
                        re.push(*_val);
                    } 
                }
            }
        }

        println!("re = {}", re);
        re
     }
}
