fn main() {
    let x:u8 = 1;
    let y = x | 0;
    let z = y * 1;

    assert!(z == 1)
}