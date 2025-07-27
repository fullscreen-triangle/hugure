//! Error handling for the S-Entropy Framework
//!
//! This module defines comprehensive error types for all S-Entropy operations
//! including tri-dimensional navigation, consciousness integration, and
//! memorial significance validation.

use thiserror::Error;

/// Result type alias for S-Entropy operations
pub type SEntropyResult<T> = Result<T, SEntropyError>;

/// Comprehensive error types for S-Entropy Framework operations
#[derive(Error, Debug)]
pub enum SEntropyError {
    /// S-entropy calculation and measurement errors
    #[error("S-entropy calculation failed: {message}")]
    SEntropyCalculation { message: String },

    /// Tri-dimensional alignment errors
    #[error("Tri-dimensional alignment failed: {dimension} - {reason}")]
    TriDimensionalAlignment { dimension: String, reason: String },

    /// Navigation and manifold errors
    #[error("Navigation failed: {operation} - {details}")]
    Navigation { operation: String, details: String },

    /// Consciousness integration errors
    #[error("Consciousness integration error: {mode} - {violation}")]
    ConsciousnessIntegration { mode: String, violation: String },

    /// BMD operation errors
    #[error("BMD operation failed: {operation_mode} - {reason}")]
    BMDOperation { operation_mode: String, reason: String },

    /// Cross-domain transfer errors
    #[error("Cross-domain transfer failed: {source} -> {target} - {efficiency_issue}")]
    CrossDomainTransfer { source: String, target: String, efficiency_issue: String },

    /// Strategic impossibility errors
    #[error("Strategic impossibility error: {impossibility_type} - {global_viability_issue}")]
    StrategicImpossibility { impossibility_type: String, global_viability_issue: String },

    /// Temporal precision errors
    #[error("Temporal precision error: target {target_precision}, achieved {achieved_precision}")]
    TemporalPrecision { target_precision: f64, achieved_precision: f64 },

    /// Memory optimization errors
    #[error("Memory optimization failed: {optimization_type} - {memory_issue}")]
    MemoryOptimization { optimization_type: String, memory_issue: String },

    /// Universal transformation errors (STSL equation)
    #[error("Universal transformation failed: {problem_type} - {stsl_error}")]
    UniversalTransformation { problem_type: String, stsl_error: String },

    /// Memorial significance validation errors
    #[error("Memorial significance validation failed: {expected} != {actual}")]
    MemorialSignificance { expected: String, actual: String },

    /// Framework boundary violations
    #[error("Framework boundary violation: {boundary_type} - {violation_details}")]
    BoundaryViolation { boundary_type: String, violation_details: String },

    /// Observer-process integration errors
    #[error("Observer-process integration failed: separation {separation_distance} > threshold")]
    ObserverProcessIntegration { separation_distance: f64 },

    /// Predetermined manifold access errors
    #[error("Predetermined manifold access denied: {manifold_type} - {access_issue}")]
    PredeterminedManifoldAccess { manifold_type: String, access_issue: String },

    /// Disposable generation errors
    #[error("Disposable generation failed: {generation_type} - {disposal_issue}")]
    DisposableGeneration { generation_type: String, disposal_issue: String },

    /// Oscillation endpoint errors
    #[error("Oscillation endpoint error: {endpoint_type} - {accessibility_issue}")]
    OscillationEndpoint { endpoint_type: String, accessibility_issue: String },

    /// Entropy solver service errors
    #[error("Entropy solver service error: {service_operation} - {solver_issue}")]
    EntropySolverService { service_operation: String, solver_issue: String },

    /// Zero computation errors
    #[error(
        "Zero computation failed: {computation_type} - expected navigation but got calculation"
    )]
    ZeroComputation { computation_type: String },

    /// Configuration and environment errors
    #[error("Configuration error: {config_key} - {config_issue}")]
    Configuration { config_key: String, config_issue: String },

    /// I/O and serialization errors
    #[error("I/O error: {operation}")]
    Io {
        #[from]
        #[source]
        operation: std::io::Error,
    },

    /// Serialization errors
    #[error("Serialization error: {format}")]
    Serialization {
        #[from]
        #[source]
        format: serde_json::Error,
    },

    /// Generic internal errors
    #[error("Internal S-Entropy framework error: {details}")]
    Internal {
        #[from]
        #[source]
        details: anyhow::Error,
    },
}

impl SEntropyError {
    /// Create an S-entropy calculation error
    pub fn s_entropy_calculation(message: impl Into<String>) -> Self {
        Self::SEntropyCalculation { message: message.into() }
    }

    /// Create a tri-dimensional alignment error
    pub fn tri_dimensional_alignment(
        dimension: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::TriDimensionalAlignment { dimension: dimension.into(), reason: reason.into() }
    }

    /// Create a navigation error
    pub fn navigation(operation: impl Into<String>, details: impl Into<String>) -> Self {
        Self::Navigation { operation: operation.into(), details: details.into() }
    }

    /// Create a consciousness integration error
    pub fn consciousness_integration(
        mode: impl Into<String>,
        violation: impl Into<String>,
    ) -> Self {
        Self::ConsciousnessIntegration { mode: mode.into(), violation: violation.into() }
    }

    /// Create a BMD operation error
    pub fn bmd_operation(operation_mode: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::BMDOperation { operation_mode: operation_mode.into(), reason: reason.into() }
    }

    /// Create a cross-domain transfer error
    pub fn cross_domain_transfer(
        source: impl Into<String>,
        target: impl Into<String>,
        efficiency_issue: impl Into<String>,
    ) -> Self {
        Self::CrossDomainTransfer {
            source: source.into(),
            target: target.into(),
            efficiency_issue: efficiency_issue.into(),
        }
    }

    /// Create a strategic impossibility error
    pub fn strategic_impossibility(
        impossibility_type: impl Into<String>,
        global_viability_issue: impl Into<String>,
    ) -> Self {
        Self::StrategicImpossibility {
            impossibility_type: impossibility_type.into(),
            global_viability_issue: global_viability_issue.into(),
        }
    }

    /// Create a temporal precision error
    pub fn temporal_precision(target: f64, achieved: f64) -> Self {
        Self::TemporalPrecision { target_precision: target, achieved_precision: achieved }
    }

    /// Create a memory optimization error
    pub fn memory_optimization(
        optimization_type: impl Into<String>,
        memory_issue: impl Into<String>,
    ) -> Self {
        Self::MemoryOptimization {
            optimization_type: optimization_type.into(),
            memory_issue: memory_issue.into(),
        }
    }

    /// Create a universal transformation error
    pub fn universal_transformation(
        problem_type: impl Into<String>,
        stsl_error: impl Into<String>,
    ) -> Self {
        Self::UniversalTransformation {
            problem_type: problem_type.into(),
            stsl_error: stsl_error.into(),
        }
    }

    /// Create a memorial significance validation error
    pub fn memorial_significance(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        Self::MemorialSignificance { expected: expected.into(), actual: actual.into() }
    }

    /// Create a framework boundary violation error
    pub fn boundary_violation(
        boundary_type: impl Into<String>,
        violation_details: impl Into<String>,
    ) -> Self {
        Self::BoundaryViolation {
            boundary_type: boundary_type.into(),
            violation_details: violation_details.into(),
        }
    }

    /// Create an observer-process integration error
    pub fn observer_process_integration(separation_distance: f64) -> Self {
        Self::ObserverProcessIntegration { separation_distance }
    }

    /// Create a zero computation error
    pub fn zero_computation(computation_type: impl Into<String>) -> Self {
        Self::ZeroComputation { computation_type: computation_type.into() }
    }

    /// Check if this error is related to memorial significance
    pub fn is_memorial_significance_error(&self) -> bool {
        matches!(self, Self::MemorialSignificance { .. })
    }

    /// Check if this error is related to consciousness boundaries
    pub fn is_consciousness_boundary_error(&self) -> bool {
        matches!(self, Self::ConsciousnessIntegration { .. } | Self::BoundaryViolation { .. })
    }

    /// Check if this error is related to S-entropy calculation
    pub fn is_s_entropy_calculation_error(&self) -> bool {
        matches!(self, Self::SEntropyCalculation { .. } | Self::TriDimensionalAlignment { .. })
    }

    /// Get error severity level
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            Self::MemorialSignificance { .. } => ErrorSeverity::Critical,
            Self::BoundaryViolation { .. } => ErrorSeverity::Critical,
            Self::ConsciousnessIntegration { .. } => ErrorSeverity::High,
            Self::SEntropyCalculation { .. } => ErrorSeverity::High,
            Self::TriDimensionalAlignment { .. } => ErrorSeverity::High,
            Self::ZeroComputation { .. } => ErrorSeverity::Medium,
            Self::Navigation { .. } => ErrorSeverity::Medium,
            Self::TemporalPrecision { .. } => ErrorSeverity::Medium,
            _ => ErrorSeverity::Low,
        }
    }
}

/// Error severity levels for S-Entropy operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// Low severity - operation can continue with degraded functionality
    Low,
    /// Medium severity - operation should be retried or alternative approach used
    Medium,
    /// High severity - operation must be stopped and corrected
    High,
    /// Critical severity - framework integrity is compromised
    Critical,
}

impl ErrorSeverity {
    /// Check if this severity level requires immediate attention
    pub fn requires_immediate_attention(&self) -> bool {
        matches!(self, Self::High | Self::Critical)
    }

    /// Check if this severity level allows operation to continue
    pub fn allows_continuation(&self) -> bool {
        matches!(self, Self::Low | Self::Medium)
    }
}

/// Helper macro for creating S-entropy specific errors with context
#[macro_export]
macro_rules! s_entropy_error {
    ($error_type:ident, $($arg:expr),*) => {
        SEntropyError::$error_type($($arg.into()),*)
    };
}

/// Helper macro for returning early with S-entropy error
#[macro_export]
macro_rules! s_entropy_bail {
    ($error:expr) => {
        return Err($error);
    };
}

/// Helper function to validate memorial significance in operations
pub fn validate_memorial_significance(actual: &str) -> SEntropyResult<()> {
    if actual != crate::MEMORIAL_SIGNIFICANCE {
        return Err(SEntropyError::memorial_significance(crate::MEMORIAL_SIGNIFICANCE, actual));
    }
    Ok(())
}

/// Helper function to check framework boundaries
pub fn check_framework_boundary(
    operation: &str,
    allowed_operations: &[&str],
) -> SEntropyResult<()> {
    if !allowed_operations.contains(&operation) {
        return Err(SEntropyError::boundary_violation(
            "operation_boundary",
            format!("Operation '{}' not in allowed list: {:?}", operation, allowed_operations),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = SEntropyError::s_entropy_calculation("Test calculation failed");
        assert!(matches!(error, SEntropyError::SEntropyCalculation { .. }));
        assert!(error.is_s_entropy_calculation_error());
    }

    #[test]
    fn test_error_severity() {
        let memorial_error = SEntropyError::memorial_significance("expected", "actual");
        assert_eq!(memorial_error.severity(), ErrorSeverity::Critical);
        assert!(memorial_error.severity().requires_immediate_attention());
    }

    #[test]
    fn test_memorial_significance_validation() {
        assert!(validate_memorial_significance(crate::MEMORIAL_SIGNIFICANCE).is_ok());
        assert!(validate_memorial_significance("invalid").is_err());
    }

    #[test]
    fn test_boundary_checking() {
        let allowed = &["operation1", "operation2"];
        assert!(check_framework_boundary("operation1", allowed).is_ok());
        assert!(check_framework_boundary("forbidden", allowed).is_err());
    }
}
