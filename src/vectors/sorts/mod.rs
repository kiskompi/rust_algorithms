mod insertion;
mod merge;
mod quick;

/// This function runs the instertion sort algorith on the vector given as a parameter and returns the ordered version of the same vector.
/// The parameters are:
/// * vector: Vec<i32> - it is the vector we want to sort
/// * increasing: bool - if this value is true, the elements of the vector will be sorted in increasing order.
/// # Example
///
/// fn insertion_sort_test() {
///     let size: usize = 30;
///     let vector = vec![10, 30, 40, 20, 60, 90];
///     let sorted_vector = algorithms::vectors::sorts::insertion_sort(vector, true);
/// }
pub fn insertion_sort(mut vector: Vec<i32>, increasing: bool) -> Vec<i32> {
    for i in 0..vector.len() {
        vector = insertion::back_propagate(vector, i + 1, increasing);
    }
    return vector;
}

pub fn merge_sort(vector: Vec<i32>) -> Vec<i32> {
    let subvectors: Vec< Vec<i32> > = merge::divide(vector);
    return  merge::merge(subvectors);
}

pub fn heapsort(){

}

pub fn quick_sort() {}
