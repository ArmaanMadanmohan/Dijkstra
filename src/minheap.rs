use std::mem;

pub struct MinHeap<T: Ord> {
    heap: Vec<T>,
}

impl <T> MinHeap<T: Ord> {

    //Initialises a minimum heap
    fn init() -> MinHeap<T> {
        MinHeap {
            heap: Vec::new(),
        }
    }

    //Inserts a value into the heap and maintans heap property
    fn insert(&mut self, value: T) {
        self.heap.push(value);
        self.heap.heapify_up();
    }

    //Starts from the last node and swaps with its parent if it is smaller, continuing upwards
    fn heapify_up(&mut self) {
        let mut i = self.heap.len() - 1;
        while i > 0 {
            let parent = (i - 1) /2;
            if self.heap[i] < self.heap[parent] {
                self.heap.swap(&mut i, &mut parent);
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
                self.heap.swap(&mut i, &mut smallest);
                i = smallest;
            }
            else {
                break;
            }
        }
    }

    fn remove_min(&mut self, index: usize) -> Option<T> {
        if self.heap.len() == 0 {
            None
        }
        else {
            let min = self.heap[0];
            self.heap[0] = self.heap.pop();
            let mut i = 0;
            while (self.heap[i] * )
        }  
    } 

}