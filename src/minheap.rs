
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Vertex<'a>{
    pub name: &'a str, 
    pub distance: usize,
}

impl<'a> Vertex<'a> {
    fn cmp_key(&self) -> (usize, &str) {
        (self.distance, &self.name)
    }
}

impl<'a> Ord for Vertex<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_key().cmp(&other.cmp_key())
    }
}

impl<'a> PartialOrd for Vertex<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct MinHeap<'a> {
    heap: Vec<Vertex<'a>>,
}

impl<'a> MinHeap<'a> {

    //Initialises a minimum heap
    pub fn init() -> MinHeap<'a> {
        MinHeap {
            // heap: Vec::<T>::new(),
            heap: Vec::new(),
        }
    }

    //Inserts a value into the heap and maintans heap property
    pub fn insert(&mut self, vertex: Vertex<'a>) {
        self.heap.push(vertex);
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
            } else {
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
            } else {
                break;
            }
        }
    }

    pub fn remove_min(&mut self) -> Option<Vertex<'a>> {
        if self.heap.len() == 0 {
            None
        } else {
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
