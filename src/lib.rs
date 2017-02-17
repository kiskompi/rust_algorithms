//! The 'algorithms' is a crate consisting algorithms and collections based on the book Introduction to Algorithms by
//! * Thomas H. Cormen
//! * Charles E. Leiserson
//! * Ronald L. Rivest and
//! * Clifford Stein
//! Many thanks for their great work!


#![deny(missing_docs,
         missing_debug_implementations, missing_copy_implementations,
         trivial_casts, trivial_numeric_casts,
         unsafe_code,
         unstable_features,
         unused_import_braces, unused_qualifications)]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![allow(dead_code)]

// Building your project with cargo build --features "dev" now automatically
// includes clip­py's lints. (By the way, you only need to allow
// unstable_features if you have pre­viously denyed it, of course.)

extern crate rand;

mod vectors;
mod trees;

#[cfg(test)]
mod tests {
    use super::vectors;
    #[test]
    fn insertion_sort_test() {
        let size: usize = 30;
        let vector = vectors::random(size);
        super::vectors::printv(&vector);
        let sorted_vector = vectors::sorts::insertion_sort(vector, true);
        super::vectors::printv(&sorted_vector);
    }
    #[test]
    fn merge_sort_test(){
        let size: usize = 30;
        let vector = vectors::random(size);
        super::vectors::printv(&vector);
        let sorted_vector = vectors::sorts::merge_sort(vector);
        super::vectors::printv(&sorted_vector);
    }
    #[test]
    fn heap_test() {
        // TODO all function tests
        unimplemented!()
    }
}
