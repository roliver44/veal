use crate::core::types::VectorStats;

pub fn compute_stats(vector: &[f32]) -> VectorStats {
    if vector.is_empty() {
        return VectorStats {
            dimension: 0,
            min_val: 0.0,
            max_val: 0.0,
            mean_val: 0.0,
            l2_norm: 0.0,
        };
    }

    let dimension = vector.len();
    let mut min_val = f32::MAX;
    let mut max_val = f32::MIN;

    // Use f64 to prevent floating point precision loss
    let mut sum: f64 = 0.0;
    let mut sq_sum: f64 = 0.0;

    for &val in vector {
        if val < min_val {
            min_val = val;
        }
        if val > max_val {
            max_val = val;
        }
        let val_f64 = val as f64;
        sum += val_f64;
        sq_sum += val_f64 * val_f64;
    }

    let mean_val = (sum / (dimension as f64)) as f32;
    let l2_norm = sq_sum.sqrt() as f32;

    VectorStats {
        dimension,
        min_val,
        max_val,
        mean_val,
        l2_norm,
    }
}

pub fn compute_matrix_stats(matrix: &[Vec<f32>]) -> VectorStats {
    if matrix.is_empty() || matrix[0].is_empty() {
        return VectorStats {
            dimension: 0,
            min_val: 0.0,
            max_val: 0.0,
            mean_val: 0.0,
            l2_norm: 0.0,
        };
    }

    let dimension = matrix[0].len();
    let mut min_val = f32::MAX;
    let mut max_val = f32::MIN;

    // f64 for large matrices
    let mut sum: f64 = 0.0;
    let mut sq_sum: f64 = 0.0;
    let mut total_elements: usize = 0;

    for vector in matrix {
        for &val in vector {
            if val < min_val {
                min_val = val;
            }
            if val > max_val {
                max_val = val;
            }
            let val_f64 = val as f64;
            sum += val_f64;
            sq_sum += val_f64 * val_f64;
            total_elements += 1;
        }
    }

    let mean_val = if total_elements > 0 {
        (sum / (total_elements as f64)) as f32
    } else {
        0.0
    };

    let frobenius_norm = sq_sum.sqrt() as f32;

    VectorStats {
        dimension,
        min_val,
        max_val,
        mean_val,
        l2_norm: frobenius_norm,
    }
}