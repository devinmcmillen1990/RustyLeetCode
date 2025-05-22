pub struct Permutations {
    nums: Vec<i32>,
    c: Vec<usize>,
    i: usize,
    first: bool,
}

impl Permutations {
    pub fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        Self {
            nums,
            c: vec![0; n],
            i: 0,
            first: true,
        }
    }
}

impl Iterator for Permutations {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(self.nums.clone());
        }

        while self.i < self.nums.len() {
            if self.c[self.i] < self.i {
                if self.i % 2 == 0 {
                    self.nums.swap(0, self.i);
                } else {
                    self.nums.swap(self.c[self.i], self.i);
                }
                let result = self.nums.clone();
                self.c[self.i] += 1;
                self.i = 0;
                return Some(result);
            } else {
                self.c[self.i] = 0;
                self.i += 1;
            }
        }
        None
    }
}
