extern crate anima;

#[test]
fn test_addition() {
    let v1 = anima::math::Vector::new_unf(1.0);
    let v2 = anima::math::Vector::new_unf(2.0);

    assert_eq!(v1 + v2, anima::math::Vector { x: 3.0, y: 3.0, z: 3.0 });
}
