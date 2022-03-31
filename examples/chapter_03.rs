use ray_tracer::{matrix::Matrix, tuple::Tuple};

fn main() {
    println!("Inverse of identity matrix");
    let inversed_identity = Matrix::new_identity_matrix().inverse();
    inversed_identity.print();

    println!("\nMultiply a matrix by its inverse");
    let matrix = Matrix::new_4x4_with_data(vec![-2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0,
        1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0]);
    let result = matrix.inverse() * matrix;
    result.print();

    println!("\nIs there a difference between the inverse of the transpose of a matrix 
    and the transpose of the inverse");
    let matrix2 = Matrix::new_4x4_with_data(vec![-2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0,
        1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0]);
    let inverse_of_transpose = matrix2.transpose().inverse();
    let transpose_of_inverse = matrix2.inverse().transpose();

    inverse_of_transpose.print();
    println!("");
    transpose_of_inverse.print();

    println!("Identity * tuple = tuple");
    let tuple = Tuple::new(1.0, 2.0, 3.0, 1.0);
    println!("{:?}", Matrix::new_identity_matrix() * tuple);
    println!("Try to change a single element in identity");
    let mut identity_updated = Matrix::new_identity_matrix();
    identity_updated.set(0, 3, 1.0);
    let tuple2 = Tuple::new(1.0, 2.0, 3.0, 1.0);
    println!("{:?}", identity_updated * tuple2);
}