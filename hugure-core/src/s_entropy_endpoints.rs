//! S-Entropy Endpoint Navigation and Oscillation Accessibility
//!
//! This module implements S_entropy endpoint navigation and oscillation
//! accessibility for predetermined manifold coordination.

use crate::error::SEntropyResult;

/// Calculate entropy endpoint navigation distance
pub async fn calculate_entropy_navigation_distance(
    problem_complexity: f64,
    accessibility: f64,
) -> SEntropyResult<f64> {
    let distance = if accessibility > 0.9 {
        0.01 // Near-zero for high accessibility
    } else {
        (1.0 - accessibility) * problem_complexity
    };

    Ok(distance.max(0.0))
}

/// Calculate oscillation accessibility factor
pub async fn calculate_oscillation_accessibility(accessibility: f64) -> SEntropyResult<f64> {
    let factor = if accessibility > 0.5 {
        accessibility.log10().abs()
    } else {
        (1.0 - accessibility) * 100.0
    };

    Ok(factor.max(0.0))
}
