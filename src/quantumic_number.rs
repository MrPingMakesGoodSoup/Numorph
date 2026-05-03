use ndarray::Array;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumicNumber {
    pub dimension: usize,
    pub phase: f64,
    pub amplitude: f64,
    pub entropy: f64,
    pub quantum_vector: Array<f64, _>,
    pub entanglement_factor: f64,
    pub temporal_drift: f64,
}

impl QuantumicNumber {
    pub fn new(dimension: usize) -> Self {
        let mut rng = rand::thread_rng();
        let quantum_vector: Vec<f64> = (0..dimension).map(|_| rng.gen::<f64>()).collect();
        QuantumicNumber {
            dimension,
            phase: rng.gen::<f64>(),
            amplitude: rng.gen::<f64>(),
            entropy: rng.gen::<f64>(),
            quantum_vector: Array::from_vec(quantum_vector),
            entanglement_factor: rng.gen::<f64>(),
            temporal_drift: rng.gen::<f64>(),
        }
    }

    pub fn normalize_quantum_vector(&mut self) {
        let norm: f64 = self.quantum_vector.iter().map(|&x| x * x).sum();
        let norm = norm.sqrt();
        if norm > 0.0 {
            self.quantum_vector = self.quantum_vector / norm;
        }
    }

    pub fn calculate_entanglement(&self) -> f64 {
        // Dummy entanglement calculation for demonstration.
        self.phase.abs() + self.amplitude.abs() + self.temporal_drift.abs()
    }

    pub fn shift_temporal(&mut self, shift_amount: f64) {
        self.temporal_drift += shift_amount;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantumic_number_creation() {
        let qn = QuantumicNumber::new(10);
        assert_eq!(qn.dimension, 10);
        assert!(qn.phase >= 0.0 && qn.phase < 1.0);
        assert!(qn.amplitude >= 0.0 && qn.amplitude < 1.0);
        assert!(qn.entropy >= 0.0 && qn.entropy < 1.0);
    }

    #[test]
    fn test_quantumic_number_normalization() {
        let mut qn = QuantumicNumber {
            dimension: 5,
            phase: 0.5,
            amplitude: 0.5,
            entropy: 0.5,
            quantum_vector: Array::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]),
            entanglement_factor: 0.5,
            temporal_drift: 0.5,
        };
        qn.normalize_quantum_vector();
        let norm = qn.quantum_vector.iter().map(|&x| x * x).sum::<f64>().sqrt();
        assert!((norm - 1.0).abs() < 1e-6);
    }
}
