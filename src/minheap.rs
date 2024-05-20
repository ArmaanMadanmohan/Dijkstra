
// use std::cmp::Ord;

#[derive(Debug)]
pub struct MinHeap<T: Ord> {
    heap: Vec<T>,
}

impl<T:Ord + Clone> MinHeap<T> {

    //Initialises a minimum heap
    fn init() -> MinHeap<T> {
        MinHeap {
            // heap: Vec::<T>::new(),
            heap: Vec::new(),
        }
    }

    //Inserts a value into the heap and maintans heap property
    fn insert(&mut self, value: T) {
        self.heap.push(value);
        self.heapify_up();
    }

    //Starts from the last node and swaps with its parent if it is smaller, continuing upwards
    fn heapify_up(&mut self) {
        let mut i = self.heap.len() - 1;
        while i > 0 {
            let parent = (i - 1) /2;
            if self.heap[i] < self.heap[parent] {
                self.heap.swap(i, parent);
                i = parent;
            }
            else {
                break;
            }
        }
    }

    //Starts from the root node (index 0) and swaps with its child if it is greater, propagating "downwards"
    fn heapify_down(&mut self) {
        let mut i = 0;
        while i < self.heap.len() {
            let left_child = 2 * i + 1;
            let right_child = 2 * i + 2;
            let mut smallest = i;
            if left_child < self.heap.len() && self.heap[left_child] < self.heap[smallest] {
                smallest = left_child;
            }
            if right_child < self.heap.len() && self.heap[right_child] < self.heap[smallest] {
                smallest = right_child;
            }
            if smallest != i {
                self.heap.swap(i, smallest);
                i = smallest;
            }
            else {
                break;
            }
        }
    }

    fn remove_min(&mut self) -> Option<T> {
        println!("{:?}",self.heap.len());
        if self.heap.len() == 0 {
            None
        }
        else {
            let last_idx = self.heap.len() - 1;
            self.heap.swap(0, last_idx);
            let min = self.heap.pop()?;
            if !self.heap.is_empty() {
                self.heapify_down();
            }
            Some(min)
        }  
    } 

}

fn main() {
    let mut heap = MinHeap::init();
    heap.insert(3);
    heap.insert(1);
    heap.insert(4);
    heap.insert(5);

    println!("{:?}", heap.remove_min());
    println!("{:?}", heap.remove_min());
    println!("{:?}", heap.remove_min());
    println!("{:?}", heap.remove_min());
}
