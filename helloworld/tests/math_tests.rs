mod math_tests {
    use helloworld::MarkovChain;
    use ndarray::array;

    #[test]
    fn math_creation() {
        let math: MarkovChain = MarkovChain::new(2, 3);
        assert_eq!(math.transition_matrix.shape(), &[2, 3]);
        assert_eq!(math.vector.shape(), &[3]);
        assert_eq!(math.transition_matrix, array![[0., 0., 0.], [0., 0., 0.]]);
        assert_eq!(math.vector, array![0., 0., 0.]);
    }

    #[test]
    fn math_initialization() {
        let mut math: MarkovChain = MarkovChain::new(2, 3);
        math = math.with_matrix(&[[1., 1., 1.], [2., 2., 2.]]);
        math = math.with_vector(&[1., 1., 1.]);
        assert_eq!(math.transition_matrix, array![[1., 1., 1.], [2., 2., 2.]]);
        assert_eq!(math.vector, array![1., 1., 1.]);
    }

    #[test]
    fn math_multiply() {
        let mut math: MarkovChain = MarkovChain::new(3, 3);
        math = math.with_matrix(&[[1., 1., 1.], [2., 2., 2.], [3., 3., 3.]]);
        math = math.with_vector(&[0., 1., 0.]);

        let result = math.vector.dot(&math.transition_matrix);
        assert_eq!(result, array![2., 2., 2.]);
    }

    #[test]
    fn math_markov_chain_stationary() {
        let mut math: MarkovChain = MarkovChain::new(3, 3);
        math = math.with_matrix(&[[0.2, 0.6, 0.2], [0.1, 0.5, 0.4], [0.3, 0.3, 0.4]]);

        math = math.with_vector(&[1., 0., 0.]);
        let result_a = math.stationary();
        assert_eq!(result_a.0, array![0.19148943, 0.44680867, 0.36170226]);
        assert_eq!(result_a.1, 14);

        math = math.with_vector(&[0., 1., 0.]);
        let result_b = math.stationary();
        assert_eq!(result_b.0, array![0.1914894, 0.44680864, 0.3617022]);
        assert_eq!(result_b.1, 16);

        math = math.with_vector(&[0., 0., 1.]);
        let result_c = math.stationary();
        assert_eq!(result_c.0, array![0.1914894, 0.44680864, 0.3617022]);
        assert_eq!(result_c.1, 15);
    }

    #[test]
    fn math_markov_chain_eigenvecotr() {
        let mut math: MarkovChain = MarkovChain::new(3, 3);
        math = math.with_matrix(&[[0.2, 0.6, 0.2], [0.1, 0.5, 0.4], [0.3, 0.3, 0.4]]);
        math = math.with_vector(&[0.1914894, 0.44680864, 0.3617022]);

        let result = math.vector_dot();
        assert_eq!(result, math.vector);
        assert_eq!(result.sum(), 1.0000002)
    }
}
