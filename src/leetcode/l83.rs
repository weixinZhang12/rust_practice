use std::collections::HashMap;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut array: HashMap<i32, i32> = HashMap::new();
    let mut head = head;
    let mut L = &mut head as *mut Option<Box<ListNode>>;
    let mut R = &mut head as *mut Option<Box<ListNode>>;
    while let Some(right_value) = unsafe { &mut *R } {
        L = R;
        R = &mut right_value.next as *mut Option<Box<ListNode>>;
        // 如果哈希表中没有，插入该元素，如果存在，删除链表中的该元素
        if array.get(&right_value.val).is_none() {
            array.insert(right_value.val, right_value.val);
        } else {
            let mut L = unsafe { &mut *L };
            let R = unsafe { &mut *R };
            L.take().unwrap().next = R.take().unwrap().next;
        }
    }
    head
}
pub fn test(){
    let mut a=ListNode::new(1);
    let mut b=ListNode::new(1);
    let mut c=ListNode::new(1);
    a.next=Some(Box::new(b));
    // b.next=Some(Box::new(c));
    
}
