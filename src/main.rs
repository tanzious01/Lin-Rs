#![allow(warnings)]
use core::f32;
use std::error::Error;
use std::f32::NAN;
use std::{array, iter, usize, vec};
use std::iter::{zip, Sum};
use std::ptr::dangling;
use std::{env::set_var, fmt, io::Seek};
use std::ops::{Add,Sub,Mul};
use std::fmt::Debug;

use num_traits::float::FloatCore;
use num_traits::{one, zero, Float, One, PrimInt, Signed, Zero};

fn main() {
    let mut A: Matrix<f32> = Matrix { nRows: 2,nCol: 2,data: vec![2.0, 4.0,1.0, 2.0 ]};
    let B: Matrix<f32> = Matrix {nRows: 2, nCol: 1, data: vec![6.0, 3.0]};
}

#[allow(dead_code)]          
#[allow(non_snake_case)]
#[derive(Debug)]
pub enum LinAlgError {
    DimensionError,
    NoUniqueSolution,
    InfiniteSolutions,
    NoSolution
}

#[allow(non_snake_case)]
#[derive(Clone,Debug,PartialEq)]
pub struct Matrix<T> {
    nRows:usize,
    nCol:usize,
    data:Vec<T>
}
impl<T: std::fmt::Display + Clone + std::fmt::Debug> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let stacked_vec: Vec<Vec<T>> = self.data.chunks(self.nCol).map(|chunk| chunk.to_vec()).collect();
        for row in stacked_vec {
            writeln!(f, "  {:?}", row)?;
        }
        write!(f, "")
    }
}
impl<T:Add +Default+ PartialEq +Clone + Debug + Zero + One + Copy>  Matrix<T> {
   #[allow(non_snake_case)]
    pub fn zeros(nRows:usize,nCol:usize) -> Self {
        return Matrix{nRows,nCol,data:vec![T::zero();nRows*nCol]};
    }

    //Because its using self.nCol, thats the reason its able to get the proper value and add it to
    //new_matrix
    pub fn transpose(self) -> Self {
        let mut new_matrix = Matrix::zeros(self.nCol, self.nRows);
        for i in 0..new_matrix.nRows {
            for j in 0..new_matrix.nCol {
                println!("{:?}",self.data[j*self.nCol+i]);
                new_matrix.data[i*new_matrix.nCol+j] = self.data[j*self.nCol+i];
            }
            
        }
        return new_matrix;
        }

    pub fn eye(size:usize) -> Self {
        let mut final_matrix = Matrix::zeros(size, size);
        let mut index = 0;
        for j in 0..final_matrix.nCol {
            final_matrix.data[j*final_matrix.nCol+index] = T::one();
            index +=1;
        }
        return final_matrix;
        }

    pub fn concat_col(&mut self,vector:Matrix<T>) -> Result<Matrix<T>,LinAlgError> {
        if self.nRows != vector.nRows{
            return Err(LinAlgError::DimensionError);
        }
        else {
            let mut new_vec:Vec<Vec<T>> = Vec::new();
            let self_vecs:Vec<Vec<T>> = self.data.chunks(self.nCol).map(|x|x.to_vec()).collect();
            let vector_vecs:Vec<Vec<T>> = vector.data.chunks(vector.nCol).map(|x|x.to_vec()).collect();
            for (mut s, v) in zip(self_vecs, vector_vecs) {
                s.extend(v);
                new_vec.push(s);
            
            }
            let new_vec:Vec<T> = new_vec.into_iter().flatten().collect();
        Ok(Matrix { nRows: self.nRows, nCol:self.nCol+vector.nCol,data:new_vec})
        }
    } 

 }
        
impl<T: Copy + Add<Output = T>> Add for &Matrix<T> {
type Output = Matrix<T>;
    fn add(mut self,other:Self) -> Matrix<T> {
            Matrix{nCol:self.nCol,nRows:self.nRows,data:self.data.iter().zip(other.data.iter()).map(|(x,y)|*x+*y).collect()}
    }
}
impl<T: Copy + Add<Output = T>> Add<T> for Matrix<T> {
type Output = Matrix<T>;
    fn add(mut self,scalar:T) -> Matrix<T> {
        for i in &mut self.data{
            *i =  *i + scalar;
        }
        self
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Matrix<T> {
type Output = Self;
    fn sub(self,other:Self) -> Matrix<T> {
        Matrix{nRows:self.nRows,nCol:self.nCol,data:self.
            data.iter().zip(other.data.iter()).map(|(x,y)|*x-*y).collect()}
    }
}

impl<T: Copy + Sub<Output = T>> Sub<T> for Matrix<T> {
type Output = Self;
    fn sub(self,scalar:T) -> Matrix<T> {
        Matrix { nRows: self.nRows, nCol:self.nCol, data: self.data.iter().map(|x|*x-scalar).collect()}
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Matrix<T> {
type Output = Self;
    fn mul(self,scalar:T) -> Matrix<T> {
        Matrix { nRows: self.nRows, nCol:self.nCol, data: self.data.iter().map(|x|*x*scalar).collect()}
    }
}


// Dot product :pensive:
impl<T: Default+ Debug+PartialEq +Clone + Debug + Zero + One + Copy+std::ops::AddAssign +Mul<Output = T>> Mul for Matrix<T> {
type Output = Result<Self,LinAlgError>;
    fn mul(self,other:Matrix<T>) -> Result<Matrix<T>,LinAlgError> {
        if self.nCol != other.nRows{
            Err(LinAlgError::DimensionError)
        }
        else {
            let mut new_matrix:Matrix<T> = Matrix::zeros(self.nRows, other.nCol);
            for i in 0..new_matrix.nRows{
                for j in 0..other.nCol {
                    for k in  0..other.nRows {
                        let a = self.data[i*self.nCol+k];
                        let b = other.data[k*other.nCol+j];
                        new_matrix.data[i*new_matrix.nCol+j] += a*b
                    }

                }
            }
            Ok(new_matrix)
            }

        }

    }


pub fn rank(mut A:Matrix<f32>) -> i32 {
        for k in 0..A.nRows-1 {
            if A.data[k*A.nCol+k] == 0.0 {
                continue;
            }
            for i in k+1..A.nRows {
                if A.data[i*A.nCol+k] == 0.0 {
                    continue;
                }
                let factor = A.data[k*A.nCol+k]/A.data[i*A.nCol+k];
                for j in k..A.nCol {
                    A.data[i*A.nCol+j] *= factor;
                    A.data[i*A.nCol+j] -= A.data[k*A.nCol+j];
                }
            }
        }
        let mut value = 0;
        for i in A.data.chunks(A.nCol) {
            if i.iter().map(|x|x.abs()).sum::<f32>() != 0.0 {
                value+=1
            }
        }
        println!("{}",A);
        return value;
}

pub fn g_elim(mut A:Matrix<f32>,mut b:Matrix<f32>) -> Result<Matrix<f32>,LinAlgError> {
    if A.nRows != b.nRows {
        return Err(LinAlgError::DimensionError);
    }
    if b.nCol > 1 {
        return Err(LinAlgError::DimensionError);
    }
    let mut x:Matrix<f32> = Matrix::zeros(b.nRows, 1);
    for k in 0..A.nRows -1 //-1 cus last row will have all zeros but pivot
    {
        if  A.data[k*A.nCol+k] == 0.0 {
            continue;
        }
        for i in (k+1)..A.nRows  // this will start at the row after the pivot one so we can
                                        // 0 it out
        {
            let factor:f32 = A.data[k*A.nCol+k] / A.data[i*A.nCol+k];
            for j in k..A.nCol {
                A.data[i*A.nCol+j] *= factor;
                A.data[i*A.nCol+j] -= A.data[k*A.nCol+j];
                if A.data[i*A.nCol+j] == NAN ||  A.data[i*A.nCol+j] <= 1e-10{
                    A.data[i*A.nCol+j] = 0.0;
                }
            }  
            b.data[i] *=factor;
            b.data[i]-= b.data[k];
        }
    }
    for i in 0..A.nRows {
        let row_is_zero = (i..A.nCol).all(|j| A.data[i*A.nCol+j].abs() < 1e-10);
        
        if row_is_zero {
            if b.data[i].abs() > 1e-10 {
                return Err(LinAlgError::NoSolution);
            } else {
                return Err(LinAlgError::InfiniteSolutions);
            }
        }
    }
    for i in (0..A.nRows).rev() { // Easy to understsand if you work it out physically!
        let mut sum = 0.0;
        for j in (i+1)..A.nCol{
            sum +=  A.data[i*A.nCol+j] * x.data[j];
        }
        x.data[i] = (b.data[i]-sum)/A.data[i*A.nCol+i];
    
    }
    return Ok(x)

   
}
// Stopping on oct 14th 2025 as i need to cover more material to ahead.
// Remember how dot product works, and how the gelim works. Refer your notes...




