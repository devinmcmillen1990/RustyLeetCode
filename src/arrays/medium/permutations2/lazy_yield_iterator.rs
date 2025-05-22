use std::collections::HashSet;

pub struct UniquePermutations {
    nums: Vec<i32>,
    c: Vec<usize>,
    i: usize,
    first: bool,
    seen: HashSet<Vec<i32>>,
}

impl UniquePermutations {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut seen = HashSet::new();
        seen.insert(nums.clone());
        let nums_len = nums.len();

        Self {
            nums,
            c: vec![0; nums_len],
            i: 0,
            first: true,
            seen,
        }
    }
}

impl Iterator for UniquePermutations {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(self.nums.clone());
        }

        let n = self.nums.len();
        while self.i < n {
            if self.c[self.i] < self.i {
                if self.i % 2 == 0 {
                    self.nums.swap(0, self.i);
                } else {
                    self.nums.swap(self.c[self.i], self.i);
                }

                let candidate = self.nums.clone();
                self.c[self.i] += 1;
                self.i = 0;

                if self.seen.insert(candidate.clone()) {
                    return Some(candidate);
                }
            } else {
                self.c[self.i] = 0;
                self.i += 1;
            }
        }

        None
    }
}
