use crate::matrix::{self, LinAlgError, Matrix};
use std::iter::zip;
use std::thread::ScopedJoinHandle;
use std::{fmt};
use std::ops::{Add, Mul, Sub};
use num_traits::Float;
impl std::fmt::Display for  Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let stacked_vec: Vec<Vec<_>> = self
            .data
            .chunks(self.nCols)
            .map(|chunk| chunk.to_vec())
            .collect();
        for row in stacked_vec {
            writeln!(f, "  {:?}", row)?;
        }
        write!(f, "")
    }
}




impl Add<f64> for Matrix {
    type Output = Self;
    fn add(self,b:f64) -> Matrix {
        let data:Vec<f64> = self.data.iter().map(|x|x+b).collect();
        return Matrix { nRows: self.nRows, nCols: self.nCols, data };      

    } 
}





impl Add<Matrix> for Matrix {
    type Output = Self;
    fn add(self,b:Matrix) -> Matrix {

        let data:Vec<f64> = self.data.iter().zip(b.data).map(|x|x.0+x.1).collect();
        return Matrix { nRows: self.nRows, nCols: self.nCols, data}
    } 
}

impl Sub<f64> for Matrix {
    type Output = Self;
    fn sub(self,b:f64) -> Matrix {
        let data:Vec<f64> = self.data.iter().map(|x|x-b).collect();
        return Matrix { nRows: self.nRows, nCols: self.nCols, data };      

    } 
}

impl Sub<Matrix> for Matrix {
    type Output = Self;
    fn sub(self,b:Matrix) -> Matrix {
        let data:Vec<f64> = self.data.iter().zip(b.data).map(|x|x.0-x.1).collect();
        return Matrix { nRows: self.nRows, nCols: self.nCols, data}
    } 
}

impl Mul<f64> for Matrix {
    type Output = Self;
    fn mul(self,b:f64) -> Matrix {
        let data:Vec<f64> = self.data.iter().map(|x|x*b).collect();
        return Matrix { nRows: self.nRows, nCols: self.nCols, data };      
    } 
}

impl Matrix {
    pub fn zeros(rows:usize,cols:usize) -> Matrix {
        let total_elements = cols*rows;
        let new_vec = vec![0.0;total_elements];
        return Matrix {nRows:rows,nCols: cols, data: new_vec}
    }
    pub fn transpose (&self) -> Self {
        //transposes the matrix, doesnt consume the current one.
        //first make zeros matrix which will be the size of the transposed one
        let mut transposed:Matrix = Matrix::zeros(self.nCols, self.nRows);
        for i in 0..self.nRows {
            for j in 0..self.nCols {
                transposed.data[(j*transposed.nCols)+i] = self.data[(i*self.nCols)+j]
            }
        }
        return transposed

    }

    pub fn dot (&self,other:&Self) -> Result<Matrix,LinAlgError> {
        let mut return_matrix:Matrix = Matrix::zeros(self.nRows, other.nCols);
        if self.nCols != other.nRows{
            return Err(LinAlgError::DimensionError)
        }
        else {
            for i in 0..self.nRows {
                for k  in 0..other.nCols {
                    for j in 0..other.nRows{
                        let product = self.data[(i*self.nCols)+j]*other.data[(j*other.nCols)+k];
                        return_matrix.data[(i*return_matrix.nCols)+k] += product
                    } 
                }
            }
            return Ok(return_matrix);
        }

    }



    pub fn eye(size:usize) -> Matrix {
        let mut zeros = Matrix::zeros(size, size);
        for i in 0..size {
            zeros.data[i*size+i] = 1.0
        }
        return zeros
   
    }

    pub fn get_row(&self,i:usize) -> &[f64]{
        let start = i*self.nCols;
        let end = start+self.nCols;
        return &self.data[start..end]
    }
    pub fn get_col(&self,i:usize) -> Vec<f64>{
        let indexes:Vec<usize> = (i..self.data.len()).step_by(self.nCols).collect();
        let homie:Vec<f64> = indexes.iter().map(|x|self.data[*x]).collect();
        return homie
    }
    pub fn set_col(&mut self, i:usize,data:&Vec<f64>) {
        //goal for this one is to take the Vec<f64> and add the column into the matrix/replace
        //current matrix with this changed one.
        let indexes:Vec<usize> = (i..self.data.len()).step_by(self.nCols).collect();
        for (i,e) in indexes.iter().enumerate(){
            self.data[*e] = data[i];
        }

   


    }
    
}
pub trait VectorArith<T> 
where T:Float{
    type Output;
    type Error;
    fn dot(&self, other: &Self) -> Result<Self::Output, Self::Error>;
    fn norm(&self) -> Self::Output;


}

// VECTOR Tr IMPLEMENTATIONS------------------------------------------------------------------------------------------------------------------
impl<T: Float + std::iter::Sum> VectorArith<T> for Vec<T> {
    type Output = T;
    type Error = LinAlgError;
    fn dot(&self, other: &Self) -> Result<T, LinAlgError> {
        if self.len() != other.len() {
            return Err(LinAlgError::DimensionError)
        }
        else {
            let scalar = self.iter().zip(other).map(|x|    *x.0 * *x.1).sum::<T>();
            return Ok(scalar)
        }
    }
    fn norm (&self)-> T {
        return self.dot(self).unwrap().sqrt()  
    
    }
}
impl<T: Float + std::iter::Sum> VectorArith<T> for [T] {
    type Output = T;
    type Error = LinAlgError;

    fn dot(&self, other: &Self) -> Result<T, LinAlgError> {
        if self.len() != other.len() {
            return Err(LinAlgError::DimensionError)
        }
        else {
            let scalar = self.iter().zip(other).map(|x|    *x.0 * *x.1).sum::<T>();
            return Ok(scalar)
        }
    }
    fn norm (&self)-> T {
        return self.dot(self).unwrap().sqrt()  
    }
}




