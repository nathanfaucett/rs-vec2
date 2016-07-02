use num::Num;


#[inline(always)]
pub fn set<'a, T: Num>(out: &'a mut [T; 2], x: T, y: T) -> &'a mut [T; 2] {
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
pub fn zero<'a, T: Num>(out: &'a mut [T; 2]) -> &'a mut [T; 2] { set(out, T::zero(), T::zero()) }
#[inline(always)]
pub fn identity<'a, T: Num>(out: &'a mut [T; 2]) -> &'a mut [T; 2] { set(out, T::zero(), T::zero()) }
#[inline(always)]
pub fn up<'a, T: Num>(out: &'a mut [T; 2]) -> &'a mut [T; 2] { set(out, T::zero(), T::one()) }
#[inline(always)]
pub fn down<'a, T: Num>(out: &'a mut [T; 2]) -> &'a mut [T; 2] { set(out, T::zero(), -T::one()) }
#[inline(always)]
pub fn left<'a, T: Num>(out: &'a mut [T; 2]) -> &'a mut [T; 2] { set(out, -T::one(), T::zero()) }
#[inline(always)]
pub fn right<'a, T: Num>(out: &'a mut [T; 2]) -> &'a mut [T; 2] { set(out, T::one(), T::zero()) }
