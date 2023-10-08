// floyds_algorithm.rs

// use std::collections::linked_list;

use crate::data_structures::linked_list::LinkedList; // Import the LinkedList from linked_list.rs

// Define a function to detect a cycle in a linked list using Floyd's Tortoise and Hare Algorithm
pub fn detect_cycle<T>(linked_list: &mut LinkedList<T>) -> Option<usize> {
    let mut tortoise = linked_list.head;
    let mut hare = linked_list.head;
    let mut steps = 0;

    while tortoise.is_some() && hare.is_some() {
        tortoise = tortoise.unwrap().next;
        hare = hare.unwrap().next;

        if hare.is_some() {
            hare = hare.unwrap().next;
        } else {
            return None; // No cycle found
        }

        steps += 1;

        if tortoise == hare {
            // Cycle detected, find the start of the cycle
            tortoise = linked_list.get;
            while tortoise != hare {
                tortoise = tortoise.unwrap().next;
                hare = hare.unwrap().next;
            }
            return Some(steps);
        }
    }

    None // No cycle found
}

// Main function for testing...
fn main() {
    // Create a linked list and add elements (you can use functions from linked_list.rs)
    let mut linked_list = LinkedList::new();
    linked_list.insert_at_tail(1);
    linked_list.insert_at_tail(2);
    linked_list.insert_at_tail(3);
    
    unsafe {
        // Create a cycle for testing (optional)
       if let Some(mut tail) = linked_list.tail {
            if let Some(mut head) = linked_list.head {
                tail.as_mut().next = Some(head);
            }
        }
    }

    // Detect a cycle and print the result
    if let Some(steps) = detect_cycle(&mut linked_list) {
        println!("Cycle found at step: {}", steps);
    } else {
        println!("No cycle found.");
    }
}
