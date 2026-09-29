use adder::add;

#[test]
fn it_adds_two() {
    let result = add(2, 3);
    assert_eq!(result, 5);
}