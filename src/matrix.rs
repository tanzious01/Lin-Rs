// Need matrix struct, has to be an array.
// Need Error handling. Use enums and Match statements this time around
// Use proper ownership
// Mitigate usage of clones
// Main things
// - Struct >  Default Matrices > Basic Arithmetic > Transpose > Dot Product > Projection
// > Orthonormal Matrices > QR decomposition
//When doing transpose, read the matrix pased, and A.T should be its own matrix returned, not the
//same one transposed.


#[derive(Debug)]
pub enum LinAlgError {
    DimensionError,
}


#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    pub nRows:usize,
    pub nCols:usize,
    pub data:Vec<f64>
}






