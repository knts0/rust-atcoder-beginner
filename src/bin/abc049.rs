use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let length = s.len();
    let s: String = s.chars().into_iter().rev().collect();
    let mut i = 0;

    while i < length {
        if i + 4 < length && (&s[i..=(i + 4)] == "maerd" || &s[i..=(i + 4)] == "esare") {
            i += 5;
            continue;
        }

        if i + 6 < length && &s[i..=(i + 6)] == "remaerd" {
            i += 7;
            continue;
        }

        if i + 5 < length && &s[i..=(i + 5)] == "resare" {
            i += 6;
            continue;
        }
        println!("NO");
        return;
    }

    println!("YES");
}