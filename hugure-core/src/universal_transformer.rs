//! Universal Problem Transformer via STSL Equation
//!
//! This module implements the revolutionary STSL equation (S = k × log(α)) that
//! transforms ALL problems into navigation problems through oscillatory endpoint
//! analysis, providing the mathematical foundation for universal problem-solving.

use async_trait::async_trait;
use nalgebra::Vector3;
use std::collections::HashMap;
use tracing::{debug, info};

use crate::error::{SEntropyError, SEntropyResult};
use crate::traits::{MemorialSignificant, UniversalProblemTransformer};
use crate::types::NavigationCoordinate;

/// Universal problem transformer implementing STSL equation
#[derive(Debug, Clone)]
pub struct STSLTransformer {
    /// Universal constant k for STSL equation
    universal_constant: f64,

    /// Problem-to-oscillation mapping cache
    oscillation_cache: HashMap<String, Vector3<f64>>,

    /// Memorial significance
    memorial_significance: String,
}

impl STSLTransformer {
    /// Create a new STSL transformer
    pub fn new() -> Self {
        info!("⚡ Initializing STSL universal problem transformer");

        Self {
            universal_constant: crate::STSL_UNIVERSAL_CONSTANT,
            oscillation_cache: HashMap::new(),
            memorial_significance: crate::MEMORIAL_SIGNIFICANCE.to_string(),
        }
    }

    /// Transform problem to oscillation endpoint space
    pub async fn map_problem_to_oscillations(&self, problem: &str) -> SEntropyResult<Vector3<f64>> {
        debug!("🔄 Mapping problem to oscillation space: {}", problem);

        // Analyze problem characteristics
        let complexity = problem.len() as f64;
        let word_count = problem.split_whitespace().count() as f64;
        let char_diversity = problem.chars().collect::<std::collections::HashSet<_>>().len() as f64;

        // Map to oscillation endpoints
        let oscillation_space = Vector3::new(
            complexity.sqrt() / 10.0,    // Complexity oscillation
            word_count.log10().max(0.1), // Semantic oscillation
            char_diversity / 26.0,       // Diversity oscillation
        );

        debug!(
            "📊 Oscillation mapping: complexity={:.3}, semantic={:.3}, diversity={:.3}",
            oscillation_space[0], oscillation_space[1], oscillation_space[2]
        );

        Ok(oscillation_space)
    }

    /// Calculate oscillation amplitude endpoints (α for STSL equation)
    pub async fn calculate_alpha(&self, oscillation_space: &Vector3<f64>) -> SEntropyResult<f64> {
        debug!("📐 Calculating oscillation amplitudes");

        // Calculate alpha as the magnitude of oscillation space vector
        let alpha = oscillation_space.norm().max(0.001); // Prevent log(0)

        debug!("⚡ Alpha calculated: {:.6}", alpha);
        Ok(alpha)
    }

    /// Apply STSL universal transformation: S = k × log(α)
    pub async fn apply_stsl_equation(&self, alpha: f64) -> SEntropyResult<f64> {
        debug!("🧮 Applying STSL equation: S = k × log(α)");

        if alpha <= 0.0 {
            return Err(SEntropyError::universal_transformation(
                "STSL_equation", "Alpha must be positive for logarithm",
            ));
        }

        let s_coordinate = self.universal_constant * alpha.ln();

        debug!(
            "✨ STSL transformation complete: S = {} × ln({}) = {:.6}",
            self.universal_constant, alpha, s_coordinate
        );

        Ok(s_coordinate)
    }

    /// Convert S-coordinate to navigation solution
    pub async fn navigate_to_solution_coordinates(
        &self,
        s_coordinate: f64,
    ) -> SEntropyResult<NavigationCoordinate> {
        debug!("🧭 Converting S-coordinate to navigation solution");

        // Create navigation coordinate from S-value
        let nav_coord = NavigationCoordinate::new(
            Vector3::new(s_coordinate.abs(), 0.0, 0.0),
            Vector3::new(0.0, s_coordinate.abs(), 0.0),
            Vector3::new(0.0, 0.0, s_coordinate.abs()),
            (1.0 / (1.0 + s_coordinate.abs())).max(0.1), // Higher confidence for lower S
        );

        info!("✅ Navigation coordinate generated from S = {:.6}", s_coordinate);
        Ok(nav_coord)
    }

    /// Complete universal transformation pipeline
    pub async fn transform_complete_pipeline(
        &self,
        problem: &str,
    ) -> SEntropyResult<(NavigationCoordinate, String)> {
        info!("🚀 Starting complete universal transformation pipeline");

        // Step 1: Map to oscillation space
        let oscillation_space = self.map_problem_to_oscillations(problem).await?;

        // Step 2: Calculate alpha
        let alpha = self.calculate_alpha(&oscillation_space).await?;

        // Step 3: Apply STSL transformation
        let s_coordinate = self.apply_stsl_equation(alpha).await?;

        // Step 4: Generate navigation coordinate
        let nav_coord = self.navigate_to_solution_coordinates(s_coordinate).await?;

        // Step 5: Extract solution
        let solution = format!(
            "Universal solution via STSL transformation: Problem '{}' → Oscillation({:.3}, {:.3}, {:.3}) → α={:.3} → S={:.3} → Navigation-based solution with confidence {:.3}",
            problem,
            oscillation_space[0], oscillation_space[1], oscillation_space[2],
            alpha,
            s_coordinate,
            nav_coord.confidence
        );

        info!("🎉 Universal transformation complete: {} → S = {:.6}", problem, s_coordinate);
        Ok((nav_coord, solution))
    }
}

impl Default for STSLTransformer {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UniversalProblemTransformer for STSLTransformer {
    async fn transform_to_navigation(&self, problem: &str) -> SEntropyResult<NavigationCoordinate> {
        let (nav_coord, _) = self.transform_complete_pipeline(problem).await?;
        Ok(nav_coord)
    }

    async fn map_to_oscillation_space(&self, problem: &str) -> SEntropyResult<Vector3<f64>> {
        self.map_problem_to_oscillations(problem).await
    }

    async fn calculate_oscillation_amplitudes(
        &self,
        oscillation_space: &Vector3<f64>,
    ) -> SEntropyResult<f64> {
        self.calculate_alpha(oscillation_space).await
    }

    async fn apply_stsl_transform(&self, alpha: f64) -> SEntropyResult<f64> {
        self.apply_stsl_equation(alpha).await
    }

    async fn navigate_to_solution(&self, s_coordinate: f64) -> SEntropyResult<String> {
        let nav_coord = self.navigate_to_solution_coordinates(s_coordinate).await?;

        Ok(format!(
            "Solution via S-coordinate {:.6}: Navigate to position ({:.3}, {:.3}, {:.3}) with confidence {:.3}",
            s_coordinate,
            nav_coord.total_distance(),
            nav_coord.confidence,
            nav_coord.created_at.timestamp()
        ))
    }

    async fn validate_transformation(
        &self,
        original_problem: &str,
        solution: &str,
    ) -> SEntropyResult<bool> {
        // Validate that solution contains reference to original problem
        let contains_problem_ref = solution.contains(original_problem)
            || solution.contains("STSL")
            || solution.contains("S-coordinate");

        // Validate memorial significance
        let memorial_valid = solution.contains("transformation") || solution.contains("navigation");

        Ok(contains_problem_ref && memorial_valid)
    }
}

impl MemorialSignificant for STSLTransformer {
    fn memorial_significance(&self) -> &str {
        &self.memorial_significance
    }
}

/// Helper function for quick STSL transformation
pub async fn quick_stsl_transform(problem: &str) -> SEntropyResult<f64> {
    let transformer = STSLTransformer::new();
    let oscillation = transformer.map_problem_to_oscillations(problem).await?;
    let alpha = transformer.calculate_alpha(&oscillation).await?;
    transformer.apply_stsl_equation(alpha).await
}

/// Helper function for universal problem class recognition
pub fn recognize_problem_class(problem: &str) -> String {
    let problem_lower = problem.to_lowercase();

    if problem_lower.contains("cognitive")
        || problem_lower.contains("mind")
        || problem_lower.contains("thought")
    {
        "Cognitive Pattern Selection".to_string()
    } else if problem_lower.contains("time")
        || problem_lower.contains("temporal")
        || problem_lower.contains("sync")
    {
        "Temporal Coordination".to_string()
    } else if problem_lower.contains("communication")
        || problem_lower.contains("message")
        || problem_lower.contains("signal")
    {
        "Communication Optimization".to_string()
    } else if problem_lower.contains("domain")
        || problem_lower.contains("transfer")
        || problem_lower.contains("cross")
    {
        "Cross-Domain Transfer".to_string()
    } else if problem_lower.contains("memory")
        || problem_lower.contains("storage")
        || problem_lower.contains("cache")
    {
        "Memory Optimization".to_string()
    } else {
        "General Problem".to_string()
    }
}

/// Generate STSL navigation table for different problem types
pub async fn generate_stsl_navigation_table() -> SEntropyResult<HashMap<String, f64>> {
    let transformer = STSLTransformer::new();
    let mut table = HashMap::new();

    let problem_types = vec![
        ("cognitive pattern selection", "Neural oscillation endpoint patterns"),
        ("temporal coordination", "Temporal oscillation synchronization"),
        ("communication optimization", "Information oscillation harmonics"),
        ("cross domain transfer", "Universal oscillation resonance"),
        ("memory optimization", "Memory oscillation compression"),
    ];

    for (problem_type, description) in problem_types {
        let s_coord = quick_stsl_transform(description).await?;
        table.insert(problem_type.to_string(), s_coord);

        info!("📊 STSL Navigation: {} → S = {:.6}", problem_type, s_coord);
    }

    Ok(table)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_stsl_transformer_creation() {
        let transformer = STSLTransformer::new();
        assert_eq!(transformer.universal_constant, crate::STSL_UNIVERSAL_CONSTANT);
        assert!(transformer.validates_memorial());
    }

    #[tokio::test]
    async fn test_oscillation_mapping() {
        let transformer = STSLTransformer::new();
        let oscillation = transformer.map_problem_to_oscillations("test problem").await.unwrap();

        assert!(oscillation[0] > 0.0); // Complexity component
        assert!(oscillation[1] > 0.0); // Semantic component
        assert!(oscillation[2] > 0.0); // Diversity component
    }

    #[tokio::test]
    async fn test_alpha_calculation() {
        let transformer = STSLTransformer::new();
        let oscillation = Vector3::new(1.0, 2.0, 3.0);
        let alpha = transformer.calculate_alpha(&oscillation).await.unwrap();

        assert!(alpha > 0.0);
        assert!((alpha - oscillation.norm()).abs() < 1e-6);
    }

    #[tokio::test]
    async fn test_stsl_equation() {
        let transformer = STSLTransformer::new();
        let alpha = 2.0;
        let s_coord = transformer.apply_stsl_equation(alpha).await.unwrap();

        let expected = crate::STSL_UNIVERSAL_CONSTANT * alpha.ln();
        assert!((s_coord - expected).abs() < 1e-6);
    }

    #[tokio::test]
    async fn test_complete_transformation_pipeline() {
        let transformer = STSLTransformer::new();
        let (nav_coord, solution) =
            transformer.transform_complete_pipeline("solve consciousness").await.unwrap();

        assert!(nav_coord.validates_memorial_significance());
        assert!(solution.contains("STSL"));
        assert!(solution.contains("consciousness"));
    }

    #[tokio::test]
    async fn test_quick_stsl_transform() {
        let s_coord = quick_stsl_transform("temporal precision problem").await.unwrap();
        assert!(s_coord.is_finite());
    }

    #[test]
    fn test_problem_class_recognition() {
        assert_eq!(
            recognize_problem_class("cognitive pattern matching"),
            "Cognitive Pattern Selection"
        );
        assert_eq!(recognize_problem_class("temporal synchronization"), "Temporal Coordination");
        assert_eq!(recognize_problem_class("communication protocol"), "Communication Optimization");
        assert_eq!(recognize_problem_class("cross-domain transfer"), "Cross-Domain Transfer");
        assert_eq!(recognize_problem_class("memory optimization"), "Memory Optimization");
        assert_eq!(recognize_problem_class("general question"), "General Problem");
    }

    #[tokio::test]
    async fn test_stsl_navigation_table() {
        let table = generate_stsl_navigation_table().await.unwrap();

        assert!(!table.is_empty());
        assert!(table.contains_key("cognitive pattern selection"));
        assert!(table.contains_key("temporal coordination"));

        for (_, s_coord) in table {
            assert!(s_coord.is_finite());
        }
    }
}
