use numorph::quantumic_number::QuantumicNumber;
use numorph::serialization::serialize_quantumic_number;
use numorph::morphing::perform_ballistic_morphing;
use numorph::error_handling::NumorphError;
use std::time::{Instant};
use log::{info, debug, warn, error};
use rand::Rng;
use ndarray::Array;
use ndarray::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json;

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
}

pub mod serialization {
    use super::*;

    pub fn serialize_quantumic_number(qn: &QuantumicNumber) -> Result<Vec<u8>, String> {
        let serialized = serde_json::to_string(qn).map_err(|e| format!("Serialization error: {}", e))?;
        let data = serialized.as_bytes().to_vec();
        Ok(data)
    }

    pub fn deserialize_quantumic_number(data: &[u8]) -> Result<QuantumicNumber, String> {
        let serialized = String::from_utf8(data.to_vec()).map_err(|e| format!("Invalid UTF-8: {}", e))?;
        serde_json::from_str(&serialized).map_err(|e| format!("Deserialization error: {}", e))
    }
}

pub mod morphing {
    use super::*;

    pub fn perform_ballistic_morphing(data: Vec<u8>, iterations: usize) -> Result<Vec<u8>, NumorphError> {
        let qn = serialization::deserialize_quantumic_number(&data)?;
        let mut qn_copy = qn.clone();
        for _ in 0..iterations {
            qn_copy.phase += 0.01 * rand::thread_rng().gen::<f64>();
            qn_copy.amplitude += 0.005 * rand::thread_rng().gen::<f64>();
            qn_copy.normalize_quantum_vector();

            //Simulated entanglement shift
            qn_copy.entanglement_factor += 0.001 * rand::thread_rng().gen::<f64>();
            if qn_copy.entanglement_factor > 1.0 {
                qn_copy.entanglement_factor = 0.9;
            }
        }
        let serialized = serialization::serialize_quantumic_number(&qn_copy)?;
        Ok(serialized)
    }
}

pub mod error_handling {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum NumorphError {
        #[error("Serialization failed: {0}")]
        SerializationError(String),
        #[error("Morphing failed: {0}")]
        MorphingError(String),
        #[error("Unexpected error: {0}")]
        GenericError(String),
    }
}

use thiserror::Error;

fn main() -> Result<(), NumorphError> {
    env_logger::init();

    info!("Numorph Application Starting...");

    let qn = QuantumicNumber::new(128);

    debug!("Quantumic Number: {:?}", qn);

    let start_time = Instant::now();

    let serialized_data = serialize_quantumic_number(&qn)?;

    let end_serialization_time = Instant::now();
    let serialization_duration = end_serialization_time.duration_since(start_time);

    info!("Serialization Time: {:?}", serialization_duration);

    let morphed_data = perform_ballistic_morphing(serialized_data.clone(), 5)?;

    let end_morphing_time = Instant::now();
    let morphing_duration = end_morphing_time.duration_since(end_serialization_time);

    info!("Morphing Time: {:?}", morphing_duration);

    let deserialized_qn = serialization::deserialize_quantumic_number(&morphed_data)?;
    debug!("Deserialized Quantumic Number: {:?}", deserialized_qn);

    info!("Numorph Application Completed.");
    Ok(())
}
