# Hugure S-Entropy BMD Orchestration Framework Makefile
# Sacred Mathematics for Consciousness Enhancement

.PHONY: help build test run clean dev prod setup deps lint format check bench profile validate deploy logs

# Default target
.DEFAULT_GOAL := help

# Variables
RUST_VERSION := 1.75
PROJECT_NAME := hugure
DOCKER_COMPOSE := docker-compose
CARGO := cargo

# S-Entropy specific variables
S_ENTROPY_PRECISION := 1e-30
CONSCIOUSNESS_MODE := enhancement_only
MEMORIAL_SIGNIFICANCE := st-stella-lorraine

# Colors for output
RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[0;33m
BLUE := \033[0;34m
PURPLE := \033[0;35m
CYAN := \033[0;36m
WHITE := \033[0;37m
NC := \033[0m # No Color

# Help target
help: ## Show this help message
	@echo ""
	@echo "$(PURPLE)🌟 Hugure S-Entropy BMD Orchestration Framework 🌟$(NC)"
	@echo "$(BLUE)Sacred Mathematics for Consciousness Enhancement$(NC)"
	@echo ""
	@echo "$(YELLOW)Available targets:$(NC)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "$(CYAN)%-20s$(NC) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(GREEN)Memorial Significance: $(MEMORIAL_SIGNIFICANCE)$(NC)"
	@echo "$(GREEN)S-Entropy Precision Target: $(S_ENTROPY_PRECISION)$(NC)"

# Setup and Installation
setup: ## Setup development environment for S-Entropy framework
	@echo "$(BLUE)Setting up Hugure S-Entropy development environment...$(NC)"
	rustup default $(RUST_VERSION)
	rustup component add clippy rustfmt
	$(CARGO) install cargo-watch cargo-audit cargo-tarpaulin cargo-criterion flamegraph
	@mkdir -p s-entropy-cache bmd-patterns consciousness-models temporal-precision-data
	@echo "$(GREEN)✅ S-Entropy development environment ready!$(NC)"

deps: ## Install and update dependencies
	@echo "$(BLUE)Installing S-Entropy dependencies...$(NC)"
	$(CARGO) update
	@echo "$(GREEN)✅ Dependencies updated$(NC)"

# Development
dev: ## Start development environment with hot reloading
	@echo "$(BLUE)Starting S-Entropy development environment...$(NC)"
	$(CARGO) watch -x "run --bin hugure-core"

dev-docker: ## Start development environment with Docker
	@echo "$(BLUE)Starting S-Entropy Docker development environment...$(NC)"
	$(DOCKER_COMPOSE) -f docker-compose.yml -f docker-compose.dev.yml up --build

# Building
build: ## Build the S-Entropy framework (debug mode)
	@echo "$(BLUE)Building Hugure S-Entropy framework...$(NC)"
	$(CARGO) build --all-features

build-release: ## Build optimized release version
	@echo "$(BLUE)Building optimized S-Entropy framework...$(NC)"
	$(CARGO) build --release --all-features

build-quantum: ## Build with quantum computing features
	@echo "$(BLUE)Building S-Entropy framework with quantum features...$(NC)"
	$(CARGO) build --release --features="quantum,performance"

build-docker: ## Build Docker images for all S-Entropy services
	@echo "$(BLUE)Building S-Entropy Docker images...$(NC)"
	docker build -t hugure/core:latest --target runtime .
	docker build -t hugure/dev:latest --target development .
	@echo "$(GREEN)✅ Docker images built$(NC)"

# Testing
test: ## Run all tests for S-Entropy framework
	@echo "$(BLUE)Running S-Entropy tests...$(NC)"
	$(CARGO) test --all-features
	@echo "$(GREEN)✅ All tests passed$(NC)"

test-consciousness: ## Test consciousness integration modules
	@echo "$(BLUE)Testing consciousness BMD integration...$(NC)"
	$(CARGO) test --package hugure-consciousness --all-features

test-entropy-solver: ## Test entropy solver service
	@echo "$(BLUE)Testing tri-dimensional S-entropy solver...$(NC)"
	$(CARGO) test --package hugure-entropy-solver --all-features

test-coverage: ## Generate test coverage report
	@echo "$(BLUE)Generating S-Entropy test coverage...$(NC)"
	$(CARGO) tarpaulin --all-features --out Html --output-dir coverage/
	@echo "$(GREEN)✅ Coverage report generated in coverage/$(NC)"

test-integration: ## Run integration tests with Docker
	@echo "$(BLUE)Running S-Entropy integration tests...$(NC)"
	$(DOCKER_COMPOSE) -f docker-compose.test.yml up --build --abort-on-container-exit
	$(DOCKER_COMPOSE) -f docker-compose.test.yml down

# Validation and Quality
validate: ## Validate S-Entropy theoretical framework
	@echo "$(BLUE)Validating S-Entropy theoretical framework...$(NC)"
	$(CARGO) run --bin hugure-validation -- --validate-s-entropy-theory
	$(CARGO) run --bin hugure-validation -- --validate-consciousness-integration
	$(CARGO) run --bin hugure-validation -- --validate-tri-dimensional-alignment
	@echo "$(GREEN)✅ S-Entropy framework validation complete$(NC)"

validate-memorial: ## Validate memorial significance coordinates
	@echo "$(PURPLE)Validating St. Stella-Lorraine memorial coordinates...$(NC)"
	$(CARGO) run --bin hugure-validation -- --validate-memorial-coordinates --st-stella-lorraine-verification
	@echo "$(GREEN)✅ Memorial significance validated$(NC)"

lint: ## Run linting and code analysis
	@echo "$(BLUE)Running S-Entropy code analysis...$(NC)"
	$(CARGO) clippy --all-features -- -D warnings
	$(CARGO) audit
	@echo "$(GREEN)✅ Code analysis complete$(NC)"

format: ## Format code according to S-Entropy standards
	@echo "$(BLUE)Formatting S-Entropy code...$(NC)"
	$(CARGO) fmt --all
	@echo "$(GREEN)✅ Code formatted$(NC)"

check: ## Check code without building
	@echo "$(BLUE)Checking S-Entropy code...$(NC)"
	$(CARGO) check --all-features

# Benchmarking and Performance
bench: ## Run S-Entropy performance benchmarks
	@echo "$(BLUE)Running S-Entropy benchmarks...$(NC)"
	$(CARGO) bench --all-features
	@echo "$(GREEN)✅ Benchmarks complete$(NC)"

bench-consciousness: ## Benchmark consciousness enhancement performance
	@echo "$(BLUE)Benchmarking consciousness BMD performance...$(NC)"
	$(CARGO) bench --package hugure-consciousness

bench-temporal: ## Benchmark ultra-precision temporal coordination
	@echo "$(BLUE)Benchmarking temporal precision (target: $(S_ENTROPY_PRECISION))...$(NC)"
	$(CARGO) bench --package hugure-frameworks -- temporal_precision

profile: ## Profile S-Entropy framework performance
	@echo "$(BLUE)Profiling S-Entropy performance...$(NC)"
	$(CARGO) build --release --all-features
	sudo flamegraph target/release/hugure-core
	@echo "$(GREEN)✅ Flamegraph generated$(NC)"

# Running Services
run: ## Run the main S-Entropy orchestration service
	@echo "$(BLUE)Starting Hugure S-Entropy BMD orchestration...$(NC)"
	$(CARGO) run --bin hugure-core --release

run-entropy-solver: ## Run the tri-dimensional entropy solver service
	@echo "$(BLUE)Starting S-Entropy solver service...$(NC)"
	$(CARGO) run --bin hugure-entropy-solver --release

run-consciousness: ## Run consciousness enhancement service
	@echo "$(BLUE)Starting consciousness enhancement service...$(NC)"
	$(CARGO) run --bin hugure-consciousness --release

# Production Deployment
prod: ## Start production environment with Docker
	@echo "$(BLUE)Starting S-Entropy production environment...$(NC)"
	$(DOCKER_COMPOSE) up -d --build
	@echo "$(GREEN)✅ Production environment started$(NC)"

prod-stop: ## Stop production environment
	@echo "$(BLUE)Stopping S-Entropy production environment...$(NC)"
	$(DOCKER_COMPOSE) down
	@echo "$(GREEN)✅ Production environment stopped$(NC)"

deploy: ## Deploy to production (requires additional configuration)
	@echo "$(BLUE)Deploying S-Entropy framework to production...$(NC)"
	@echo "$(YELLOW)Note: Ensure production configuration is properly set$(NC)"
	$(DOCKER_COMPOSE) -f docker-compose.yml -f docker-compose.prod.yml up -d --build

# Monitoring and Debugging
logs: ## View logs from all S-Entropy services
	@echo "$(BLUE)Viewing S-Entropy service logs...$(NC)"
	$(DOCKER_COMPOSE) logs -f

logs-core: ## View core service logs
	$(DOCKER_COMPOSE) logs -f hugure-core

logs-consciousness: ## View consciousness service logs
	$(DOCKER_COMPOSE) logs -f hugure-consciousness

logs-entropy: ## View entropy solver logs
	$(DOCKER_COMPOSE) logs -f hugure-entropy-solver

health: ## Check health of all S-Entropy services
	@echo "$(BLUE)Checking S-Entropy service health...$(NC)"
	curl -f http://localhost:8080/health || echo "$(RED)Core service unhealthy$(NC)"
	curl -f http://localhost:8081/health || echo "$(RED)Entropy solver unhealthy$(NC)"
	curl -f http://localhost:8082/health || echo "$(RED)Consciousness service unhealthy$(NC)"

# Database Operations
db-setup: ## Setup S-Entropy database schema
	@echo "$(BLUE)Setting up S-Entropy database...$(NC)"
	$(DOCKER_COMPOSE) exec postgres psql -U hugure -d hugure_s_entropy -f /docker-entrypoint-initdb.d/init-db.sql

db-migrate: ## Run database migrations
	@echo "$(BLUE)Running S-Entropy database migrations...$(NC)"
	$(CARGO) run --bin hugure-migration

db-reset: ## Reset S-Entropy database (WARNING: destroys data)
	@echo "$(RED)⚠️  WARNING: This will destroy all S-Entropy data!$(NC)"
	@read -p "Are you sure? [y/N] " confirm && [ "$$confirm" = "y" ]
	$(DOCKER_COMPOSE) down -v
	$(DOCKER_COMPOSE) up -d postgres
	sleep 5
	make db-setup

# Cleaning
clean: ## Clean build artifacts and caches
	@echo "$(BLUE)Cleaning S-Entropy build artifacts...$(NC)"
	$(CARGO) clean
	docker system prune -f
	@echo "$(GREEN)✅ Clean complete$(NC)"

clean-docker: ## Clean Docker containers and volumes
	@echo "$(BLUE)Cleaning S-Entropy Docker resources...$(NC)"
	$(DOCKER_COMPOSE) down -v --remove-orphans
	docker system prune -a -f

clean-cache: ## Clean S-Entropy specific caches
	@echo "$(BLUE)Cleaning S-Entropy caches...$(NC)"
	rm -rf s-entropy-cache/* bmd-patterns/* consciousness-models/* temporal-precision-data/*
	@echo "$(GREEN)✅ S-Entropy caches cleaned$(NC)"

# Documentation
docs: ## Generate and serve documentation
	@echo "$(BLUE)Generating S-Entropy documentation...$(NC)"
	$(CARGO) doc --all-features --open

docs-book: ## Build documentation book
	@echo "$(BLUE)Building S-Entropy documentation book...$(NC)"
	mdbook build docs/
	mdbook serve docs/

# Special S-Entropy Operations
s-entropy-calibrate: ## Calibrate tri-dimensional S-entropy measurements
	@echo "$(PURPLE)Calibrating S-Entropy tri-dimensional framework...$(NC)"
	$(CARGO) run --bin hugure-calibration -- --precision $(S_ENTROPY_PRECISION)

consciousness-enhance: ## Run consciousness enhancement validation
	@echo "$(PURPLE)Running consciousness enhancement validation...$(NC)"
	$(CARGO) run --bin hugure-consciousness -- --mode $(CONSCIOUSNESS_MODE) --validate

memorial-honor: ## Honor St. Stella-Lorraine through mathematical validation
	@echo "$(PURPLE)Honoring St. Stella-Lorraine through sacred mathematics...$(NC)"
	$(CARGO) run --bin hugure-validation -- --memorial-honor --st-stella-lorraine
	@echo "$(GREEN)✅ Memorial honor complete - Sacred mathematics validated$(NC)"

# Quick development workflow
quick: format lint test ## Quick development workflow: format, lint, test
	@echo "$(GREEN)✅ Quick development workflow complete$(NC)"

# Full validation workflow
full-validate: build test validate validate-memorial bench ## Complete validation workflow
	@echo "$(GREEN)✅ Full S-Entropy validation complete$(NC)"

# Environment status
status: ## Show S-Entropy framework status
	@echo "$(PURPLE)🌟 Hugure S-Entropy Framework Status 🌟$(NC)"
	@echo "$(BLUE)Rust Version:$(NC) $$(rustc --version)"
	@echo "$(BLUE)Cargo Version:$(NC) $$(cargo --version)"
	@echo "$(BLUE)S-Entropy Precision Target:$(NC) $(S_ENTROPY_PRECISION)"
	@echo "$(BLUE)Consciousness Mode:$(NC) $(CONSCIOUSNESS_MODE)"
	@echo "$(BLUE)Memorial Significance:$(NC) $(MEMORIAL_SIGNIFICANCE)"
	@echo "$(BLUE)Docker Status:$(NC)"
	@docker --version 2>/dev/null || echo "$(RED)Docker not available$(NC)"
	@echo "$(BLUE)Docker Compose Status:$(NC)"
	@docker-compose --version 2>/dev/null || echo "$(RED)Docker Compose not available$(NC)" 