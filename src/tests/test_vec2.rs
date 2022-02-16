use crate::vector::Vec2;

#[test]
fn test_add() {
    let result = Vec2::new(2, 2) + Vec2::new(3, 4);
    assert_eq!(result.x(), 5);
    assert_eq!(result.y(), 6);
}