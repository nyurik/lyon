
use std::mem;
use std::ops;
use std::marker::PhantomData;

use vec2::Vector2D;
use vec3::Vector3D;
use vec4::Vector4D;
use common::Untyped;
use constants::*;

pub type Mat4 = Matrix4x4<Untyped>;
pub type Mat3 = Matrix3x3<Untyped>;
pub type Mat2 = Matrix2x2<Untyped>;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Matrix2x2<Unit> {
    pub _11: f32, pub _21: f32,
    pub _12: f32, pub _22: f32,
    _unit: PhantomData<Unit>
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Matrix3x3<Unit> {
    pub _11: f32, pub _21: f32, pub _31: f32,
    pub _12: f32, pub _22: f32, pub _32: f32,
    pub _13: f32, pub _23: f32, pub _33: f32,
    _unit: PhantomData<Unit>
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Matrix4x4<Unit> {
    pub _11: f32, pub _21: f32, pub _31: f32, pub _41: f32,
    pub _12: f32, pub _22: f32, pub _32: f32, pub _42: f32,
    pub _13: f32, pub _23: f32, pub _33: f32, pub _43: f32,
    pub _14: f32, pub _24: f32, pub _34: f32, pub _44: f32,
    _unit: PhantomData<Unit>
}



impl<U> Matrix2x2<U> {
    pub fn new(a11: f32, a21: f32, a12: f32, a22: f32) -> Matrix2x2<U> {
        Matrix2x2 {
            _11: a11, _21: a21,
            _12: a12, _22: a22,
            _unit: PhantomData
        }
    }

    pub fn from_slice(from: &[f32]) -> Matrix2x2<U> {
        assert!(from.len() >= 4);
        return Matrix2x2 {
            _11: from[0], _21: from[1],
            _12: from[2], _22: from[3],
            _unit: PhantomData
        };
    }

    pub fn as_slice<'l>(&'l self) -> &'l [f32] {
        unsafe {
            return mem::transmute((&self._11 as *const f32, 4 as usize ));
        }
    }

    pub fn as_mut_slice<'l>(&'l mut self) -> &'l mut [f32] {
        unsafe {
            return mem::transmute((&mut self._11 as *mut f32, 4 as usize ));
        }
    }

    pub fn row_1<'l>(&'l self) -> &'l Vector2D<U> {
        unsafe { mem::transmute(&self._11 as *const f32) }
    }

    pub fn row_2<'l>(&'l self) -> &'l Vector2D<U> {
        unsafe { mem::transmute(&self._12 as *const f32) }
    }

    #[inline]
    pub fn transform(&self, v: &Vector2D<U>) -> Vector2D<U> {
        Vector2D::new(
            v.x * self._11 + v.y * self._21,
            v.x * self._12 + v.y * self._22
        )
    }

    #[inline]
    pub fn identity() -> Matrix2x2<U> {
        Matrix2x2 {
            _11: 1.0, _21: 0.0,
            _12: 0.0, _22: 1.0,
            _unit: PhantomData
        }
    }

    #[inline]
    pub fn set_indentity(&mut self) {
        self._11 = 1.0; self._21 = 0.0;
        self._12 = 0.0; self._22 = 1.0;
    }
}

#[allow(dead_code)]
impl<U> Matrix3x3<U> {

    pub fn new(
        a11: f32, a21: f32, a31: f32,
        a12: f32, a22: f32, a32: f32,
        a13: f32, a23: f32, a33: f32
    ) -> Matrix3x3<U> {
        Matrix3x3 {
            _11: a11, _21: a21, _31: a31,
            _12: a12, _22: a22, _32: a32,
            _13: a13, _23: a23, _33: a33,
            _unit: PhantomData
        }
    }

    pub fn from_slice(from: &[f32]) -> Matrix3x3<U> {
        assert_eq!(from.len(), 9);
        return Matrix3x3 {
            _11: from[0], _21: from[1], _31: from[2],
            _12: from[3], _22: from[4], _32: from[5],
            _13: from[6], _23: from[7], _33: from[8],
            _unit: PhantomData
        };
    }

    pub fn as_slice<'l>(&'l self) -> &'l [f32] {
        unsafe {
            return mem::transmute((&self._11 as *const f32, 9 as usize ));
        }
    }

    pub fn as_mut_slice<'l>(&'l mut self) -> &'l mut [f32] {
        unsafe {
            return mem::transmute((&mut self._11 as *mut f32, 9 as usize ));
        }
    }

    pub fn transform(&self, p: &Vector3D<U>) -> Vector3D<U> {
        Vector3D::new(
            p.x * self._11 + p.y * self._21 + p.z * self._31,
            p.x * self._12 + p.y * self._22 + p.z * self._32,
            p.x * self._13 + p.y * self._23 + p.z * self._33
        )
    }

    pub fn transform_2d(&self, p: &Vector2D<U>) -> Vector2D<U> {
        Vector2D::new(
            p.x * self._11 + p.y * self._21 + self._31,
            p.x * self._12 + p.y * self._22 + self._32
        )
    }

    pub fn scale_by(&mut self, v: &Vector2D<U>) {
        self._11 = self._11 * v.x;
        self._21 = self._21 * v.x;
        self._31 = self._31 * v.x;
        self._12 = self._12 * v.y;
        self._22 = self._22 * v.y;
        self._32 = self._32 * v.y;
    }

    pub fn scale(v: &Vector2D<U>) -> Matrix3x3<U> {
        return Matrix3x3 {
            _11: v.x,  _21: 0.0,  _31: 0.0,
            _12: 0.0,  _22: v.y,  _32: 0.0,
            _13: 0.0,  _23: 0.0,  _33: 1.0,
            _unit: PhantomData
        }
    }

    pub fn translation(v: &Vector2D<U>) -> Matrix3x3<U> {
        return Matrix3x3 {
            _11: 1.0, _21: 1.0, _31: v.x,
            _12: 0.0, _22: 1.0, _32: v.y,
            _13: 0.0, _23: 0.0, _33: 1.0,
            _unit: PhantomData
        }
    }

    pub fn rotation(rad: f32) -> Matrix3x3<U> {
        return Matrix3x3 {
            _11: rad.cos(), _21: -rad.sin(), _31: 0.0,
            _12: rad.sin(), _22: rad.cos(),  _32: 0.0,
            _13: 0.0,       _23: 0.0,        _33: 1.0,
            _unit: PhantomData
        }
    }

    pub fn row_1<'l>(&'l self) -> &'l Vector3D<U> {
        unsafe { mem::transmute(&self._11 as *const f32) }
    }

    pub fn row_2<'l>(&'l self) -> &'l Vector3D<U> {
        unsafe { mem::transmute(&self._12 as *const f32) }
    }

    pub fn row_3<'l>(&'l self) -> &'l Vector3D<U> {
        unsafe { mem::transmute(&self._13 as *const f32) }
    }

    #[inline]
    pub fn identity() -> Matrix3x3<U> {
        Matrix3x3 {
            _11: 1.0, _21: 0.0, _31: 0.0,
            _12: 0.0, _22: 1.0, _32: 0.0,
            _13: 0.0, _23: 0.0, _33: 1.0,
            _unit: PhantomData
        }
    }

    #[inline]
    pub fn set_indentity(&mut self) {
        self._11 = 1.0; self._21 = 0.0; self._31 = 0.0;
        self._12 = 0.0; self._22 = 1.0; self._32 = 0.0;
        self._13 = 0.0; self._23 = 0.0; self._33 = 1.0;
    }
}

#[allow(dead_code)]
impl<U> Matrix4x4<U> {

    pub fn from_slice(from: &[f32]) -> Matrix4x4<U> {
        assert!(from.len() >= 16);
        return Matrix4x4 {
            _11: from[0],  _21: from[1],  _31: from[2],  _41: from[3],
            _12: from[4],  _22: from[5],  _32: from[6],  _42: from[7],
            _13: from[8],  _23: from[9],  _33: from[10], _43: from[11],
            _14: from[12], _24: from[13], _34: from[14], _44: from[15],
            _unit: PhantomData
        };
    }

    pub fn as_slice<'l>(&'l self) -> &'l [f32] {
        unsafe {
            return mem::transmute((&self._11 as *const f32, 16 as usize ));
        }
    }

    pub fn as_mut_slice<'l>(&'l mut self) -> &'l mut [f32] {
        unsafe {
            return mem::transmute((&mut self._11 as *mut f32, 16 as usize ));
        }
    }

    pub fn transform(&self, p: &Vector4D<U>) -> Vector4D<U> {
        Vector4D::new(
            p.x * self._11 + p.y * self._21 + p.z * self._31 + p.w * self._41,
            p.x * self._12 + p.y * self._22 + p.z * self._32 + p.w * self._42,
            p.x * self._13 + p.y * self._23 + p.z * self._33 + p.w * self._43,
            p.x * self._14 + p.y * self._24 + p.z * self._34 + p.w * self._44
        )
    }

    pub fn row_1<'l>(&'l self) -> &'l Vector4D<U> {
        unsafe { mem::transmute(&self._11 as *const f32) }
    }

    pub fn row_2<'l>(&'l self) -> &'l Vector4D<U> {
        unsafe { mem::transmute(&self._12 as *const f32) }
    }

    pub fn row_3<'l>(&'l self) -> &'l Vector4D<U> {
        unsafe { mem::transmute(&self._13 as *const f32) }
    }

    pub fn row_4<'l>(&'l self) -> &'l Vector4D<U> {
        unsafe { mem::transmute(&self._14 as *const f32) }
    }

    #[inline]
    pub fn identity() -> Matrix4x4<U> {
        Matrix4x4 {
            _11: 1.0, _21: 0.0, _31: 0.0, _41: 0.0,
            _12: 0.0, _22: 1.0, _32: 0.0, _42: 0.0,
            _13: 0.0, _23: 0.0, _33: 1.0, _43: 0.0,
            _14: 0.0, _24: 0.0, _34: 0.0, _44: 1.0,
            _unit: PhantomData
        }
    }

    pub fn scale(v: &Vector3D<U>) -> Matrix4x4<U> {
        return Matrix4x4 {
            _11: v.x, _21: 1.0, _31: 0.0, _41: 0.0,
            _12: 0.0, _22: v.y, _32: 0.0, _42: 0.0,
            _13: 0.0, _23: 0.0, _33: v.z, _43: 0.0,
            _14: 0.0, _24: 0.0, _34: 0.0, _44: 1.0,
            _unit: PhantomData
        }
    }

    pub fn translation(v: &Vector3D<U>) -> Matrix4x4<U> {
        return Matrix4x4 {
            _11: 1.0, _21: 1.0, _31: 0.0, _41: v.x,
            _12: 0.0, _22: 1.0, _32: 0.0, _42: v.y,
            _13: 0.0, _23: 0.0, _33: 1.0, _43: v.z,
            _14: 0.0, _24: 0.0, _34: 0.0, _44: 1.0,
            _unit: PhantomData
        }
    }

    #[inline]
    pub fn set_indentity(&mut self) {
        self._11 = 1.0; self._21 = 0.0; self._31 = 0.0; self._41 = 0.0;
        self._12 = 0.0; self._22 = 1.0; self._32 = 0.0; self._42 = 0.0;
        self._13 = 0.0; self._23 = 0.0; self._33 = 1.0; self._43 = 0.0;
        self._14 = 0.0; self._24 = 0.0; self._34 = 0.0; self._44 = 1.0;
    }
}

impl<U> Matrix4x4<U> {
    pub fn rotate(&mut self, rad: f32, axis: &Vector3D<U>) {
        let len = (axis.x * axis.x + axis.y * axis.y + axis.z * axis.z).sqrt();

        if len.abs() < EPSILON { return; }

        let len = 1.0 / len;
        let x = axis.x * len;
        let y = axis.y * len;
        let z = axis.z * len;

        let s = rad.sin();
        let c = rad.cos();
        let t = 1.0 - c;

        let a00 = self._11;
        let a01 = self._21;
        let a02 = self._31;
        let a03 = self._41;
        let a10 = self._12;
        let a11 = self._22;
        let a12 = self._32;
        let a13 = self._42;
        let a20 = self._13;
        let a21 = self._23;
        let a22 = self._33;
        let a23 = self._43;

        // Construct the elements of the rotation matrix
        let b00 = x * x * t + c;
        let b01 = y * x * t + z * s;
        let b02 = z * x * t - y * s;
        let b10 = x * y * t - z * s;
        let b11 = y * y * t + c;
        let b12 = z * y * t + x * s;
        let b20 = x * z * t + y * s;
        let b21 = y * z * t - x * s;
        let b22 = z * z * t + c;

        // Perform rotation-specific matrix multiplication
        self._11 = a00 * b00 + a10 * b01 + a20 * b02;
        self._21 = a01 * b00 + a11 * b01 + a21 * b02;
        self._31 = a02 * b00 + a12 * b01 + a22 * b02;
        self._41 = a03 * b00 + a13 * b01 + a23 * b02;
        self._12 = a00 * b10 + a10 * b11 + a20 * b12;
        self._22 = a01 * b10 + a11 * b11 + a21 * b12;
        self._32 = a02 * b10 + a12 * b11 + a22 * b12;
        self._42 = a03 * b10 + a13 * b11 + a23 * b12;
        self._13 = a00 * b20 + a10 * b21 + a20 * b22;
        self._23 = a01 * b20 + a11 * b21 + a21 * b22;
        self._33 = a02 * b20 + a12 * b21 + a22 * b22;
        self._43 = a03 * b20 + a13 * b21 + a23 * b22;
    }

    pub fn translate(&mut self, v: &Vector3D<U>) {
        self._14 = self._11 * v.x + self._12 * v.y + self._13 * v.z + self._14;
        self._24 = self._21 * v.x + self._22 * v.y + self._23 * v.z + self._24;
        self._34 = self._31 * v.x + self._32 * v.y + self._33 * v.z + self._34;
        self._44 = self._41 * v.x + self._42 * v.y + self._43 * v.z + self._44;
    }

    pub fn scale_by(&mut self, v: &Vector3D<U>) {
        self._11 = self._11 * v.x;
        self._21 = self._21 * v.x;
        self._31 = self._31 * v.x;
        self._41 = self._41 * v.x;
        self._12 = self._12 * v.y;
        self._22 = self._22 * v.y;
        self._32 = self._32 * v.y;
        self._42 = self._42 * v.y;
        self._13 = self._13 * v.z;
        self._23 = self._23 * v.z;
        self._33 = self._33 * v.z;
        self._43 = self._43 * v.z;
    }

    pub fn invert(&self, out: &mut Matrix4x4<U>) {
        let a00 = self._11;
        let a01 = self._21;
        let a02 = self._31;
        let a03 = self._41;
        let a10 = self._12;
        let a11 = self._22;
        let a12 = self._32;
        let a13 = self._42;
        let a20 = self._13;
        let a21 = self._23;
        let a22 = self._33;
        let a23 = self._43;
        let a30 = self._14;
        let a31 = self._24;
        let a32 = self._34;
        let a33 = self._44;

        let b00 = a00 * a11 - a01 * a10;
        let b01 = a00 * a12 - a02 * a10;
        let b02 = a00 * a13 - a03 * a10;
        let b03 = a01 * a12 - a02 * a11;
        let b04 = a01 * a13 - a03 * a11;
        let b05 = a02 * a13 - a03 * a12;
        let b06 = a20 * a31 - a21 * a30;
        let b07 = a20 * a32 - a22 * a30;
        let b08 = a20 * a33 - a23 * a30;
        let b09 = a21 * a32 - a22 * a31;
        let b10 = a21 * a33 - a23 * a31;
        let b11 = a22 * a33 - a23 * a32;

        let det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;

        if det.abs() < EPSILON {
            panic!(); // TODO
        }

        let det = 1.0 / det;

        out._11 = (a11 * b11 - a12 * b10 + a13 * b09) * det;
        out._21 = (a02 * b10 - a01 * b11 - a03 * b09) * det;
        out._31 = (a31 * b05 - a32 * b04 + a33 * b03) * det;
        out._41 = (a22 * b04 - a21 * b05 - a23 * b03) * det;
        out._12 = (a12 * b08 - a10 * b11 - a13 * b07) * det;
        out._22 = (a00 * b11 - a02 * b08 + a03 * b07) * det;
        out._32 = (a32 * b02 - a30 * b05 - a33 * b01) * det;
        out._42 = (a20 * b05 - a22 * b02 + a23 * b01) * det;
        out._13 = (a10 * b10 - a11 * b08 + a13 * b06) * det;
        out._23 = (a01 * b08 - a00 * b10 - a03 * b06) * det;
        out._33 = (a30 * b04 - a31 * b02 + a33 * b00) * det;
        out._43 = (a21 * b02 - a20 * b04 - a23 * b00) * det;
        out._14 = (a11 * b07 - a10 * b09 - a12 * b06) * det;
        out._24 = (a00 * b09 - a01 * b07 + a02 * b06) * det;
        out._34 = (a31 * b01 - a30 * b03 - a32 * b00) * det;
        out._44 = (a20 * b03 - a21 * b01 + a22 * b00) * det;
    }
}

#[allow(dead_code)]
impl<U> ops::Mul<Matrix4x4<U>> for Matrix4x4<U> {

    type Output = Matrix4x4<U>;

    #[inline]
    fn mul(self, rhs: Matrix4x4<U>) -> Matrix4x4<U> {
        return Matrix4x4 {
            _11: self._11 * rhs._11 + self._12 * rhs._21 + self._13 * rhs._31 + self._14 * rhs._41,
            _21: self._21 * rhs._11 + self._22 * rhs._21 + self._23 * rhs._31 + self._24 * rhs._41,
            _31: self._31 * rhs._11 + self._32 * rhs._21 + self._33 * rhs._31 + self._34 * rhs._41,
            _41: self._41 * rhs._11 + self._42 * rhs._21 + self._43 * rhs._31 + self._44 * rhs._41,
            _12: self._11 * rhs._12 + self._12 * rhs._22 + self._13 * rhs._32 + self._14 * rhs._42,
            _22: self._21 * rhs._12 + self._22 * rhs._22 + self._23 * rhs._32 + self._24 * rhs._42,
            _32: self._31 * rhs._12 + self._32 * rhs._22 + self._33 * rhs._32 + self._34 * rhs._42,
            _42: self._41 * rhs._12 + self._42 * rhs._22 + self._43 * rhs._32 + self._44 * rhs._42,
            _13: self._11 * rhs._13 + self._12 * rhs._23 + self._13 * rhs._33 + self._14 * rhs._43,
            _23: self._21 * rhs._13 + self._22 * rhs._23 + self._23 * rhs._33 + self._24 * rhs._43,
            _33: self._31 * rhs._13 + self._32 * rhs._23 + self._33 * rhs._33 + self._34 * rhs._43,
            _43: self._41 * rhs._13 + self._42 * rhs._23 + self._43 * rhs._33 + self._44 * rhs._43,
            _14: self._11 * rhs._14 + self._12 * rhs._24 + self._13 * rhs._34 + self._14 * rhs._44,
            _24: self._21 * rhs._14 + self._22 * rhs._24 + self._23 * rhs._34 + self._24 * rhs._44,
            _34: self._31 * rhs._14 + self._32 * rhs._24 + self._33 * rhs._34 + self._34 * rhs._44,
            _44: self._41 * rhs._14 + self._42 * rhs._24 + self._43 * rhs._34 + self._44 * rhs._44,
            _unit: PhantomData
        };
    }
}

#[allow(dead_code)]
impl<U> ops::Mul<Matrix3x3<U>> for Matrix3x3<U> {

    type Output = Matrix3x3<U>;

    #[inline]
    fn mul(self, rhs: Matrix3x3<U>) -> Matrix3x3<U> {
        return Matrix3x3 {
            _11: self._11 * rhs._11 + self._12 * rhs._21 + self._13 * rhs._31,
            _21: self._21 * rhs._11 + self._22 * rhs._21 + self._23 * rhs._31,
            _31: self._31 * rhs._11 + self._32 * rhs._21 + self._33 * rhs._31,
            _12: self._11 * rhs._12 + self._12 * rhs._22 + self._13 * rhs._32,
            _22: self._21 * rhs._12 + self._22 * rhs._22 + self._23 * rhs._32,
            _32: self._31 * rhs._12 + self._32 * rhs._22 + self._33 * rhs._32,
            _13: self._11 * rhs._13 + self._12 * rhs._23 + self._13 * rhs._33,
            _23: self._21 * rhs._13 + self._22 * rhs._23 + self._23 * rhs._33,
            _33: self._31 * rhs._13 + self._32 * rhs._23 + self._33 * rhs._33,
            _unit: PhantomData
        };
    }
}

#[allow(dead_code)]
impl<U> ops::Mul<Matrix2x2<U>> for Matrix2x2<U> {

    type Output = Matrix2x2<U>;

    #[inline]
    fn mul(self, rhs: Matrix2x2<U>) -> Matrix2x2<U> {
        return Matrix2x2 {
            _11: self._11 * rhs._11 + self._12 * rhs._21,
            _21: self._21 * rhs._11 + self._22 * rhs._21,
            _12: self._11 * rhs._12 + self._12 * rhs._22,
            _22: self._21 * rhs._12 + self._22 * rhs._22,
            _unit: PhantomData
        };
    }
}

impl<U> Matrix4x4<U> {

    pub fn new(
        a11: f32, a21: f32, a31: f32, a41: f32,
        a12: f32, a22: f32, a32: f32, a42: f32,
        a13: f32, a23: f32, a33: f32, a43: f32,
        a14: f32, a24: f32, a34: f32, a44: f32
    ) -> Matrix4x4<U> {
        Matrix4x4 {
            _11: a11, _21: a21, _31: a31, _41: a41,
            _12: a12, _22: a22, _32: a32, _42: a42,
            _13: a13, _23: a23, _33: a33, _43: a43,
            _14: a14, _24: a24, _34: a34, _44: a44,
            _unit: PhantomData
        }
    }

    pub fn perspective(
        fovy: f32, aspect: f32, near: f32, far: f32,
        mat: &mut Matrix4x4<U>
    ) {
        let f = 1.0 / (fovy / 2.0).tan();
        let nf: f32 = 1.0 / (near - far);

        mat._11 = f / aspect;
        mat._21 = 0.0;
        mat._31 = 0.0;
        mat._41 = 0.0;
        mat._12 = 0.0;
        mat._22 = f;
        mat._32 = 0.0;
        mat._42 = 0.0;
        mat._13 = 0.0;
        mat._23 = 0.0;
        mat._33 = (far + near) * nf;
        mat._43 = -1.0;
        mat._14 = 0.0;
        mat._24 = 0.0;
        mat._34 = (2.0 * far * near) * nf;
        mat._44 = 0.0;
    }

    pub fn rotation(rad: f32, s: &Vector3D<U>) -> Matrix4x4<U> {
        let mut m: Matrix4x4<U> = Matrix4x4::identity();
        m.rotate(rad, s);
        return m;
    }
}

