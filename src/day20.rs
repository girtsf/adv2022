use adv2022::read_input;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node {
    n: isize,
    // Mostly doing the Rc/Weak + RefCell as an exercise. Would have been easier to just use a
    // usize index into the "nodes" vector.
    prev: Option<Weak<RefCell<Node>>>,
    next: Option<Weak<RefCell<Node>>>,
}

#[derive(Debug)]
struct ReorgList {
    nodes: Vec<Rc<RefCell<Node>>>,
    first: Weak<RefCell<Node>>,
}

impl ReorgList {
    fn parse(input: &str, decryption_key: isize) -> ReorgList {
        let nodes: Vec<_> = input
            .lines()
            .map(|line| {
                let n = line.trim().parse::<isize>().unwrap() * decryption_key;
                Rc::new(RefCell::new(Node {
                    n,
                    prev: None,
                    next: None,
                }))
            })
            .collect();

        for i in 1..nodes.len() {
            nodes[i].borrow_mut().prev = Some(Rc::downgrade(&nodes[i - 1]));
        }
        nodes[0].borrow_mut().prev = Some(Rc::downgrade(nodes.last().unwrap()));

        for i in 0..nodes.len() - 1 {
            nodes[i].borrow_mut().next = Some(Rc::downgrade(&nodes[i + 1]));
        }
        nodes.last().unwrap().borrow_mut().next = Some(Rc::downgrade(&nodes[0]));

        ReorgList {
            first: Rc::downgrade(&nodes[0]),
            nodes,
        }
    }

    fn find_by_value(&self, value: isize) -> Rc<RefCell<Node>> {
        // Assumes the value exists.
        let mut cur_weak = self.first.clone();
        loop {
            let cur_rc = cur_weak.upgrade().unwrap();
            let cur = cur_rc.borrow();
            if cur.n == value {
                return cur_rc.clone();
            }
            cur_weak = cur_rc.borrow().next.clone().unwrap();
            if cur_weak.ptr_eq(&self.first) {
                panic!("failed to find {}", value);
            }
        }
    }

    fn skip(&self, node: &Rc<RefCell<Node>>, mut i: usize) -> Rc<RefCell<Node>> {
        i %= self.nodes.len();
        let mut cur = node.clone();
        for _ in 0..i {
            // Without splitting it apart, Rust thinks that the value gets dropped before it gets
            // used??
            let cur1 = cur.borrow().next.as_ref().unwrap().upgrade().unwrap();
            cur = cur1.clone();
        }
        cur
    }

    fn dump_list(&self) {
        let mut cur_weak = self.first.clone();
        println!("------------------------");
        loop {
            let cur_rc = cur_weak.upgrade().unwrap();
            {
                let cur = cur_rc.borrow();
                println!(
                    "n: {} next: {} prev: {}",
                    cur.n,
                    cur.next.as_ref().unwrap().upgrade().unwrap().borrow().n,
                    cur.prev.as_ref().unwrap().upgrade().unwrap().borrow().n,
                );
            }

            cur_weak = cur_rc.borrow().next.clone().unwrap();
            if cur_weak.ptr_eq(&self.first) {
                break;
            }
        }
        println!("------------------------");
    }

    /// Moves the node left or right in the linked list, based on its n.
    fn move_node(&mut self, i: usize) {
        let node_weak_ptr = Rc::downgrade(&self.nodes[i]);
        let mut node = self.nodes[i].borrow_mut();
        let mut n = node.n % (self.nodes.len() as isize - 1);
        while n != 0 {
            // Ugh.
            // Also, this does not handle tiny lists.
            let prev_rc = node.prev.as_ref().unwrap().upgrade().unwrap();
            let mut prev = prev_rc.borrow_mut();
            let next_rc = node.next.as_ref().unwrap().upgrade().unwrap();
            let mut next = next_rc.borrow_mut();
            let next_next_rc = next.next.as_ref().unwrap().upgrade().unwrap();
            let mut next_next = next_next_rc.borrow_mut();
            let prev_prev_rc = prev.prev.as_ref().unwrap().upgrade().unwrap();
            let mut prev_prev = prev_prev_rc.borrow_mut();

            // Move right. Inefficiently.
            if n > 0 {
                prev.next = node.next.clone();
                next.prev = node.prev.clone();
                next_next.prev = Some(node_weak_ptr.clone());
                next.next = Some(node_weak_ptr.clone());

                node.next = Some(Rc::downgrade(&next_next_rc));
                node.prev = Some(Rc::downgrade(&next_rc));

                if self.first.ptr_eq(&node_weak_ptr) {
                    self.first = Rc::downgrade(&next_rc);
                }
                n -= 1;
            }
            // Move left. Inefficiently.
            if n < 0 {
                next.prev = node.prev.clone();
                prev.next = node.next.clone();
                prev_prev.next = Some(node_weak_ptr.clone());
                prev.prev = Some(node_weak_ptr.clone());

                node.prev = Some(Rc::downgrade(&prev_prev_rc));
                node.next = Some(Rc::downgrade(&prev_rc));

                if self.first.ptr_eq(&node_weak_ptr) {
                    self.first = Rc::downgrade(&prev_rc);
                }
                n += 1;
            }
        }
    }

    fn mix(&mut self) {
        for i in 0..self.nodes.len() {
            self.move_node(i);
        }
    }
}

fn main() {
    {
        // Part 1:
        let mut list = ReorgList::parse(&read_input(), 1);
        // dbg!(&list);
        list.dump_list();
        list.mix();
        list.dump_list();
        let zero_node = list.find_by_value(0);
        // dbg!(&zero_node);
        let part1 = (0..3)
            .fold((0isize, zero_node), |(sum, node), _| {
                let n1000 = list.skip(&node, 1000);
                let new_sum = sum + n1000.borrow().n;
                (new_sum, n1000)
            })
            .0;
        dbg!(&part1);
    }
    {
        // Part 2:
        let mut list = ReorgList::parse(&read_input(), 811589153);
        // dbg!(&list);
        list.dump_list();
        for _ in 0..10 {
            list.mix();
        }
        list.dump_list();
        let zero_node = list.find_by_value(0);
        // dbg!(&zero_node);
        let part2 = (0..3)
            .fold((0isize, zero_node), |(sum, node), _| {
                let n1000 = list.skip(&node, 1000);
                let new_sum = sum + n1000.borrow().n;
                (new_sum, n1000)
            })
            .0;
        dbg!(&part2);
    }
}
