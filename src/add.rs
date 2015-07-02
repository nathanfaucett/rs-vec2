use num::Num;


#[inline(always)]
pub fn add<T: Num>(out: &mut [T; 2], a: [T; 2], b: [T; 2]) ->  &mut [T; 2] {
    out[0] = a[0] + b[0];
    out[1] = a[1] + b[1];
    out
}
#[test]
fn test_add() {
    let mut v = [0, 0];
    add(&mut v, [1, 1], [1, 1]);
    assert!(v[0] == 2);
    assert!(v[1] == 2);
}

#[inline(always)]
pub fn sadd<T: Num>(out: &mut [T; 2], a: [T; 2], s: T) ->  &mut [T; 2] {
    out[0] = a[0] + s;
    out[1] = a[1] + s;
    out
}
#[test]
fn test_sadd() {
    let mut v = [0, 0];
    sadd(&mut v, [0, 0], 1);
    assert!(v[0] == 1);
    assert!(v[1] == 1);
}
