#[no_mangle]
pub fn two_plus(x: u8) -> u8 {
    return x + 2;
}

#[test]
fn it_works() {
    let x = two_plus(2);
    assert_eq!(x, 4);
}
