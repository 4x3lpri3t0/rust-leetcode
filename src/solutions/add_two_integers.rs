// https://leetcode.com/problems/add-two-integers/

pub fn sum(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

#[test]
fn test() {
    assert_eq!(2, sum(1, 1));
    assert_eq!(4, sum(2, 2));
    assert_eq!(0, sum(0, 0));
}
