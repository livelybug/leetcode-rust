// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
fn get_val(list: Option<Box<ListNode>>, index: u32, mut sum: u128) -> u128 {
    match list {
        Some(node) => {
            // println!("{} = {:?}", index, node.val);
            sum = sum + ((node.val as u128) * (10 as u128).pow(index));
            // println!("sum = {}", sum);
            if(node.next != None) {
                return get_val(node.next, index + 1, sum);
            } else {
                return sum;
            }
            
        },
        None => {panic!("!1");}
    }    
}

fn get_list_node(val: u128, index: i32, node: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mod_ = (10 as u128).pow(index as u32);
    // println!{"mod1 = {}, val = {}", mod_, val};
    let pre_val = val / mod_ % 10;
    // println!("extracted = {}", pre_val);

    let mut pre_node = ListNode::new(pre_val as i32);
    pre_node.next = node;
    
    if(index > 0) {
        get_list_node(val, index - 1, Some(Box::new(pre_node)))            
    } else {
        Some(Box::new(pre_node))
    }
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {        
        let val1 = get_val(l1, 0, 0);
        let val2 = get_val(l2, 0, 0);
        println!("val1 = {}, val2 ={}, val1 + val2 = {}", val1, val2, val1 + val2);

        let val = val1 + val2;
        if(val == 0) {
            return Some(Box::new(ListNode::new(0)));
        }

        let mut index = (val as f64).log(10.0).floor() as i32;
        let mut mod_ = (10 as u128).pow(index as u32);
        println!("index = {}, mod_= {}", index, mod_);

        if(val / mod_ >= 10) {
            mod_ = mod_ * 10;
            index = index + 1;
        }

        let init_val =  val / mod_ % 10;
        println!("mode_ = {}, init_val = {}", mod_, init_val);
        let init_node = Some(Box::new(ListNode::new(init_val as i32)));

        if(index >= 1) {
            get_list_node(val, index - 1, init_node)
        } else {
            init_node
        }
    }
}
