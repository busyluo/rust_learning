#![feature(linked_list_cursors)]

use rand::Rng;
use std::collections::LinkedList;
use std::option::Option::Some;

fn main() {
    let mut nums = [0u32; 4800];
    rand::thread_rng().fill(&mut nums[..]);
    let mut list = LinkedList::new();

    for num in &nums[..] {
        let mut cursor = list.cursor_front_mut();

        loop {
            match cursor.current() {
                Some(n) => {
                    if num > n {
                        cursor.insert_before(*num);
                        break
                    }
                    cursor.move_next();
                },
                None => {
                    cursor.insert_before(*num);
                    break
                }
            }
        }
        if list.len() > 10 {
            list.cursor_back_mut().remove_current();
        }
    }
    println!("{:?}", list)
}
