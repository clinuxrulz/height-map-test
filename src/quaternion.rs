use crate::{One, Sqrt, Vec3, Zero};
use std::ops::{Add, Sub, Mul, Div};

pub struct Quaternion<T> {
    pub w: T,
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Copy> Quaternion<T> {
    pub fn identity() -> Quaternion<T>
    where
        T: One + Zero,
    {
        Quaternion {
            w: T::one(),
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    pub fn from_uv(u: Vec3<T>, v: Vec3<T>) -> Quaternion<T>
    where
        T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Sqrt<Output=T> + PartialOrd + Zero + One
    {
        let w = u.cross(v);
        Self::from_uvw(u, v, w)
    }

    pub fn from_vw(v: Vec3<T>, w: Vec3<T>) -> Quaternion<T>
    where
        T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Sqrt<Output=T> + PartialOrd + Zero + One
    {
        let u = v.cross(w);
        Self::from_uvw(u, v, w)
    }

    pub fn from_wu(w: Vec3<T>, u: Vec3<T>) -> Quaternion<T>
    where
        T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Sqrt<Output=T> + PartialOrd + Zero + One
    {
        let v = w.cross(u);
        Self::from_uvw(u, v, w)
    }

    fn from_uvw(u: Vec3<T>, v: Vec3<T>, w: Vec3<T>) -> Quaternion<T>
    where
        T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Sqrt<Output=T> + PartialOrd + Zero + One
    {
        let m00 = u.x;
        let m10 = u.y;
        let m20 = u.z;
        let m01 = v.x;
        let m11 = v.y;
        let m21 = v.z;
        let m02 = w.x;
        let m12 = w.y;
        let m22 = w.z;
        let tr = m00 + m11 + m22;
        let qw: T;
        let qx: T;
        let qy: T;
        let qz: T;
        let two = T::one() + T::one();
        let four = two + two;
        let quarter = T::one() / four;
        if tr > T::zero() {
            let s = (tr + T::one()).sqrt() * two; // S=4*qw 
            qw = quarter * s;
            qx = (m21 - m12) / s;
            qy = (m02 - m20) / s; 
            qz = (m10 - m01) / s; 
        } else if (m00 > m11)&(m00 > m22) { 
            let s = (T::one() + m00 - m11 - m22).sqrt() * two; // S=4*qx 
            qw = (m21 - m12) / s;
            qx = quarter * s;
            qy = (m01 + m10) / s; 
            qz = (m02 + m20) / s; 
        } else if m11 > m22 { 
            let s = (T::one() + m11 - m00 - m22).sqrt() * two; // S=4*qy
            qw = (m02 - m20) / s;
            qx = (m01 + m10) / s; 
            qy = quarter * s;
            qz = (m12 + m21) / s; 
        } else { 
            let s = (T::one() + m22 - m00 - m11).sqrt() * two; // S=4*qz
            qw = (m10 - m01) / s;
            qx = (m02 + m20) / s;
            qy = (m12 + m21) / s;
            qz = quarter * s;
        }
        Quaternion { w: qw, x: qx, y: qy, z: qz, }
    }
}
