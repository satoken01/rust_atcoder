use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32
    }
    if a == b {
        println!("Yes");
    } else {
        println!("No");
    }
}
