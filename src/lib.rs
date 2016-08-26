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

