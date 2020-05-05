#![feature(type_alias_impl_trait)]

#[cutlass::curry]
fn add(x: u32, y: u32, z: u32) -> u32 {
    return x + y + z;
}

#[test]
fn add_works() {
    let plus_3 = add(1)(2);
    let v: Vec<u32> = (1..=3).map(plus_3).collect();
    assert_eq!(v, vec![4, 5, 6]);
}
