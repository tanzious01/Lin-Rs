use crate::matrix::Matrix;
mod la_funcs;
mod matrix;
mod implementations;
use crate::implementations::VectorArith;  // Changed this line
use crate::la_funcs::{orthonormal, projection, q_solve};
fn main() {
    let A = Matrix { 
        nRows: 4, 
        nCols: 4, 
        data: vec![
            0.,  1., -2., -8., 
           -9.,  7., -7.,  1., 
            0.,  5., -1., -2., 
           -2., -7.,  1.,  6.
        ] 
    };

    let b = vec![-85.0, -13.0, -40.0, 92.0];

    // A = QR
    // R = QTA
    // R should be a Upper Triangle Matrix
    let Q = orthonormal(&A);
    let R = Q.transpose().dot(&A).unwrap();
    let g = q_solve(&A, &b);
    println!("{:?}",g);
}
