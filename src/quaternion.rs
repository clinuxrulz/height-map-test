use crate::{Acos, One, Sin, Sqrt, Vec3, Zero};
use std::ops::{Add, Sub, Mul, Div, Neg};

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

    pub fn dot(self, rhs: Quaternion<T>) -> T
    where
        T: Add<Output=T> + Mul<Output=T>
    {
        let lhs = self;
        lhs.w * rhs.w + lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    pub fn conjugate(self) -> Quaternion<T>
    where
        T: Neg<Output=T>
    {
        Quaternion {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn rotate(self, pt: Vec3<T>) -> Vec3<T>
    where
        T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Neg<Output=T> + Zero
    {
        let p = Quaternion { w: T::zero(), x: pt.x, y: pt.y, z: pt.z, };
        let p2 = self * p * self.conjugate();
        Vec3::new(p2.x, p2.y, p2.z)
    }

    pub fn slerp(self, target: Quaternion<T>, t: T, allow_flip: bool) -> Quaternion<T>
    where
        T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Neg<Output=T> + Zero + One + Neg + PartialOrd + Acos + Sin
    {
        let a = self;
        let b = target;
        let cos_angle: T = a.dot(b);
        let mut c1: T;
        let c2: T;
        let ten: T;
        {
            let mut ten2 = T::zero();
            for _i in 0..10 {
                ten2 = ten2 + T::one(); 
            }
            ten = ten2;
        }
        let one_hundredth = T::one() / (ten * ten);
        let abs_cos_angle: T = if cos_angle < T::zero() { -cos_angle } else { cos_angle };
        if (T::one() - abs_cos_angle) < one_hundredth {
            c1 = T::one() - t;
            c2 = t;
        } else {
            let angle = abs_cos_angle.acos();
            let sin_angle = angle.sin();
            c1 = (angle * (T::one() - t)).sin() / sin_angle;
            c2 = (angle * t).sin() / sin_angle;
        }
        if allow_flip && (cos_angle < T::zero()) {
            c1 = -c1;
        }
        return Quaternion {
            w: c1 * a.w + c2 * b.w,
            x: c1 * a.x + c2 * b.x,
            y: c1 * a.y + c2 * b.y,
            z: c1 * a.z + c2 * b.z,
        };
    }
}

impl<T: Clone> Clone for Quaternion<T> {
    fn clone(&self) -> Self {
        Self { w: self.w.clone(), x: self.x.clone(), y: self.y.clone(), z: self.z.clone() }
    }
}

impl<T: Copy> Copy for Quaternion<T> {}

impl<T: Copy + Add<Output=T> + Sub<Output=T> + Mul<Output=T>> Mul for Quaternion<T> {
    type Output = Quaternion<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        let lhs = self;
        Quaternion {
            w: lhs.w * rhs.w - lhs.x * rhs.x - lhs.y * rhs.y - lhs.z * rhs.z,
            x: lhs.w * rhs.x + lhs.x * rhs.w + lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.w * rhs.y - lhs.x * rhs.z + lhs.y * rhs.w + lhs.z * rhs.x,
            z: lhs.w * rhs.z + lhs.x * rhs.y - lhs.y * rhs.x + lhs.z * rhs.w,
        }
    }
}
