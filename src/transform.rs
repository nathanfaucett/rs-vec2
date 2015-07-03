extern crate vec3;

use num::Num;
use length::length_values;


#[inline(always)]
pub fn transform_angle<T: Num>(out: &mut [T; 2], a: [T; 2], angle: T) -> &mut [T; 2] {
    let c = angle.cos();
    let s = angle.sin();

    out[0] = a[0] * c - a[1] * s;
    out[1] = a[0] * s + a[1] * c;
    out
}

#[inline(always)]
pub fn transform_mat2<T: Num>(out: &mut [T; 2], a: [T; 2], m: [T; 4]) -> &mut [T; 2] {
    out[0] = a[0] * m[0] + a[1] * m[2];
    out[1] = a[0] * m[1] + a[1] * m[3];
    out
}

#[inline(always)]
pub fn transform_mat32<T: Num>(out: &mut [T; 2], a: [T; 2], m: [T; 6]) -> &mut [T; 2] {
    out[0] = a[0] * m[0] + a[1] * m[2] + m[4];
    out[1] = a[0] * m[1] + a[1] * m[3] + m[5];
    out
}

#[inline(always)]
pub fn transform_mat3<T: Num>(out: &mut [T; 2], a: [T; 2], m: [T; 9]) -> &mut [T; 2] {
    out[0] = a[0] * m[0] + a[1] * m[3] + m[6];
    out[1] = a[0] * m[1] + a[1] * m[4] + m[7];
    out
}

#[inline(always)]
pub fn transform_mat4<T: Num>(out: &mut [T; 2], a: [T; 2], m: [T; 16]) -> &mut [T; 2] {
    out[0] = a[0] * m[0] + a[1] * m[4] + m[12];
    out[1] = a[0] * m[1] + a[1] * m[5] + m[13];
    out
}

#[inline(always)]
pub fn transform_projection<T: Num>(out: &mut [T; 2], a: [T; 2], m: [T; 16]) -> &mut [T; 2] {
    let mut d = a[0] * m[3] + a[1] * m[7] + m[11] + m[15];
    d = if d != T::zero() {T::one() / d} else {d};

    out[0] = (a[0] * m[0] + a[1] * m[4] + m[12]) * d;
    out[1] = (a[0] * m[1] + a[1] * m[5] + m[13]) * d;
    out
}

#[inline(always)]
pub fn position_mat32<T: Num>(out: &mut [T; 2], m: [T; 6]) -> &mut [T; 2] {
    out[0] = m[4];
    out[1] = m[5];
    out
}

#[inline(always)]
pub fn position_mat4<T: Num>(out: &mut [T; 2], m: [T; 16]) -> &mut [T; 2] {
    out[0] = m[12];
    out[1] = m[13];
    out
}

#[inline(always)]
pub fn scale_mat2<T: Num>(out: &mut [T; 2], m: [T; 4]) -> &mut [T; 2] {
    out[0] = length_values(m[0], m[2]);
    out[1] = length_values(m[1], m[3]);
    out
}

#[inline(always)]
pub fn scale_mat32<T: Num>(out: &mut [T; 2], m: [T; 6]) -> &mut [T; 2] {
    out[0] = length_values(m[0], m[2]);
    out[1] = length_values(m[1], m[3]);
    out
}

#[inline(always)]
pub fn scale_mat3<T: Num>(out: &mut [T; 2], m: [T; 9]) -> &mut [T; 2] {
    out[0] = vec3::length_values(m[0], m[3], m[6]);
    out[1] = vec3::length_values(m[1], m[4], m[7]);
    out
}

#[inline(always)]
pub fn scale_mat4<T: Num>(out: &mut [T; 2], m: [T; 16]) -> &mut [T; 2] {
    out[0] = vec3::length_values(m[0], m[4], m[8]);
    out[1] = vec3::length_values(m[1], m[5], m[9]);
    out
}
