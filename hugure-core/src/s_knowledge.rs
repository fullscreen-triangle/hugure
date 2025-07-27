//! S-Knowledge Analysis and Frame Selection Coordinates
//!
//! This module implements S_knowledge measurement: information deficit analysis
//! and frame selection coordinates for consciousness-aware BMD operations.

use crate::error::SEntropyResult;
use crate::types::ObserverSophistication;

/// Analyze information deficit for S_knowledge calculation
pub async fn analyze_information_deficit(
    context: &str,
    observer: ObserverSophistication,
) -> SEntropyResult<f64> {
    let base_deficit = match observer {
        ObserverSophistication::Naive => 1000.0,
        ObserverSophistication::Intermediate => 100.0,
        ObserverSophistication::Expert => 10.0,
        ObserverSophistication::Universal => 0.0,
    };

    let context_factor = (context.len() as f64).log10().max(1.0);
    Ok(base_deficit / context_factor)
}

/// Calculate frame selection coordinates
pub async fn calculate_frame_selection_coordinates(problem_context: &str) -> SEntropyResult<f64> {
    let complexity = problem_context.len() as f64;
    let word_count = problem_context.split_whitespace().count() as f64;

    let coordinates = (complexity.sqrt() + word_count.log10()) / 10.0;
    Ok(coordinates.max(0.01))
}
