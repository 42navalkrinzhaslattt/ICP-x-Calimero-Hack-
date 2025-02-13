# Makefile for Calimero x ICP PoW Mining Project

# Configuration
DFX_VERSION=0.24.3
NODE_NAME=default
SERVER_PORT=2428
SWARM_PORT=2528

# Check if dfx is installed and has correct version
check-dfx:
	@if ! command -v dfx >/dev/null 2>&1; then \
		echo "dfx is not installed. Installing..."; \
		sh -ci "$$(curl -fsSL https://internetcomputer.org/install.sh)"; \
	fi
	@if ! dfx --version | grep -q $(DFX_VERSION); then \
		echo "Wrong dfx version. Setting to $(DFX_VERSION)..."; \
		dfxvm default $(DFX_VERSION); \
	fi

# Setup ICP environment
setup: check-dfx
	@if [ -d ".dfx" ]; then \
		echo "Existing dfx environment found. Using addon deployment..."; \
		./icp-devnet/deploy_devnet_addon.sh; \
	else \
		echo "No dfx environment found. Using fresh deployment..."; \
		./icp-devnet/deploy_devnet_fresh.sh; \
	fi

# Deploy mining contract (requires working dfx environment)
deploy-mining: setup
	@echo "Deploying mining contract..."
	dfx deploy mining_contract
	@echo "Initializing mining parameters..."
	dfx canister call mining_contract init_mining '(record { initial_difficulty = 100; reward_amount = 50 })'

# Clean up
clean:
	@echo "Cleaning up..."
	-dfx stop
	rm -rf .dfx
	rm -rf canister_ids.json

# Development setup
dev: deploy-mining
	@echo "Development environment ready!"

# Help
help:
	@echo "Calimero x ICP PoW Mining Project Makefile"
	@echo ""
	@echo "Available commands:"
	@echo "  make setup        - Set up ICP environment (auto-detects fresh/addon)"
	@echo "  make deploy-mining- Deploy mining contract (includes setup)"
	@echo "  make clean        - Clean up environment"
	@echo "  make dev          - Complete development setup"
	@echo "  make help         - Show this help message"

.PHONY: check-dfx setup deploy-mining clean dev help 