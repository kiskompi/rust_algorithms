struct MaxHeap{
    array: Vec<i32>
}

impl MaxHeap{
    fn heapify(&mut self, index: usize, length: usize){
        let left = self.left_child(0);
        let right = self.right_child(0);
        let mut max;

        let cond1 = if left < length {true}
                    else {false};
        let cond2 = if self.array[left] > self.array[index] {true}
                    else {false};


        if cond1 && cond2 {
            max = left;
        } else {
            max = index;
        }

        let cond3 = if self.array[right] > self.array[max] {true}
                    else {false};

        if cond1 && cond3 {
            max = right;
        }
        if max != index {
            self.swap(index, max);
            self.heapify(max, length);
        }

    }

    pub fn build(array_in: Vec<i32>) -> MaxHeap{
        let mut length = array_in.len();
        let mut heap = MaxHeap{array: array_in};
        while length > 0 {
            heap.heapify(1, length);
            length = length - 1;
        }
        return heap;
    }


    pub fn insert(&mut self, new_child:i32){
        //TODO
        self.array.append(&mut vec![new_child]);
    }

    pub fn extract_max(&mut self) -> i32 {
        if *&self.len() < 1 {
            // TODO with exception
            print!("HEAP UNDERFLOW");
        }
        let max = self.array[0];

        let mut i: usize = 1;
        while i < *&self.array.len() {
            self.array[i-1] = self.array[i];
            i = i + 1;
        }
        return max;
    }

    pub fn increase_key(&mut self, index: usize, new_key: i32){
        //TODO
        if &self.array[index] > &new_key {
            // TODO with error handling
            print!("new key is smaller than the old!");
        } else {
            self.array[index] = new_key;
            let i = index;
            let cond = &self.array[MaxHeap::parent(i)] < &self.array[index];
            while i > 1 && cond {
                let i = MaxHeap::parent(i);
            }
            //TODO i-ik helyre berakni a new_key-t
            
        }
    }

    pub fn maximum(&self) -> i32 {
        //TODO
        self.array[0]
    }

    #[inline]
    fn right_child(&self, current: usize) -> usize{
    // Similarly, the RIGHT procedure can quickly compute 2i + 1 by shifting the binary representation of i left one bit position and adding in a 1 as the low-order bit.
        return current/2;
    }

    #[inline]
    fn left_child(&self, current: usize) -> usize{
    // On most computers, the LEFT procedure can compute 2i in one instruction by simply shifting the binary representation of i left one bit position.
        return 2*current;
    }

    #[inline]
    fn parent(current: usize) -> usize{
    // The PARENT procedure can compute i/2 by shifting i right one bit position.
        return 2*current+1;
    }

    #[inline]
    fn swap(&mut self, a: usize, b:usize){
        self.array.swap(a,b);
    }

    fn len(&self) -> usize {
        self.array.len()
    }
}

pub fn heapsort(array_in: Vec<i32>)-> Vec<i32>{
    //TODO
    let mut heap = MaxHeap::build(array_in);
    let length = heap.len();
    let i = 0;
    while i > 1{
        heap.swap(1,i);
        heap.heapify(1, length);
    }
    return heap.array;
}
