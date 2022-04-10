use proconio::input;

fn main() {
    input! {
        n: i32,
        y: i32,
    }

    for a in 0..=n {
        for b in 0..=(n - a) {
            let c = n - a - b;
            let sum = a * 10000 + b * 5000 + c * 1000;

            if sum == y {
                println!("{} {} {}", a, b, c);
                return;
            }
        }
    }

    println!("-1 -1 -1");
}