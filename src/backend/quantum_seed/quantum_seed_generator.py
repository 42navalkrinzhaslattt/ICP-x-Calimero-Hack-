from qiskit import QuantumCircuit
from qiskit_aer import AerSimulator

def quantum_100bit_simulator():
    """Generate 100-bit random number using chunks of 20 qubits"""
    chunk_size = 20  # Qubits per circuit
    total_bits = 100
    full_bits = ""

    simulator = AerSimulator()

    # Generate 5 chunks (20 qubits × 5 = 100 bits)
    for _ in range(5):
        qc = QuantumCircuit(chunk_size, chunk_size)
        qc.h(range(chunk_size))
        qc.measure_all()

        # Execute and get raw bitstring
        job = simulator.run(qc, shots=1)
        result = job.result()
        counts = result.get_counts()

        # Extract bitstring (remove any spaces)
        bitstring = list(counts.keys())[0].replace(" ", "")
        full_bits += bitstring

    # Return as both integer and binary string
    return int(full_bits[:total_bits], 2), full_bits[:total_bits]

if __name__ == "__main__":
    random_number, binary_str = quantum_100bit_simulator()

    print("\n=== 100-bit Quantum Random Number ===")
    print(f"Decimal:\n{random_number}")
    print(f"\nBinary (no spaces):\n{binary_str}")
    print(f"\nHexadecimal:\n{hex(random_number)}")
    print(f"Bit Length: {random_number.bit_length()} bits")