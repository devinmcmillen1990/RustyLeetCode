pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let n = nums.len();
    let mut result = vec![nums.clone()];
    let mut c = vec![0; n];
    let mut i = 0;

    while i < n {
        if c[i] < i {
            if i % 2 == 0 {
                nums.swap(0, i);
            } else {
                nums.swap(c[i], i);
            }

            result.push(nums.clone());
            
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }

    result
}
