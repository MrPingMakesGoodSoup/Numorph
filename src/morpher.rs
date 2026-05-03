use super::*;
use num_traits::Float;
use rand::prelude::*;
use ndarray::Array1;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Exp;

pub fn perform_ballistic_morphing(data: Vec<u8>, iterations: usize) -> Result<Vec<u8>, NumorphError> {
    let qn = serialization::deserialize_quantumic_number(&data)?;
    let mut mutable_qn = qn.clone();

    let seed = rand::thread_rng().gen::<u64>();
    let mut rng = StdRng::seed_from_u64(seed);

    for _ in 0..iterations {
        mutable_qn.phase = mutable_qn.phase.powf(0.75) + rng.gen::<f64>().abs() * 0.01;
        mutable_qn.amplitude = mutable_qn.amplitude.ln().exp() + rng.gen::<f64>().abs() * 0.005;
        mutable_qn.entropy = (rng.gen::<f64>().powf(0.5) * mutable_qn.entropy).abs() + rng.gen::<f64>().abs() * 0.001;
        mutable_qn.entanglement_factor = (mutable_qn.entanglement_factor * (1.0 + rng.gen::<f64>().abs() * 0.002)).abs();
        mutable_qn.temporal_drift = mutable_qn.temporal_drift.sqrt() + rng.gen::<f64>().abs() * 0.0005;

        let mut drift_vector: Array1<f64> = mutable_qn.quantum_vector.random_using(Exp::new(0.1).unwrap());
        mutable_qn.quantum_vector = mutable_qn.quantum_vector + drift_vector * 0.0001;

        mutable_qn.normalize_quantum_vector();

        let interaction_strength: f64 = rng.gen::<f64>().powf(1.2);
        mutable_qn.phase = (mutable_qn.phase * interaction_strength).abs();
    }
    let serialized = serialization::serialize_quantumic_number(&mutable_qn)?;
    Ok(serialized)
}

pub fn adaptive_morphing(data: Vec<u8>, feedback_signal: Vec<f64>, iterations: usize) -> Result<Vec<u8>, NumorphError> {
    let qn = serialization::deserialize_quantumic_number(&data)?;
    let mut mutable_qn = qn.clone();
    let mut rng = rand::thread_rng();

    for i in 0..iterations {
        let feedback_index = i % feedback_signal.len();
        let feedback = feedback_signal[feedback_index];

        let adjustment_factor = rng.gen::<f64>().powf(0.8);

        mutable_qn.phase += feedback * adjustment_factor * 0.001;
        mutable_qn.amplitude += feedback * adjustment_factor * 0.0005;
        mutable_qn.entanglement_factor = (mutable_qn.entanglement_factor + feedback * adjustment_factor * 0.0001).abs();
        mutable_qn.temporal_drift = (mutable_qn.temporal_drift - feedback * adjustment_factor * 0.00005).abs();
        mutable_qn.quantum_vector = mutable_qn.quantum_vector + Array::from_vec(feedback_signal[feedback_index..feedback_index+10].to_vec()).mapv(|x| x * adjustment_factor * 0.00001);

        mutable_qn.normalize_quantum_vector();

        if rng.gen::<f64>() < 0.01 {
            mutable_qn.entropy = rng.gen::<f64>();
        }

    }

    let serialized = serialization::serialize_quantumic_number(&mutable_qn)?;
    Ok(serialized)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_ballistic_morphing() {
        let initial_data = serialization::serialize_quantumic_number(&QuantumicNumber::new(64)).unwrap();
        let morphed_data = perform_ballistic_morphing(initial_data.clone(), 5).unwrap();
        assert_ne!(initial_data, morphed_data);
    }

    #[test]
    fn test_adaptive_morphing() {
        let initial_data = serialization::serialize_quantumic_number(&QuantumicNumber::new(64)).unwrap();
        let feedback_signal = vec![1.0; 10];
        let morphed_data = adaptive_morphing(initial_data.clone(), feedback_signal, 5).unwrap();
        assert_ne!(initial_data, morphed_data);
    }
}
