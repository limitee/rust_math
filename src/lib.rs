extern crate core;
use core::convert::Into;

pub trait BaseFloat where Self: Copy + Clone 
{

} 

impl BaseFloat for f32 {

}

impl BaseFloat for f64 {

}

#[derive(Copy, Clone, Debug)]
pub struct Vector3<T> {
    e1:T,
    e2:T,
    e3:T,
}

impl<T> Into<[T;3]> for Vector3<T> {
    
    fn into(self) -> [T; 3] {
        [self.e1, self.e2, self.e3]
    }

}

impl<T:BaseFloat> Vector3<T> {

    pub fn new(e1:T, e2:T, e3:T) -> Vector3<T> {
        Vector3 {
            e1: e1,
            e2: e2,
            e3: e3,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Matrix2<T> {
    e11:T,
    e12:T,
    e21:T,
    e22:T,
}

impl<T:BaseFloat> Matrix2<T> {

    pub fn new(e11:T, e12:T, e21:T, e22:T) -> Matrix2<T> {
        Matrix2 {
            e11: e11,
            e12: e12,
            e21: e21,
            e22: e22,
        }
    }
}

impl<T> Into<[[T;2];2]> for Matrix2<T> {
    
    fn into(self) -> [[T; 2]; 2] {
        [
            [self.e11, self.e12],
            [self.e21, self.e22],
        ]
    }

}

#[derive(Copy, Clone, Debug)]
pub struct Matrix4<T> {
    e11:T,
    e12:T,
    e13:T,
    e14:T,
    e21:T,
    e22:T,
    e23:T,
    e24:T,
    e31:T,
    e32:T,
    e33:T,
    e34:T,
    e41:T,
    e42:T,
    e43:T,
    e44:T,
}

impl<T:BaseFloat> Matrix4<T> {

    pub fn new(e11:T, e12:T, e13:T, e14:T,
        e21:T, e22:T, e23:T, e24:T,
        e31:T, e32:T, e33:T, e34:T,
        e41:T, e42:T, e43:T, e44:T) -> Matrix4<T> {
        Matrix4 {
            e11: e11,
            e12: e12,
            e13: e13,
            e14: e14,
            e21: e21,
            e22: e22,
            e23: e23,
            e24: e24,
            e31: e31,
            e32: e32,
            e33: e33,
            e34: e34,
            e41: e41,
            e42: e42,
            e43: e43,
            e44: e44,
        }
    }

    /**
     * 视图矩阵
     * pos, 所在位置
     * at, 看向的位置
     * up, 向上的方向
     */
    pub fn look_at(pos:Vector3<f32>, at:Vector3<f32>, up:Vector3<f32>) -> Matrix4<f32> {
        let f = {
            let len = at.e1*at.e1 + at.e2*at.e2 + at.e3*at.e3;
            let len = len.sqrt();
            [at.e1/len, at.e2/len, at.e3/len]
        };
        let s = 
        [
            up.e2 * f[2] - up.e3 * f[1],
            up.e3 * f[0] - up.e1 * f[2],
            up.e1 * f[1] - up.e2 * f[0]
        ];
        let s_norm = {
            let len = s[0]*s[0] + s[1]*s[1] + s[2]*s[2];
            let len = len.sqrt();
            [s[0]/len, s[1]/len, s[2]/len]
        };
        let u = 
        [
            f[1]*s_norm[2] - f[2]*s_norm[1],
            f[2]*s_norm[0] - f[0]*s_norm[2],
            f[0]*s_norm[1] - f[1]*s_norm[0],
        ];
        let p = 
        [
            -pos.e1*s_norm[0] - pos.e2*s_norm[1] - pos.e3*s_norm[2],
            -pos.e1*u[0] - pos.e2*u[1] - pos.e3*u[2],
            -pos.e1*f[0] - pos.e2*f[1] - pos.e3*f[2],
        ];
        Matrix4::new(
            s_norm[0], u[0], f[0], 0.0,
            s_norm[1], u[1], f[1], 0.0,
            s_norm[2], u[2], f[2], 0.0,
            p[0], p[1], p[2], 1.0_f32
        ) 
    }
}

impl<T> Into<[[T;4];4]> for Matrix4<T> {
    
    fn into(self) -> [[T; 4]; 4] {
        [
            [self.e11, self.e12, self.e13, self.e14],
            [self.e21, self.e22, self.e23, self.e24],
            [self.e31, self.e32, self.e33, self.e34],
            [self.e41, self.e42, self.e43, self.e44],
        ]
    }

}

