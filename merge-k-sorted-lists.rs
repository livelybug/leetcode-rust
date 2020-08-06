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

fn comp_inst(mut list: Vec<i32>, val: i32) -> Vec<i32> {
    println!("list_re = {:?}", list);
    if(list.len() == 0) {
        list.push(val);
        return list;
    }
    
    if(val < list[0]) {
        list.insert(0, val);
        return list;
    }
    
    let len = list.len();
    for i in 0..len {
        if(val >= list[len - 1 - i]) {
            list.insert(len - i, val);
            break;
        }
    }
    
    list
}


fn _merge_k_lists(lists: Vec<Option<Box<ListNode>>>, mut list_re: Vec<i32>) -> Vec<i32> {
        let mut lists_nx = vec![];        
        
        for _list in lists {
            match _list {
                Some(list) => {
                    println!("val = {:?}", list.val); // print the elements in "row"
                    list_re = comp_inst(list_re, list.val);
                    lists_nx.push(list.next);
                },    
                None => {
                    // ;
                    println!("End of list!")
                }
            }    
        }
        
        if(lists_nx.len() == 0) {
            return list_re;    
        } 
    
        _merge_k_lists(lists_nx, list_re)
}


impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut list_re: Vec<i32> = vec![];

        list_re = _merge_k_lists(lists, list_re);        
        
        // println!("Final list_re = {:?}", list_re);
        
        if(list_re.len() == 0) {
            return None;
        }
        
        let mut len = list_re.len();
        let mut node = Some(Box::new(ListNode::new(list_re[len - 1])));

        for i in 0..(len - 1) {
            let _node = ListNode {
                val: list_re[len - 2 - i],
                next: node
            };
            node = Some(Box::new(_node));
        }

        node
    }
}

