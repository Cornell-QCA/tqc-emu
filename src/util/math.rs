use nalgebra::{DMatrix, Matrix, Complex};

pub type c64 = Complex<f64>;

pub fn kronecker<T>(a: &DMatrix<T>, b: &DMatrix<T>) -> DMatrix<T>
where
    T: Scalar + Copy + std::ops::Mul<Output = T>,
{
    let (a_rows, a_cols) = a.shape();
    let (b_rows, b_cols) = b.shape();
    let mut kron_matrix = DMatrix::zeros(a_rows * b_rows, a_cols * b_cols);

    for i in 0..a_rows {
        for j in 0..a_cols {
            let a_val = a[(i, j)];
            for k in 0..b_rows {
                for l in 0..b_cols {
                    kron_matrix[(i * b_rows + k, j * b_cols + l)] = a_val * b[(k, l)];
                }
            }
        }
    }

    kron_matrix
}