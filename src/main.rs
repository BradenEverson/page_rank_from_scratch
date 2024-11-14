use page_rank_from_scratch::{
    matrix::Matrix,
    vector::{General, Vector},
};

fn main() {
    let vec1: Vector<3> = Vector::from_data(&[1f32, 2f32, 3f32]).unwrap();
    let vec2: Vector<3> = Vector::from_data(&[4f32, 5f32, 6f32]).unwrap();
    let vec3: Vector<3> = Vector::from_data(&[7f32, 8f32, 9f32]).unwrap();

    let mat = Matrix::from_vectors([vec1, vec2, vec3]);
    let mat = mat * 2f32;
    println!("{mat:?}");
    let prob = Vector::<3, General>::from_data(&[0.25, 0.25, 0.5])
        .unwrap()
        .probability_vector()
        .unwrap();
    println!("{prob:?}")
}
