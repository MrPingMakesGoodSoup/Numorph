use super::*;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fmt;

// Implement Display for QuantumicNumber for easier debugging
impl fmt::Display for QuantumicNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "QuantumicNumber {{ dimension: {}, phase: {}, amplitude: {}, entropy: {}, entanglement_factor: {}, temporal_drift: }}",
            self.dimension, self.phase, self.amplitude, self.entropy, self.entanglement_factor, self.temporal_drift
        )
    }
}

pub fn serialize_quantumic_number(qn: &QuantumicNumber) -> Result<Vec<u8>, String> {
    // Add some padding for metadata, which is not part of the core quantum data
    let mut buffer: Vec<u8> = Vec::new();
    buffer.extend_from_slice(&[0u8; 4]); // Magic Number - Version of the serialization format
    buffer.extend_from_slice(&[qn.dimension as u32; 4]); // Dimension as u32
    buffer.extend_from_slice(&[qn.phase.to_le_bytes(); 8]); // Phase as f64 (8 bytes)
    buffer.extend_from_slice(&[qn.amplitude.to_le_bytes(); 8]); // Amplitude as f64
    buffer.extend_from_slice(&[qn.entropy.to_le_bytes(); 8]); // Entropy as f64
    buffer.extend_from_slice(&qn.quantum_vector.as_view().to_vec().into_iter().collect::<Vec<f64>>().as_slice()); // Quantum Vector - f64 array
    buffer.extend_from_slice(&[qn.entanglement_factor.to_le_bytes(); 8]); // Entanglement Factor
    buffer.extend_from_slice(&[qn.temporal_drift.to_le_bytes(); 8]); // Temporal Drift

    let serialized = serde_json::to_string(qn).map_err(|e| format!("Serialization error: {}", e))?;
    //Alternative serialization with json, if something goes wrong in binary
    //let data = serialized.as_bytes().to_vec();
    //Ok(data)
    Ok(buffer)
}

pub fn deserialize_quantumic_number(data: &[u8]) -> Result<QuantumicNumber, String> {
    if data.len() < 4 + 4 + 8 + 8 + 8 + (std::usize::MAX / 8) + 8 + 8{
        return Err("Data too short for QuantumicNumber".to_string());
    }

    let mut buffer = data.to_vec();
    let dimension = buffer[4..8].try_into().map_err(|_| "Invalid dimension format".to_string())?;
    let phase = buffer[8..16].try_into().map_err(|_| "Invalid phase format".to_string()).unwrap().iter().from_le_bytes([buffer[8],buffer[9],buffer[10],buffer[11],buffer[12],buffer[13],buffer[14],buffer[15]]);
    let amplitude = buffer[16..24].try_into().map_err(|_| "Invalid amplitude format".to_string()).unwrap().iter().from_le_bytes([buffer[16],buffer[17],buffer[18],buffer[19],buffer[20],buffer[21],buffer[22],buffer[23]]);
    let entropy = buffer[24..32].try_into().map_err(|_| "Invalid entropy format".to_string()).unwrap().iter().from_le_bytes([buffer[24],buffer[25],buffer[26],buffer[27],buffer[28],buffer[29],buffer[30],buffer[31]]);
    let quantum_vector_bytes = buffer[32..].to_vec();
    let quantum_vector = Array::from_vec(quantum_vector_bytes);
    let entanglement_factor = buffer[32+quantum_vector_bytes.len()..32+quantum_vector_bytes.len()+8].try_into().map_err(|_| "Invalid entanglement format".to_string()).unwrap().iter().from_le_bytes([buffer[32+quantum_vector_bytes.len()],buffer[33+quantum_vector_bytes.len()],buffer[34+quantum_vector_bytes.len()],buffer[35+quantum_vector_bytes.len()],buffer[36+quantum_vector_bytes.len()],buffer[37+quantum_vector_bytes.len()],buffer[38+quantum_vector_bytes.len()],buffer[39+quantum_vector_bytes.len()]]);
    let temporal_drift = buffer[32+quantum_vector_bytes.len()+8..32+quantum_vector_bytes.len()+16].try_into().map_err(|_| "Invalid temporal drift format".to_string()).unwrap().iter().from_le_bytes([buffer[32+quantum_vector_bytes.len()+8],buffer[33+quantum_vector_bytes.len()+8],buffer[34+quantum_vector_bytes.len()+8],buffer[35+quantum_vector_bytes.len()+8],buffer[36+quantum_vector_bytes.len()+8],buffer[37+quantum_vector_bytes.len()+8],buffer[38+quantum_vector_bytes.len()+8],buffer[39+quantum_vector_bytes.len()+8]]);


    let qn = QuantumicNumber {
        dimension,
        phase,
        amplitude,
        entropy,
        quantum_vector,
        entanglement_factor,
        temporal_drift,
    };

    Ok(qn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_deserialize() {
        let original_qn = QuantumicNumber::new(64);
        let serialized_data = serialize_quantumic_number(&original_qn).unwrap();
        let deserialized_qn = deserialize_quantumic_number(&serialized_data).unwrap();

        assert_eq!(original_qn.dimension, deserialized_qn.dimension);
        assert!((original_qn.phase - deserialized_qn.phase).abs() < 1e-6);
        assert!((original_qn.amplitude - deserialized_qn.amplitude).abs() < 1e-6);
        assert!((original_qn.entropy - deserialized_qn.entropy).abs() < 1e-6);
        assert_eq!(original_qn.quantum_vector, deserialized_qn.quantum_vector);
        assert!((original_qn.entanglement_factor - deserialized_qn.entanglement_factor).abs() < 1e-6);
        assert!((original_qn.temporal_drift - deserialized_qn.temporal_drift).abs() < 1e-6);
    }

    #[test]
    fn test_deserialization_error() {
        let short_data = vec![1; 10];
        let result = deserialize_quantumic_number(&short_data);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Data too short for QuantumicNumber");
    }
}
