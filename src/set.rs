use num::Num;


#[inline(always)]
pub fn set<T: Num>(out: &mut [T; 2], x: T, y: T) -> &mut [T; 2] {
    out[0] = x;
    out[1] = y;
    out
}
#[test]
fn test_set() {
    let mut v = [0, 0];
    set(&mut v, 1, 2);
    assert!(v == [1, 2]);
}

#[inline(always)]
pub fn zero<T: Num>(out: &mut [T; 2]) -> &mut [T; 2] { set(out, T::zero(), T::zero()) }
#[inline(always)]
pub fn identity<T: Num>(out: &mut [T; 2]) -> &mut [T; 2] { set(out, T::zero(), T::zero()) }
#[inline(always)]
pub fn up<T: Num>(out: &mut [T; 2]) -> &mut [T; 2] { set(out, T::zero(), T::one()) }
#[inline(always)]
pub fn down<T: Num>(out: &mut [T; 2]) -> &mut [T; 2] { set(out, T::zero(), -T::one()) }
#[inline(always)]
pub fn left<T: Num>(out: &mut [T; 2]) -> &mut [T; 2] { set(out, -T::one(), T::zero()) }
#[inline(always)]
pub fn right<T: Num>(out: &mut [T; 2]) -> &mut [T; 2] { set(out, T::one(), T::zero()) }
