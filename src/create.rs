use num::Num;


#[inline(always)]
pub fn create<T: Num>(x: T, y: T) -> [T; 2] {[x, y]}
#[test]
fn test_create() {
    let v = create(1, 2);
    assert!(v[0] == 1);
    assert!(v[1] == 2);
}

#[inline(always)]
pub fn zero<T: Num>() -> [T; 2] {create(T::zero(), T::zero())}
#[inline(always)]
pub fn up<T: Num>() -> [T; 2] {create(T::zero(), T::one())}
#[inline(always)]
pub fn down<T: Num>() -> [T; 2] {create(T::zero(), -T::one())}
#[inline(always)]
pub fn right<T: Num>() -> [T; 2] {create(T::one(), T::zero())}
#[inline(always)]
pub fn left<T: Num>() -> [T; 2] {create(-T::one(), T::zero())}

#[inline(always)]
pub fn copy<T: Num>(v: [T; 2]) -> [T; 2] {create(v[0], v[1])}
#[inline(always)]
pub fn clone<T: Num>(v: [T; 2]) -> [T; 2] {create(v[0], v[1])}
