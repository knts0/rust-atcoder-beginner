use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [(i32, i32, i32); n],
    }

    let (mut pt, mut px, mut py) = (0, 0, 0);
    for i in 0..n {
        let next = v[i];
        let t_diff = next.0 - pt;
        let distance = (next.1 - px).abs() + (next.2 - py).abs();

        if distance > t_diff || distance % 2 != t_diff % 2 {
            println!("No");
            return;
        }

        pt = next.0;
        px = next.1;
        py = next.2;
    }

    println!("Yes");
}