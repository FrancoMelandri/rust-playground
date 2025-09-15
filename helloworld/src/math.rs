use ndarray::{arr1, arr2, Array1, Array2};

pub struct Math {
    pub matrix: Array2<f32>,
    pub vector: Array1<f32>,
}

impl Math {
    pub fn new(rows: usize, columns: usize) -> Self {
        Math { matrix: Array2::<f32>::zeros((rows, columns)), vector: Array1::<f32>::zeros(columns) }
    }

    pub fn with_matrix<const N: usize>(&self, xs: &[[f32; N]]) -> Self {
        Math {
            matrix: arr2(xs),
            vector: self.vector.clone()
        }
    }

    pub fn with_vector<const N: usize>(&self, xs: &[f32; N]) -> Self {
        Math {
            matrix: self.matrix.clone(),
            vector: arr1(xs)
        }
    }
}