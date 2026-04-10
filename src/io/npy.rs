use ndarray::Array2;
use ndarray_npy::ReadNpyExt;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use crate::core::types::VectorData;

pub fn load_npy_vector(file_path: &Path) -> Result<VectorData, Box<dyn Error>> {
    let file = File::open(file_path).map_err(|e| format!("Failed to open NPY file '{:?}': {}", file_path, e))?;

    // attempt load as 2d array aka a matrix
    if let Ok(matrix) = Array2::<f32>::read_npy(&file) {
        let mut vec_matrix = Vec::with_capacity(matrix.nrows());
        for row in matrix.outer_iter() {
            vec_matrix.push(row.to_vec());
        }
        return Ok(VectorData::Matrix(vec_matrix));
    }

    // try to load single vector
    let mut file = File::open(file_path)?;
    if let Ok(vector) = ndarray::Array1::<f32>::read_npy(&mut file) {
        return Ok(VectorData::Single(vector.to_vec()));
    }

    // cast to f32 for memory opt
    let mut file = File::open(file_path)?;
    if let Ok(matrix) = Array2::<f64>::read_npy(&mut file) {
        let mut vec_matrix = Vec::with_capacity(matrix.nrows());
        for row in matrix.outer_iter() {
            let casted_row: Vec<f32> = row.iter().map(|&x| x as f32).collect();
            vec_matrix.push(casted_row);
        }
        return Ok(VectorData::Matrix(vec_matrix));
    }

    let mut file = File::open(file_path)?;
    if let Ok(vector) = ndarray::Array1::<f64>::read_npy(&mut file) {
        let casted_vec: Vec<f32> = vector.iter().map(|&x| x as f32).collect();
        return Ok(VectorData::Single(casted_vec));
    }

    Err("Failed to parse NPY file. The data must be a 1D or 2D array of floats.".into())
}