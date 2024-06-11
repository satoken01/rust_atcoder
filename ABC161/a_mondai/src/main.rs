fn main() {
    proconio::input! {
        x: i32,
        y: i32,
        z: i32
    }

    let mut new_x: i32 = y;
    let new_y: i32 = x;
    let new_z: i32 = new_x;
    new_x = z;

    println!("{} {} {}",new_x,new_y,new_z);
}
