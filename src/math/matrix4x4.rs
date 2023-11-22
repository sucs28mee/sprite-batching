use std::ops::{MulAssign, Mul};

use super::Dot;

pub struct Matrix4x4 {
    matrix: [[f32; 4]; 4]
}

impl Matrix4x4 {
    pub fn new(matrix: [[f32; 4]; 4]) -> Self {
        Self { matrix }
    }

    pub fn new_translation(x: f32, y: f32, z: f32) -> Self {
        Self::new(
            [
                [1f32, 0f32, 0f32, 0f32],
                [0f32, 1f32, 0f32, 0f32],
                [0f32, 0f32, 1f32, 0f32],
                [x, y, z, 1f32]
            ]
        )
    }

    pub fn new_scaling(x: f32, y: f32, z: f32) -> Self {
        Self::new(
            [
                [x, 0f32, 0f32, 0f32],
                [0f32, y, 0f32, 0f32],
                [0f32, 0f32, z, 0f32],
                [0f32, 0f32, 0f32, 1f32]
            ]
        )
    }

    pub fn new_rotation(rotation: f32) -> Matrix4x4 {
        Self::new(
            [
                [rotation.cos(), rotation.sin(), 0f32, 0f32],
                [-rotation.sin(), rotation.cos(), 0f32, 0f32],
                [0f32, 0f32, 1f32, 0f32],
                [0f32, 0f32, 0f32, 1f32]
            ]
        )
    }
    
    pub fn to_array(self) -> [[f32; 4]; 4] {
        self.matrix
    }

    fn column(&self, index: usize) -> Option<[f32; 4]> {
        Some([
            self.matrix.get(0)?.get(index)?.clone(), 
            self.matrix.get(1)?.get(index)?.clone(), 
            self.matrix.get(2)?.get(index)?.clone(), 
            self.matrix.get(3)?.get(index)?.clone()
        ])
    }
}

impl Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Matrix4x4) -> Self::Output {
        let mut matrix = [[0f32; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                matrix[i][j] = self.matrix[i].dot(&rhs.column(j).unwrap()).unwrap();
            }
        }

        Matrix4x4::new(matrix)
    }
}