#[derive(Debug, PartialEq)]
pub struct Matrix {
    row: usize,
    column: usize,
    data: Vec<f32>
}

impl Matrix {
    pub fn new(row: usize, column: usize) -> Matrix {
        Matrix { 
                row,
                column,
                data: Vec::with_capacity(row * column) 
        }
    }

    pub fn new_with_values(row: usize, column: usize, values: Vec<f32>) -> Matrix {
        Matrix { 
            row,
            column,
            data: values
        }
    }

    pub fn get(&self, row: usize, column: usize) -> Option<f32> {
        match self.is_in_bound(row, column) {
            true => Some(self.data[row * self.column + column]),
            false => None
        }
    }

    fn is_in_bound(&self, row: usize, column: usize) -> bool {
        self.row > row && self.column > column
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_4x4_matrix() {
        let matrix_values: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 
                                        5.5, 6.5, 7.5, 8.5, 
                                        9.0, 10.0, 11.0, 12.0, 
                                        13.5, 14.5, 15.5, 16.5];
        let matrix = Matrix::new_with_values(4, 4, matrix_values);
        assert_eq!(matrix.get(0, 0), Some(1.0));
        assert_eq!(matrix.get(0, 3), Some(4.0));
        assert_eq!(matrix.get(1, 0), Some(5.5));
        assert_eq!(matrix.get(1, 2), Some(7.5));
        assert_eq!(matrix.get(2, 2), Some(11.0));
        assert_eq!(matrix.get(3, 0), Some(13.5));
        assert_eq!(matrix.get(3, 2), Some(15.5));
    }

    #[test]
    fn new_2x2_matrix() {
        let matrix_values = vec![-3.0, 5.0, 1.0, -2.0];
        let matrix = Matrix::new_with_values(2, 2, matrix_values);
        assert_eq!(matrix.get(0, 0), Some(-3.0));
        assert_eq!(matrix.get(0, 1), Some(5.0));
        assert_eq!(matrix.get(1, 0), Some(1.0));
        assert_eq!(matrix.get(1, 1), Some(-2.0));
    }

    #[test]
    fn new_3x3_matrix() {
        let matrix_values = vec![-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0];
        let matrix = Matrix::new_with_values(3, 3, matrix_values);
        assert_eq!(matrix.get(0, 0), Some(-3.0));
        assert_eq!(matrix.get(1, 1), Some(-2.0));
        assert_eq!(matrix.get(2, 2), Some(1.0));
    }
}