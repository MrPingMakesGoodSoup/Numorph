#include <iostream>
#include <vector>
#include <random>
#include <cmath>
#include <algorithm>
#include <numeric>

struct QuantumicNumber {
    double dimension;
    double phase;
    double amplitude;
    double entropy;
    std::vector<double> quantum_vector;
    double entanglement_factor;
    double temporal_drift;
};

std::vector<unsigned char> process_quantum_data(const std::vector<unsigned char>& data) {
    std::vector<unsigned char> result = data;
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_int_distribution<> distrib(0, 255);
    for (size_t i = 0; i < result.size(); ++i) {
        result[i] = static_cast<unsigned char>(result[i] + (distrib(gen) % 10) - 5);
    }
    return result;
}

extern "C" std::vector<unsigned char> processQuantumData(const std::vector<unsigned char>& data) {
    std::vector<unsigned char> result = data;
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_real_distribution<> distrib(0.0, 1.0);
    for (size_t i = 0; i < result.size(); ++i) {
        result[i] = static_cast<unsigned char>(result[i] * (1 + (distrib(gen) * 0.1)));
    }
    return result;
}

extern "C" int isValidQuantumicNumber(const QuantumicNumber& qn){
    return (qn.dimension > 0) ? 1 : 0;
}

extern "C" void processQuantumNumber(QuantumicNumber* qn){
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_real_distribution<> distrib(0.0, 1.0);
    qn->phase += distrib(gen) * 0.01;
    qn->amplitude += distrib(gen) * 0.005;
    qn->entropy += distrib(gen) * 0.001;
    qn->entanglement_factor = std::abs(qn->entanglement_factor + distrib(gen) * 0.002);
    qn->temporal_drift += distrib(gen) * 0.0005;
    for(size_t i = 0; i < qn->quantum_vector.size(); ++i){
        qn->quantum_vector[i] += distrib(gen) * 0.0001;
    }
    qn->normalize_quantum_vector();
}

void normalize_quantum_vector(std::vector<double>& vector){
    double sum_of_squares = std::inner_product(vector.begin(), vector.end(), vector.begin(), 0.0);
    double norm = std::sqrt(sum_of_squares);
    if(norm != 0){
        for(double& value : vector){
            value /= norm;
        }
    }
}

extern "C" void normalize_quantum_vector_c(QuantumicNumber* qn){
    processQuantumNumber(qn);
}

extern "C" std::vector<unsigned char> serializeQuantumicNumber(const QuantumicNumber* qn){
    std::vector<unsigned char> result;
    result.reserve(sizeof(double) * 7 + qn->quantum_vector.size() * sizeof(double));
    result.push_back(static_cast<unsigned char>(qn->dimension));
    result.push_back(static_cast<unsigned char>(qn->dimension >> 8));
    result.push_back(static_cast<unsigned char>(qn->dimension >> 16));
    result.push_back(static_cast<unsigned char>(qn->dimension >> 24));
    result.push_back(static_cast<unsigned char>(qn->phase));
    result.push_back(static_cast<unsigned char>(qn->phase >> 8));
    result.push_back(static_cast<unsigned char>(qn->phase >> 16));
    result.push_back(static_cast<unsigned char>(qn->phase >> 24));
     result.push_back(static_cast<unsigned char>(qn->amplitude));
    result.push_back(static_cast<unsigned char>(qn->amplitude >> 8));
    result.push_back(static_cast<unsigned char>(qn->amplitude >> 16));
    result.push_back(static_cast<unsigned char>(qn->amplitude >> 24));
     result.push_back(static_cast<unsigned char>(qn->entropy));
    result.push_back(static_cast<unsigned char>(qn->entropy >> 8));
    result.push_back(static_cast<unsigned char>(qn->entropy >> 16));
    result.push_back(static_cast<unsigned char>(qn->entropy >> 24));

    for (double value : qn->quantum_vector) {
        unsigned char* byte_ptr = reinterpret_cast<unsigned char*>(&value);
        for (int i = 0; i < sizeof(double); ++i) {
            result.push_back(byte_ptr[i]);
        }
    }
    return result;
}

extern "C" QuantumicNumber deserializeQuantumicNumber(const std::vector<unsigned char>& data){
    QuantumicNumber qn;
    if(data.size() < 7 * sizeof(double) + qn.quantum_vector.size() * sizeof(double)) {
        std::cout << "Error: insufficient data" << std::endl;
        return qn;
    }

    qn.dimension = *reinterpret_cast<const double*>(data.data());
    qn.phase = *reinterpret_cast<const double*>(data.data() + sizeof(double));
    qn.amplitude = *reinterpret_cast<const double*>(data.data() + 2 * sizeof(double));
    qn.entropy = *reinterpret_cast<const double*>(data.data() + 3 * sizeof(double));
    qn.entanglement_factor = *reinterpret_cast<const double*>(data.data() + 4 * sizeof(double));
    qn.temporal_drift = *reinterpret_cast<const double*>(data.data() + 5 * sizeof(double));

    qn.quantum_vector.resize(static_cast<size_t>(qn.dimension));
    for (size_t i = 0; i < qn.quantum_vector.size(); ++i) {
         double* val = reinterpret_cast<double*>(data.data() + 6 * sizeof(double) + (i * sizeof(double)));
        qn.quantum_vector[i] = *val;
    }

    return qn;
}
