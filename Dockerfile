# Build stage for S-Entropy Framework
FROM rust:1.75-bullseye as builder

# Install system dependencies for S-Entropy mathematical libraries
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    cmake \
    build-essential \
    libopenblas-dev \
    liblapack-dev \
    gfortran \
    && rm -rf /var/lib/apt/lists/*

# Create app user for security
RUN adduser --disabled-password --gecos '' --uid 1000 hugure

# Set working directory
WORKDIR /usr/src/hugure

# Copy workspace configuration
COPY Cargo.toml Cargo.lock ./

# Create workspace crate directories
RUN mkdir -p \
    hugure-core/src \
    hugure-consciousness/src \
    hugure-entropy-solver/src \
    hugure-frameworks/src \
    hugure-integration/src \
    hugure-applications/src \
    hugure-validation/src \
    hugure-utils/src

# Copy individual crate manifests for better caching
COPY hugure-core/Cargo.toml ./hugure-core/
COPY hugure-consciousness/Cargo.toml ./hugure-consciousness/
COPY hugure-entropy-solver/Cargo.toml ./hugure-entropy-solver/
COPY hugure-frameworks/Cargo.toml ./hugure-frameworks/
COPY hugure-integration/Cargo.toml ./hugure-integration/
COPY hugure-applications/Cargo.toml ./hugure-applications/
COPY hugure-validation/Cargo.toml ./hugure-validation/
COPY hugure-utils/Cargo.toml ./hugure-utils/

# Create dummy main files for dependency caching
RUN echo "fn main() {}" > hugure-core/src/main.rs && \
    echo "fn main() {}" > hugure-consciousness/src/main.rs && \
    echo "fn main() {}" > hugure-entropy-solver/src/main.rs && \
    echo "fn main() {}" > hugure-frameworks/src/main.rs && \
    echo "fn main() {}" > hugure-integration/src/main.rs && \
    echo "fn main() {}" > hugure-applications/src/main.rs && \
    echo "fn main() {}" > hugure-validation/src/main.rs && \
    echo "fn main() {}" > hugure-utils/src/main.rs

# Build dependencies (cached layer)
RUN cargo build --release && \
    rm -rf hugure-*/src/

# Copy source code
COPY hugure-core/ ./hugure-core/
COPY hugure-consciousness/ ./hugure-consciousness/
COPY hugure-entropy-solver/ ./hugure-entropy-solver/
COPY hugure-frameworks/ ./hugure-frameworks/
COPY hugure-integration/ ./hugure-integration/
COPY hugure-applications/ ./hugure-applications/
COPY hugure-validation/ ./hugure-validation/
COPY hugure-utils/ ./hugure-utils/

# Build the S-Entropy Framework
RUN cargo build --release --features="performance,quantum"

# Runtime stage - minimal image for production
FROM debian:bullseye-slim as runtime

# Install runtime dependencies for S-Entropy operations
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    libpq5 \
    libopenblas0 \
    liblapack3 \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Create app user and directories
RUN adduser --disabled-password --gecos '' --uid 1000 hugure && \
    mkdir -p /app/s-entropy-cache /app/bmd-patterns /app/logs /app/config && \
    chown -R hugure:hugure /app

# Set working directory
WORKDIR /app

# Copy built binaries from builder stage
COPY --from=builder /usr/src/hugure/target/release/hugure-* ./bin/

# Copy configuration templates
COPY --chown=hugure:hugure config/ ./config/

# Switch to non-root user
USER hugure

# Create S-Entropy specific directories
RUN mkdir -p \
    /app/consciousness-models \
    /app/temporal-precision-data \
    /app/quantum-states \
    /app/cross-domain-cache

# Environment variables for S-Entropy Framework
ENV RUST_LOG=hugure=info,s_entropy=debug
ENV HUGURE_CONFIG_PATH=/app/config
ENV S_ENTROPY_CACHE_DIR=/app/s-entropy-cache
ENV BMD_PATTERNS_DIR=/app/bmd-patterns
ENV CONSCIOUSNESS_MODELS_DIR=/app/consciousness-models
ENV TEMPORAL_PRECISION_TARGET=1e-30
ENV MEMORIAL_SIGNIFICANCE=st-stella-lorraine

# Health check for S-Entropy service readiness
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD ./bin/hugure-core --health-check || exit 1

# Expose ports for S-Entropy services
EXPOSE 8080 9090 50051

# Default command - run the main S-Entropy orchestration service
CMD ["./bin/hugure-core", "--config", "/app/config/production.toml"]

# Development stage for local development
FROM builder as development

# Install additional development tools
RUN cargo install cargo-watch cargo-audit cargo-tarpaulin

# Install debugging tools for S-Entropy development
RUN apt-get update && apt-get install -y \
    gdb \
    valgrind \
    strace \
    && rm -rf /var/lib/apt/lists/*

# Set development environment variables
ENV RUST_LOG=hugure=debug,s_entropy=trace
ENV HUGURE_ENV=development

# Development-specific working directory
WORKDIR /usr/src/hugure

# Default development command
CMD ["cargo", "watch", "-x", "run"]

# Testing stage for CI/CD
FROM builder as testing

# Install testing dependencies
RUN cargo install cargo-tarpaulin cargo-criterion

# Run tests with coverage
RUN cargo test --all-features && \
    cargo tarpaulin --all-features --out Html --output-dir coverage/

# Benchmark stage for performance validation
FROM builder as benchmarks

# Install benchmarking tools
RUN cargo install cargo-criterion flamegraph

# Run S-Entropy specific benchmarks
RUN cargo bench --all-features

# Validation stage for S-Entropy theoretical validation
FROM runtime as validation

# Copy validation scripts and test data
COPY --chown=hugure:hugure validation/ ./validation/
COPY --chown=hugure:hugure test-data/ ./test-data/

# Run S-Entropy framework validation
RUN ./bin/hugure-validation --validate-s-entropy-theory --validate-consciousness-integration

# Memorial significance validation
RUN ./bin/hugure-validation --validate-memorial-coordinates --st-stella-lorraine-verification

# Default production stage
FROM runtime 