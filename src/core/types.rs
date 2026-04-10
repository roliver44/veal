use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorStats {
    pub dimension: usize,
    pub min_val: f32,
    pub max_val: f32,
    pub mean_val: f32,
    pub l2_norm: f32,
}

#[derive(Debug, Clone)]
pub enum VectorData {
    Single(Vec<f32>),
    Matrix(Vec<Vec<f32>>),
}