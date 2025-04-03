mod instructions;
mod push;

fn main() {
    let arr: Vec<i32> = vec![];
    let slice = &arr[..2];
    println!("{:?}", slice);
}
