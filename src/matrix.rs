use std::ops::{Mul, Neg};
use crate::{tuple::Tuple, helper::Axis};

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
                data: vec![0.0; row * column]
        }
    }

    pub fn new_with_data(row: usize, column: usize, data: Vec<f32>) -> Matrix {
        Matrix { 
            row,
            column,
            data
        }
    }

    pub fn new_4x4_with_data(data: Vec<f32>) -> Matrix {
        Matrix::new_with_data(4, 4, data)
    }

    pub fn new_3x3_with_data(data: Vec<f32>) -> Matrix {
        Matrix::new_with_data(3, 3, data)
    }

    pub fn new_identity_matrix() -> Matrix {
        Matrix::new_4x4_with_data( vec![1.0, 0.0, 0.0, 0.0,
                                        0.0, 1.0, 0.0, 0.0,
                                        0.0, 0.0, 1.0, 0.0,
                                        0.0, 0.0, 0.0, 1.0])
    }

    pub fn rotation(axis: Axis, value: f32) -> Matrix {
        let mut result = Matrix::new_identity_matrix();
        match axis {
            Axis::X => {
                result.data[5] = value.cos();
                result.data[6] = value.sin().neg();
                result.data[9] = value.sin();
                result.data[10] = value.cos();
            },
            Axis::Y => {
                result.data[0] = value.cos();
                result.data[2] = value.sin();
                result.data[8] = value.sin().neg();
                result.data[10] = value.cos();    
            },
            Axis::Z => {
                result.data[0] = value.cos();
                result.data[1] = value.sin().neg();
                result.data[4] = value.sin();
                result.data[5] = value.cos();
            }
        }
        result
    }

    pub fn shearing(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Matrix {
        let mut result = Matrix::new_identity_matrix();
        result.data[1] = xy;
        result.data[2] = xz;
        result.data[4] = yx;
        result.data[6] = yz;
        result.data[8] = zx;
        result.data[9] = zy;
        result
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Matrix {
        let mut result = Matrix::new_identity_matrix();
        result.data[3] = x;
        result.data[7] = y;
        result.data[11] = z;
        result
    }

    pub fn scaling(x: f32, y: f32, z: f32) -> Matrix {
        let mut result = Matrix::new_identity_matrix();
        result.data[0] = x;
        result.data[5] = y;
        result.data[10] = z;
        result
    }

    pub fn print(&self) {
        for i in 0..self.row {
            for j in 0..self.column {
                print!("{:?} ", self.get(i, j).unwrap());
            }
            println!("");
        }
    }

    pub fn get(&self, row: usize, column: usize) -> Option<f32> {
        match self.is_in_bound(row, column) {
            true => Some(self.data[row * self.column + column]),
            false => None
        }
    }

    pub fn set(&mut self, row: usize, column: usize, value: f32) {
        if self.is_in_bound(row, column) {
            self.data[row * self.column + column] = value;
        }
    }

    pub fn transpose(&self) -> Matrix {
        let mut result_data = Vec::with_capacity(self.row * self.column);
        for i in 0..=3 {
            result_data.push(self.data[0 + i]);
            result_data.push(self.data[4 + i]);
            result_data.push(self.data[8 + i]);
            result_data.push(self.data[12 + i]);
        }
        Matrix { row: self.row, column: self.column, data: result_data }
    }

    pub fn submatrix(&self, row: usize, column: usize) -> Matrix {
        let mut result = Matrix::new_with_data(self.row - 1, self.column - 1, 
        Vec::with_capacity((self.row - 1) * (self.column - 1)));
        // data between min_row and max_row will be deleted
        let min_row = row * self.row;
        let max_row = (row + 1) * self.row - 1;
        
        let column_indexes = self.get_indexes_of_column(column);

        let mut i = 0;
        while i < self.data.len() {
            if !(i >= min_row && i <= max_row || column_indexes.contains(&i)) {
                result.data.push(self.data[i]);
            }
            i += 1;
        }

        result
    }

    pub fn inverse(&self) -> Matrix {
        let mut result = Matrix::new_with_data(self.row, self.column, 
        Vec::with_capacity(self.row * self.column));
        let determinant = self.determinant();
        
        for i in 0..self.data.len() {
            let indexes = self.get_row_and_column(i);
            result.data.push(self.cofactor(indexes.0, indexes.1));
        }
        result = result.transpose();

        for i in 0..result.data.len() {
            result.data[i] = result.data[i] / determinant;
        }

        result
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn cofactor(&self, row: usize, column: usize) -> f32 {
        let result = self.minor(row, column);
        if (row + column) % 2 == 0 {
            return result;
        }
        result.neg()
    }

    fn minor(&self, row: usize, column: usize) -> f32 {
        let submatrix = self.submatrix(row, column);
        submatrix.determinant()
    }

    fn determinant(&self) -> f32 {
        if self.row == 2 || self.column == 2 {
            return self.data[0] * self.data[3] - self.data[1] * self.data[2]
        }
        let mut result = 0.0;
        for i in 0..self.row {
            result +=  self.data[i] * self.cofactor(0, i);
        }

        result
    }

    fn is_in_bound(&self, row: usize, column: usize) -> bool {
        self.row > row && self.column > column
    }

    /// given the column number, returns the column indexes
    fn get_indexes_of_column(&self, column_number: usize) -> Vec<usize> {
        let mut column_indexes = vec![0; self.column];
        for i in 0..self.column {
            column_indexes[i] = column_number + i * self.column;
        }
        column_indexes
    }

    fn get_row_and_column(&self, index: usize) -> (usize, usize) {
        let column = index % self.column;
        let row = index / self.row;

        (row, column)
    }

}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut new_matrix = Matrix::new(self.row, self.column);
        if self.data.len() != rhs.data.len() ||
            self.row != rhs.row ||
            self.column != rhs.column
        {
            return new_matrix; 
        }

        for row in 0..self.row {
            for column in 0..self.column {
                new_matrix.data[row * self.column + column] = self.data[row * self.column + 0] * rhs.data[0 * self.column + column] +
                                                             self.data[row * self.column + 1] * rhs.data[1 * self.column + column] +
                                                             self.data[row * self.column + 2] * rhs.data[2 * self.column + column] +
                                                             self.data[row * self.column + 3] * rhs.data[3 * self.column + column]
            }
        }
        new_matrix
    }
}

impl<'a, 'b> Mul<&'b Matrix> for &'a Matrix {
    type Output = Matrix;

    fn mul(self, rhs:&'b Matrix) -> Self::Output {
        let mut new_matrix = Matrix::new(self.row, self.column);
        if self.data.len() != rhs.data.len() ||
            self.row != rhs.row ||
            self.column != rhs.column
        {
            return new_matrix; 
        }

        for row in 0..self.row {
            for column in 0..self.column {
                new_matrix.data[row * self.column + column] = self.data[row * self.column + 0] * rhs.data[0 * self.column + column] +
                                                             self.data[row * self.column + 1] * rhs.data[1 * self.column + column] +
                                                             self.data[row * self.column + 2] * rhs.data[2 * self.column + column] +
                                                             self.data[row * self.column + 3] * rhs.data[3 * self.column + column]
            }
        }
        new_matrix
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let mut result = Tuple::new(0.0, 0.0, 0.0, 0.0);
        if self.row != 4 || self.column != 4 {
            return result;
        }
        
        let mut buffer = [0.0; 4];
        let mut i = 0;
        let mut buffer_index = 0;
        while i < 13 {
            buffer[buffer_index] = self.data[0 + i] * rhs.x + 
                        self.data[1 + i] * rhs.y +
                        self.data[2 + i] * rhs.z +
                        self.data[3 + i] * rhs.w;
            buffer_index += 1;
            i += 4;
        }

        result.x = buffer[0];
        result.y = buffer[1];
        result.z = buffer[2];
        result.w = buffer[3];
        result
    }
}

impl<'a, 'b> Mul<&'b Tuple> for &'a Matrix {
    type Output = Tuple;

    fn mul(self, rhs: &'b Tuple) -> Self::Output {
        let mut result = Tuple::new(0.0, 0.0, 0.0, 0.0);
        if self.row != 4 || self.column != 4 {
            return result;
        }
        
        let mut buffer = [0.0; 4];
        let mut i = 0;
        let mut buffer_index = 0;
        while i < 13 {
            buffer[buffer_index] = self.data[0 + i] * rhs.x + 
                        self.data[1 + i] * rhs.y +
                        self.data[2 + i] * rhs.z +
                        self.data[3 + i] * rhs.w;
            buffer_index += 1;
            i += 4;
        }

        result.x = buffer[0];
        result.y = buffer[1];
        result.z = buffer[2];
        result.w = buffer[3];
        result
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use crate::helper::equal;

    use super::*;

    #[test]
    fn new_4x4_matrix() {
        let matrix_values: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 
                                        5.5, 6.5, 7.5, 8.5, 
                                        9.0, 10.0, 11.0, 12.0, 
                                        13.5, 14.5, 15.5, 16.5];
        let matrix = Matrix::new_with_data(4, 4, matrix_values);
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
        let matrix = Matrix::new_with_data(2, 2, matrix_values);
        assert_eq!(matrix.get(0, 0), Some(-3.0));
        assert_eq!(matrix.get(0, 1), Some(5.0));
        assert_eq!(matrix.get(1, 0), Some(1.0));
        assert_eq!(matrix.get(1, 1), Some(-2.0));
    }

    #[test]
    fn new_3x3_matrix() {
        let matrix_values = vec![-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0];
        let matrix = Matrix::new_with_data(3, 3, matrix_values);
        assert_eq!(matrix.get(0, 0), Some(-3.0));
        assert_eq!(matrix.get(1, 1), Some(-2.0));
        assert_eq!(matrix.get(2, 2), Some(1.0));
    }

    #[test]
    fn multiplying_two_matrices() {
        let matrix_a = Matrix::new_4x4_with_data(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0,
                                                                9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]);
        let matrix_b = Matrix::new_4x4_with_data(vec![-2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0,
                                                                4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0]);

        let expected = Matrix::new_4x4_with_data(vec![20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0,
                                                                40.0, 58.0, 110.0, 102.0, 16.0, 26.0, 46.0, 42.0]);
        let result = matrix_a * matrix_b;

        assert_eq!(expected, result);
    }

    #[test]
    fn multiply_matrix_and_tuple() {
        let matrix = Matrix::new_4x4_with_data(vec![1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0,
                                                                8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
        let tuple = Tuple::new(1.0, 2.0, 3.0, 1.0);
        let expected = Tuple::new(18.0, 24.0, 33.0, 1.0);
        let result = matrix * tuple;
        assert_eq!(expected, result);
    }

    #[test]
    fn transpose() {
        let matrix = Matrix::new_4x4_with_data(vec![0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0,
                                                        1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0]);
        let expected = Matrix::new_4x4_with_data(vec![0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0,
                                                        3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0]);
        let result = matrix.transpose();
        assert_eq!(expected, result);
    }

    #[test]
    fn determinant() {
        let matrix = Matrix::new_with_data(2, 2, vec![1.0, 5.0, -3.0, 2.0]);
        let expected = 17.0;
        let result = matrix.determinant();
        assert_eq!(expected, result);
    }

    #[test]
    fn submatrix_of_3x3() {
        let matrix = Matrix::new_with_data(3, 3, vec![1.0, 5.0, 0.0, -3.0, 2.0, 7.0,
                                                                                    0.0, 6.0, -3.0]);
        let expected = Matrix::new_with_data(2, 2, vec![-3.0, 2.0, 0.0, 6.0]);
        let result = matrix.submatrix(0, 2);
        assert_eq!(expected, result);
    }

    #[test]
    fn submatrix_of_4x4() {
        let matrix = Matrix::new_4x4_with_data(vec![-6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0,
                                                            -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0]);
        let expected = Matrix::new_with_data(3, 3, vec![-6.0, 1.0, 6.0, -8.0,
                                                            8.0, 6.0 ,-7.0, -1.0, 1.0]);
        let result = matrix.submatrix(2, 1);
        assert_eq!(expected, result);
    }

    #[test]
    fn minor() {
        let matrix = Matrix::new_3x3_with_data(vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
        let expected = 25.0;
        let result = matrix.minor(1, 0);
        assert_eq!(expected, result);
    }

    #[test]
    fn cofactor() {
        let matrix = Matrix::new_3x3_with_data(vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
        assert_eq!(matrix.minor(0, 0), matrix.cofactor(0, 0));
        assert_eq!(matrix.minor(1, 0), matrix.cofactor(1, 0).neg());
    }

    #[test]
    fn determinant_of_3x3() {
        let matrix = Matrix::new_3x3_with_data(vec![1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0]);
        assert_eq!(56.0, matrix.cofactor(0, 0));
        assert_eq!(12.0, matrix.cofactor(0, 1));
        assert_eq!(-46.0, matrix.cofactor(0, 2));
        assert_eq!(-196.0, matrix.determinant());
    }

    #[test]
    fn determinant_of_4x4() {
        let matrix = Matrix::new_4x4_with_data(vec![-2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0,
                                                            1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0]);
        assert_eq!(690.0, matrix.cofactor(0, 0));
        assert_eq!(447.0, matrix.cofactor(0, 1));
        assert_eq!(210.0, matrix.cofactor(0, 2));
        assert_eq!(51.0, matrix.cofactor(0, 3));
        assert_eq!(-4071.0, matrix.determinant());
    }

    #[test]
    fn matrix_is_invertible() {
        let matrix = Matrix::new_4x4_with_data(vec![6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0,
                                                            4.0, -9.0, 3.0, -7.0, 9.0, 1.0, 7.0, -6.0]);
        let expected = true;
        let result = matrix.is_invertible();
        assert_eq!(expected, result);
    }

    #[test]
    fn matrix_is_not_invertible() {
        let matrix = Matrix::new_4x4_with_data(vec![-4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0,
                                                            0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0]);
        let expected = false;
        let result = matrix.is_invertible();
        assert_eq!(expected, result);
    }

    #[test]
    fn inverse_of_4x4_matrix() {
        let matrix = Matrix::new_4x4_with_data(vec![-5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 
                                                7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0, 4.0]);
        let inversed = matrix.inverse();
        let expected = Matrix::new_4x4_with_data(vec![0.21805, 0.45113, 0.24060, -0.04511,
                                                            -0.80827, -1.45677, -0.44361, 0.52068,
                                                            -0.07895, -0.22368, -0.05263, 0.19737,
                                                            -0.52256, -0.81391, -0.30075, 0.30639]);
        assert_eq!(532.0, matrix.determinant());
        assert_eq!(-160.0, matrix.cofactor(2, 3));
        assert_eq!(105.0, matrix.cofactor(3, 2));
        for i in 0..inversed.data.len() {
            assert!(equal(expected.data[i], inversed.data[i]));
        }
    }

    #[test]
    fn inversing_4x4_matrices() {
        let matrix1 = Matrix::new_4x4_with_data(vec![8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, 
                                                            -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0, -4.0]);
        let matrix2 = Matrix::new_4x4_with_data(vec![9.0, 3.0, 0.0, 9.0, -5.0, -2.0, -6.0, -3.0, 
                                                            -4.0, 9.0, 6.0, 4.0, -7.0, 6.0, 6.0, 2.0]);
        
        let expected1 = Matrix::new_4x4_with_data(vec![-0.15385, -0.15385, -0.28205, -0.53846,
                                                                -0.07692, 0.12308, 0.02564, 0.03077,
                                                                0.35897, 0.35897, 0.43590, 0.92308,
                                                                -0.69231, -0.69231, -0.76923, -1.92308]);
        let expected2 = Matrix::new_4x4_with_data(vec![-0.04074, -0.07778, 0.14444, -0.22222,
                                                                -0.07778, 0.03333, 0.36667, -0.33333,
                                                                -0.02901, -0.14630, -0.10926, 0.12963,
                                                                0.17778, 0.06667, -0.26667, 0.33333]);
        let result1 = matrix1.inverse();
        let result2 = matrix2.inverse();

        for i in 0..expected1.data.len() {
            assert!(equal(expected1.data[i], result1.data[i]));
        }
        
        for i in 0..expected2.data.len() {
            assert!(equal(expected2.data[i], result2.data[i]));
        }
    }

    #[test]
    fn multiplying_a_product_with_its_inverse() {
        let matrixa = Matrix::new_4x4_with_data(vec![3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, 
                                                            -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0, 1.0]);
                                                            
        let matrixb = Matrix::new_4x4_with_data(vec![8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 
                                                            7.0, 0.0, 5.0, 4.0, 6.0, -2.0, 0.0, 5.0]);
        let c = &matrixa * &matrixb;
        let inversed = &c * &matrixb.inverse();

        for i in 0..matrixa.data.len() {
            println!("{}) {:?}, {:?}", i, matrixa.data[i], inversed.data[i]);
            assert!(equal(matrixa.data[i], inversed.data[i]));
        }
    }

    #[test]
    fn multiplying_a_translation_matrix() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let p = Tuple::new_point(-3.0, 4.0, 5.0);
        let expected = Tuple::new_point(2.0, 1.0, 7.0);
        let result = transform * p;
        assert_eq!(expected, result); 
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let inv = transform.inverse();
        let p = Tuple::new_point(-3.0, 4.0, 5.0);
        let expected = Tuple::new_point(-8.0, 7.0, 3.0);
        let result = inv * p;
        assert_eq!(expected, result);
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let v = Tuple::new_vector(-3.0, 4.0, 5.0);
        let expected = Tuple::new_vector(-3.0, 4.0, 5.0);
        let result = transform * v;
        assert_eq!(expected, result);
    }

    #[test]
    fn scaling_matrix_applied_to_a_point() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let p = Tuple::new_point(-4.0, 6.0, 8.0);
        let expected = Tuple::new_point(-8.0, 18.0, 32.0);
        let result = transform * p;
        assert_eq!(expected, result);
    }

    #[test]
    fn scaling_matrix_applied_to_a_vector() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let p = Tuple::new_vector(-4.0, 6.0, 8.0);
        let expected = Tuple::new_vector(-8.0, 18.0, 32.0);
        let result = transform * p;
        assert_eq!(expected, result);
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse();
        let v = Tuple::new_vector(-4.0, 6.0, 8.0);
        let expected = Tuple::new_vector(-2.0, 2.0, 2.0);
        let result = inv * v;
        assert_eq!(expected, result);
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let transform = Matrix::scaling(-1.0, 1.0, 1.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        let expected = Tuple::new_point(-2.0, 3.0, 4.0);
        let result = transform * p;
        assert_eq!(expected, result);
    }

    #[test]
    fn rotating_a_point_around_x_axis() {
        let p = Tuple::new_point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation(Axis::X, PI / 4.0);
        let full_quarter = Matrix::rotation(Axis::X, PI / 2.0);

        let expected1 = Tuple::new_point(0.0, 2f32.sqrt() / 2f32, 2f32.sqrt() / 2f32);
        let expected2 = Tuple::new_point(0.0, 0.0, 1.0);

        let result1 = &half_quarter * &p;
        let result2 = &full_quarter * &p;

        assert_eq!(expected1, result1);
        assert_eq!(expected2, result2);
    }

    #[test]
    fn inverse_of_x_rotates_opposite_direction() {
        let p = Tuple::new_point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation(Axis::X, PI / 4.0);
        let inv = half_quarter.inverse();

        let expected = Tuple::new_point(0.0, 2f32.sqrt() / 2f32, 2f32.sqrt().neg() / 2f32);
        let result = inv * p;
        assert_eq!(expected, result); 
    }

    #[test]
    fn rotating_a_point_around_y_axis() {
        let p = Tuple::new_point(0.0, 0.0, 1.0);
        let half_quarter = Matrix::rotation(Axis::Y, PI / 4.0);
        let full_quarter = Matrix::rotation(Axis::Y, PI / 2.0);

        let expected1 = Tuple::new_point(2f32.sqrt() / 2f32, 0.0, 2f32.sqrt() / 2f32);
        let expected2 = Tuple::new_point(1.0, 0.0, 0.0);

        let result1 = &half_quarter * &p;
        let result2 = &full_quarter * &p;

        assert_eq!(expected1, result1);
        assert_eq!(expected2, result2);
    }

    #[test]
    fn rotating_a_point_around_z_axis() {
        let p = Tuple::new_point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation(Axis::Z, PI / 4.0);
        let full_quarter = Matrix::rotation(Axis::Z, PI / 2.0);

        let expected1 = Tuple::new_point(2f32.sqrt().neg() / 2f32, 2f32.sqrt() / 2f32, 0.0);
        let expected2 = Tuple::new_point(-1.0, 0.0, 0.0);

        let result1 = &half_quarter * &p;
        let result2 = &full_quarter * &p;

        assert_eq!(expected1, result1);
        assert_eq!(expected2, result2);
    }

    #[test]
    fn shearing_x_proportion_to_y() {
        let transform = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        
        let expected = Tuple::new_point(5.0, 3.0, 4.0);
        let result = transform * p;
        assert_eq!(expected, result);
    }

    #[test]
    fn shearing_x_proportion_to_z() {
        let transform = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        
        let expected = Tuple::new_point(6.0, 3.0, 4.0);
        let result = transform * p;
        assert_eq!(expected, result);
    }

    #[test]
    fn shearing_y_proportion_to_x() {
        let transform = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        
        let expected = Tuple::new_point(2.0, 5.0, 4.0);
        let result = transform * p;
        assert_eq!(expected, result);
    }

    #[test]
    fn shearing_y_proportion_to_z() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        
        let expected = Tuple::new_point(2.0, 7.0, 4.0);
        let result = transform * p;
        assert_eq!(expected, result);
    }

    #[test]
    fn shearing_z_proportion_to_x() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        
        let expected = Tuple::new_point(2.0, 3.0, 6.0);
        let result = transform * p;
        assert_eq!(expected, result);
    }

    #[test]
    fn shearing_z_proportion_to_y() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        
        let expected = Tuple::new_point(2.0, 3.0, 7.0);
        let result = transform * p;
        assert_eq!(expected, result);
    }

    #[test]
    fn transformation_in_scale() {
        let p = Tuple::new_point(1.0, 0.0, 1.0);
        let a = Matrix::rotation(Axis::X, PI / 2f32);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::translation(10.0, 5.0, 7.0);

        let expected_p2 = Tuple::new_point(1.0, -1.0, 0.0);
        let p2 = &a * &p;
        assert_eq!(expected_p2, p2);

        let expected_p3 = Tuple::new_point(5.0, -5.0, 0.0);
        let p3 = &b * &p2;
        assert_eq!(expected_p3, p3);

        let expected_p4 = Tuple::new_point(15.0, 0.0, 7.0);
        let p4 = &c * &p3;
        assert_eq!(expected_p4, p4);

        let expected_result = Tuple::new_point(15.0, 0.0, 7.0);
        let result = c * b * a * p;
        assert_eq!(expected_result, result);
    }

    #[test]
    fn deneme12() {
        let twelve = Tuple::new_point(0.0, 0.0, 1.0);
        let rotation_radius = PI / 6f32;
        
        let r = Matrix::rotation(Axis::Y, 3.0 * rotation_radius);
        let three = r * twelve;
        println!("{:?}", three);
    }
}