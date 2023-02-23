fn main() {
    let limit: u64 = std::env::args().nth(1).unwrap().parse().unwrap();
    let iter = 1000;

    let result = (1..limit)
        .into_par_iter()
        .map(|n| {
            let mut c = 0;
            let mut cur = n;
            while c < iter {
                if cur % 2 == 0 {
                    cur /= 2;
                } else {
                    cur = cur * 3 + 1;
                }
                c += 1;
                if cur <= 1 {
                    break;
                }
            }
            (c, n)
        })
        .reduce(|| (0, 0), |a, b| a.max(b));
}
