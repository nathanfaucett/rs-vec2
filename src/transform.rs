#[cfg(test)]
use core::f32::consts::FRAC_PI_2;

use vec3;
use num::Num;
use length::{length, dot, length_values};


#[inline(always)]
pub fn transform_angle<'a, 'b, T: Num>(out: &'a mut [T; 2], a: &'b [T; 2], angle: T) -> &'a mut [T; 2] {
    let c = angle.cos();
    let s = angle.sin();

    out[0] = a[0] * c - a[1] * s;
    out[1] = a[0] * s + a[1] * c;
    out
}

#[inline(always)]
pub fn transform_mat2<'a, 'b, T: Num>(out: &'a mut [T; 2], a: &'b [T; 2], m: &'b [T; 4]) -> &'a mut [T; 2] {
    out[0] = a[0] * m[0] + a[1] * m[2];
    out[1] = a[0] * m[1] + a[1] * m[3];
    out
}

#[inline(always)]
pub fn transform_mat32<'a, 'b, T: Num>(out: &'a mut [T; 2], a: &'b [T; 2], m: &'b [T; 6]) -> &'a mut [T; 2] {
    out[0] = a[0] * m[0] + a[1] * m[2] + m[4];
    out[1] = a[0] * m[1] + a[1] * m[3] + m[5];
    out
}

#[inline(always)]
pub fn transform_mat3<'a, 'b, T: Num>(out: &'a mut [T; 2], a: &'b [T; 2], m: &'b [T; 9]) -> &'a mut [T; 2] {
    out[0] = a[0] * m[0] + a[1] * m[3];
    out[1] = a[0] * m[1] + a[1] * m[4];
    out
}

#[inline(always)]
pub fn transform_mat4<'a, 'b, T: Num>(out: &'a mut [T; 2], a: &'b [T; 2], m: &'b [T; 16]) -> &'a mut [T; 2] {
    out[0] = a[0] * m[0] + a[1] * m[4] + m[12];
    out[1] = a[0] * m[1] + a[1] * m[5] + m[13];
    out
}

#[inline(always)]
pub fn transform_projection<'a, 'b, T: Num>(out: &'a mut [T; 2], a: &'b [T; 2], m: &'b [T; 16]) -> &'a mut [T; 2] {
    let mut d = a[0] * m[3] + a[1] * m[7] + m[11] + m[15];
    d = if d != T::zero() {T::one() / d} else {d};

    out[0] = (a[0] * m[0] + a[1] * m[4] + m[8] + m[12]) * d;
    out[1] = (a[0] * m[1] + a[1] * m[5] + m[9] + m[13]) * d;
    out
}

#[inline(always)]
pub fn transform_projection_no_position<'a, 'b, T: Num>(out: &'a mut [T; 2], a: &'b [T; 2], m: &'b [T; 16]) -> &'a mut [T; 2] {
    let mut d = a[0] * m[3] + a[1] * m[7] + m[11] + m[15];
    d = if d != T::zero() {T::one() / d} else {d};

    out[0] = (a[0] * m[0] + a[1] * m[4] + m[8]) * d;
    out[1] = (a[0] * m[1] + a[1] * m[5] + m[9]) * d;
    out
}

#[inline(always)]
pub fn position_mat32<'a, 'b, T: Num>(out: &'a mut [T; 2], m: &'b [T; 6]) -> &'a mut [T; 2] {
    out[0] = m[4];
    out[1] = m[5];
    out
}

#[inline(always)]
pub fn position_mat4<'a, 'b, T: Num>(out: &'a mut [T; 2], m: &'b [T; 16]) -> &'a mut [T; 2] {
    out[0] = m[12];
    out[1] = m[13];
    out
}

#[inline(always)]
pub fn scale_mat2<'a, 'b, T: Num>(out: &'a mut [T; 2], m: &'b [T; 4]) -> &'a mut [T; 2] {
    out[0] = length_values(m[0], m[2]);
    out[1] = length_values(m[1], m[3]);
    out
}

#[inline(always)]
pub fn scale_mat32<'a, 'b, T: Num>(out: &'a mut [T; 2], m: &'b [T; 6]) -> &'a mut [T; 2] {
    out[0] = length_values(m[0], m[2]);
    out[1] = length_values(m[1], m[3]);
    out
}

#[inline(always)]
pub fn scale_mat3<'a, 'b, T: Num>(out: &'a mut [T; 2], m: &'b [T; 9]) -> &'a mut [T; 2] {
    out[0] = vec3::length_values(m[0], m[3], m[6]);
    out[1] = vec3::length_values(m[1], m[4], m[7]);
    out
}

#[inline(always)]
pub fn scale_mat4<'a, 'b, T: Num>(out: &'a mut [T; 2], m: &'b [T; 16]) -> &'a mut [T; 2] {
    out[0] = vec3::length_values(m[0], m[4], m[8]);
    out[1] = vec3::length_values(m[1], m[5], m[9]);
    out
}

#[inline(always)]
pub fn angle<'a, T: Num>(a: &'a [T; 2], b: &'a [T; 2]) -> T {
    (dot(a, b) / (length(a) * length(b))).acos()
}
#[test]
fn angle_test() {
    let rad = angle(&[0f32, 1f32], &[1f32, 0f32]);
    assert_eq!(rad, FRAC_PI_2);
}
