/// This is a recursive function, which iterates through the elements of the vector and propagates them back while the previous element is not equal or less than them.
pub fn back_propagate(mut array: Vec<i32>, current: usize, inc_ord: bool) -> Vec<i32> {
    if array.get(current - 1) != None &&
       (array.get(current - 1) >= array.get(current) && inc_ord == false) ||
       (array.get(current - 1) <= array.get(current) && inc_ord == true) {

       let tmp_current = array[current];
       let tmp_cmo = array[current - 1];

       array[current] = tmp_cmo;
       array[current - 1] = tmp_current;

       if current > 1 {
           array = back_propagate(array, current - 1, inc_ord);
       }
    }
    return array;
}
