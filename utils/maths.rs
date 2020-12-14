pub fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

pub fn mod_inverse(x: i64, n: i64) -> i64 {
    let (_, x, _) = egcd(x, n);
    (x % n + n) % n
}

/// http://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
pub fn chinese_remainder(pairs: &Vec<(i64, i64)>) -> i64 {
    let product = pairs.iter().map(|(r, _)| r).product::<i64>();

    pairs.iter().fold(0, |acc, (i, j)| {
        let p = product / i;
        acc + j * mod_inverse(p, *i) * p
    }) % product
}
