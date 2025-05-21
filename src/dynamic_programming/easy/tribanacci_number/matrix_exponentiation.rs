pub fn tribonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 || n == 2 {
        return 1;
    }

    let base = [[1, 1, 1], [1, 0, 0], [0, 1, 0]];
    let res = mat_pow(base, n - 2);
    (res[0][0] + res[0][1]) as i32 // res * [1,1,0]^T
}

fn mat_mul(a: &[[i64; 3]; 3], b: &[[i64; 3]; 3]) -> [[i64; 3]; 3] {
    let mut res = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                res[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    res
}

fn mat_pow(mut mat: [[i64; 3]; 3], mut exp: i32) -> [[i64; 3]; 3] {
    let mut res = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
    while exp > 0 {
        if exp % 2 == 1 {
            res = mat_mul(&res, &mat);
        }
        mat = mat_mul(&mat, &mat);
        exp /= 2;
    }
    res
}
