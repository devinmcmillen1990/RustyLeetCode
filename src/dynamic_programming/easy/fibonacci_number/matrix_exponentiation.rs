fn multiply(a: [[i32; 2]; 2], b: [[i32; 2]; 2]) -> [[i32; 2]; 2] {
    [
        [
            a[0][0] * b[0][0] + a[0][1] * b[1][0],
            a[0][0] * b[0][1] + a[0][1] * b[1][1],
        ],
        [
            a[1][0] * b[0][0] + a[1][1] * b[1][0],
            a[1][0] * b[0][1] + a[1][1] * b[1][1],
        ],
    ]
}

fn matrix_pow(mut mat: [[i32; 2]; 2], mut n: i32) -> [[i32; 2]; 2] {
    let mut res = [[1, 0], [0, 1]]; // Identity
    while n > 0 {
        if n % 2 == 1 {
            res = multiply(res, mat);
        }
        mat = multiply(mat, mat);
        n /= 2;
    }
    res
}

pub fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    let base = [[1, 1], [1, 0]];
    let result = matrix_pow(base, n - 1);
    result[0][0]
}
