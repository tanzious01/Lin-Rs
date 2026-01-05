use crate::implementations::VectorArith;
use crate::matrix;
use crate::implementations;
use crate::matrix::Matrix;

// Her i need to make function to turn a matrix orthonormal!
// Use grandschmidt procedure.
// Do not consume the current matrix, just read its data and transform it

fn add_scalar(vec:Vec<f64>,scalar:f64) -> Vec<f64>{
    vec.iter().map(|x|*x+scalar).collect()
}

fn sub_scalar(vec:Vec<f64>,scalar:f64) -> Vec<f64>{
    vec.iter().map(|x|*x-scalar).collect()
}


fn add_vector(vec: &mut Vec<f64>, b: &Vec<f64>) {
    vec.iter_mut().zip(b).for_each(|(a, b)| *a += b);
}

fn sub_vector(vec:&mut Vec<f64>,b:&Vec<f64>){
    vec.iter_mut().zip(b).for_each(|(A,B)|*A-=B);
}

fn mul_vector(vec:Vec<f64>,b:Vec<f64>) -> Vec<f64>{
    vec.iter().zip(b).map(|(A,B)|A*B).collect()
}


fn div_scalar(vec:&Vec<f64>,b:&f64) -> Vec<f64>{
    vec.iter().map(|A|A/b).collect()
}


// pub fn projection(x:&[f64],v:&[f64]) -> Vec<f64> {
//     let numerator = x.dot(&v).unwrap();
//     let denom = v.norm().powf(2.0);
//     let c = numerator/denom;
//     let final_vec:Vec<f64> = v.iter().map(|x|x*c).collect();
//     return final_vec;
// }
//

pub fn projection(x:&Vec<f64>,v:&Vec<f64>) -> Vec<f64> {
    let numerator = x.dot(&v).unwrap();
    let denom = v.norm().powf(2.0);
    let c = numerator/denom;
    let final_vec:Vec<f64> = v.iter().map(|x|x*c).collect();
    return final_vec;
}


//Note orthonormal matrices, takes a vector, then removes all the other parallel components to
//previous ui vectors.



pub fn orthonormal(A: &Matrix) -> Matrix {
    let mut ui_vec:Vec<Vec<f64>> = Vec::new();
    let first_col = A.get_col(0);
    ui_vec.push(div_scalar(&first_col, &first_col.norm()));
    for i in 1..A.nCols {
        let mut current_col = A.get_col(i);
        for _pass in 0..2{
            for j in 0..i {
                 let projected = projection(&current_col, &ui_vec[j]);
                 sub_vector(&mut current_col, &projected);
            }
            }
         let normalized = div_scalar(&current_col, &current_col.norm());
         ui_vec.push(normalized);
    }
    let mut new_Matrix:Matrix = Matrix::zeros(ui_vec[0].len(), ui_vec.len());
    ui_vec.iter().enumerate().for_each(|(i,e)|new_Matrix.set_col(i, e));
    return new_Matrix
}

pub fn q_solve(A: &Matrix, b: &Vec<f64>) -> Vec<f64> {
    // 1. Setup the math
    let b_mat = Matrix { nRows: b.len(), nCols: 1, data: b.to_vec() };
    let Q = orthonormal(A);
    let R = Q.transpose().dot(&A).unwrap();
    let QTb = Q.transpose().dot(&b_mat).unwrap();
    println!("{}",R);
    let mut x: Vec<f64> = vec![0.0; A.nCols]; 
    // FIGURE THIS OUT NEXT TIME!
}
