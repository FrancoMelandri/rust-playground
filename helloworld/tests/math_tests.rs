mod math_tests {
    use helloworld::Math;
    use ndarray::{array};

    #[test]
    fn math_creation() {
        let math: Math = Math::new(2, 3);
        assert_eq!(math.matrix.shape(), &[2, 3]);
        assert_eq!(math.vector.shape(), &[3]);
        assert_eq!(math.matrix, array![[0., 0., 0.],[0., 0., 0.]]);
        assert_eq!(math.vector, array![0., 0., 0.]);
    }

    #[test]
    fn math_initialization() {
        let mut math: Math = Math::new(2,3);
        math = math.with_matrix(&[[1.,1.,1.], [2.,2.,2.]]);
        math = math.with_vector(&[1.,1.,1.]);
        assert_eq!(math.matrix, array![[1.,1.,1.], [2.,2.,2.]]);
        assert_eq!(math.vector, array![1.,1.,1.]);
    }

    #[test]
    fn math_multiply() {
        let mut math: Math = Math::new(3,3);
        math = math.with_matrix(&[[1.,1.,1.], [2.,2.,2.], [3.,3.,3.]]);
        math = math.with_vector(&[0.,1.,0.]);

        let result = math.vector.dot(&math.matrix);
        assert_eq!(result, array![2.,2.,2.]);
    }
}
