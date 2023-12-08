fn main() {
    assert_eq!(11, find_gcd_euclid_algo(22, 55));
}

fn find_gcd_euclid_algo(mut m: usize, mut n: usize) -> usize {
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }

    n
}
