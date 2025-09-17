use ndarray::{Array1, Array2, arr1, arr2};

pub struct MarkovChain {
    pub transition_matrix: Array2<f32>,
    pub vector: Array1<f32>,
}

impl MarkovChain {
    pub fn new(rows: usize, columns: usize) -> Self {
        MarkovChain {
            transition_matrix: Array2::<f32>::zeros((rows, columns)),
            vector: Array1::<f32>::zeros(columns),
        }
    }

    pub fn with_matrix<const N: usize>(&self, xs: &[[f32; N]]) -> Self {
        MarkovChain {
            transition_matrix: arr2(xs),
            vector: self.vector.clone(),
        }
    }

    pub fn with_vector<const N: usize>(&self, xs: &[f32; N]) -> Self {
        MarkovChain {
            transition_matrix: self.transition_matrix.clone(),
            vector: arr1(xs),
        }
    }

    pub fn vector_dot(&self) -> Array1<f32> {
        let p = self.vector.clone();
        let m = self.transition_matrix.clone();
        p.dot(&m)
    }

    pub fn stationary(&self) -> (Array1<f32>, i32) {
        let mut p = self.vector.clone();
        let m = self.transition_matrix.clone();
        let mut counter = 1;
        loop {
            let new_p = p.dot(&m);
            if p == new_p {
                break;
            }
            p = new_p;
            counter += 1;
        }
        (p, counter)
    }
}
