// Brute Force	    O(n log n)	    O(n)	    {Use count_ones()}
pub fn count_bits(n: i32) -> Vec<i32> {
    (0..=n).map(|x| x.count_ones() as i32).collect()
}
