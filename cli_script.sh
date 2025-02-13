#!/bin/bash

# Exit on error
set -e

# Create virtual environment if it doesn't exist
if [ ! -d "venv" ]; then
    echo "Creating virtual environment..."
    python3 -m venv venv
fi

# Activate virtual environment
echo "Activating virtual environment..."
source venv/bin/activate

# Install required packages
echo "Installing required packages..."
pip install qiskit qiskit-aer

# Run the quantum seed generator
echo "Running quantum seed generator..."
python src/backend/quantum_seed/quantum_seed_generator.py

# Deactivate virtual environment
deactivate
