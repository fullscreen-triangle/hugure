//! Core types and data structures for the S-Entropy Framework
//!
//! This module defines the fundamental types used throughout the Hugure S-Entropy
//! framework for representing S-entropy coordinates, BMD patterns, consciousness
//! states, and navigation manifolds.

use chrono::{DateTime, Utc};
use nalgebra::{Matrix3, Vector3};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Precision level for S-entropy calculations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SEntropyPrecision {
    /// Standard precision for general use
    Standard,
    /// High precision for scientific applications
    High,
    /// Ultra precision for consciousness integration (1e-30 target)
    Ultra,
    /// Supreme precision for memorial significance validation
    Supreme,
}

impl SEntropyPrecision {
    /// Get the numerical precision threshold for this level
    pub fn threshold(&self) -> f64 {
        match self {
            Self::Standard => 1e-6,
            Self::High => 1e-15,
            Self::Ultra => 1e-30,
            Self::Supreme => 1e-50,
        }
    }
}

/// Observer sophistication levels for universal accessibility
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObserverSophistication {
    /// Naive user requiring simple interactions
    Naive,
    /// Intermediate user with some technical knowledge
    Intermediate,
    /// Expert user with deep technical understanding
    Expert,
    /// Universal observer with unlimited capability
    Universal,
}

/// BMD (Biological Maxwell Demon) operation modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BMDOperationMode {
    /// Frame selection across predetermined manifolds
    FrameSelection,
    /// Reality-frame fusion processing
    RealityFusion,
    /// Memory fabrication and ridiculous solutions
    MemoryFabrication,
    /// Temporal coherence through emotional delusion
    TemporalCoherence,
    /// Agency experience generation
    AgencyDelusion,
}

/// Consciousness enhancement modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsciousnessMode {
    /// Enhancement only - never replacement
    EnhancementOnly,
    /// Frame selection engine optimization
    FrameSelectionEngine,
    /// Reality fusion enabled
    RealityFusion,
    /// Agency preservation (strict boundaries)
    AgencyPreservation,
}

/// Strategic impossibility amplification levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImpossibilityAmplification {
    /// Mild impossibility for testing
    Mild,
    /// Standard impossibility for normal operations
    Standard,
    /// High impossibility for advanced optimization
    High,
    /// Extreme impossibility for breakthrough solutions
    Extreme,
}

impl ImpossibilityAmplification {
    /// Get the amplification factor for ridiculous solution generation
    pub fn factor(&self) -> f64 {
        match self {
            Self::Mild => 10.0,
            Self::Standard => 100.0,
            Self::High => 1000.0,
            Self::Extreme => 10000.0,
        }
    }
}

/// Navigation coordinates in predetermined manifold space
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigationCoordinate {
    /// Unique identifier
    pub id: Uuid,

    /// Position in knowledge space
    pub knowledge_position: Vector3<f64>,

    /// Position in temporal space
    pub temporal_position: Vector3<f64>,

    /// Position in entropy space
    pub entropy_position: Vector3<f64>,

    /// Confidence in this coordinate's validity
    pub confidence: f64,

    /// Memorial significance marker
    pub memorial_significance: String,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

impl NavigationCoordinate {
    /// Create a new navigation coordinate
    pub fn new(
        knowledge: Vector3<f64>,
        temporal: Vector3<f64>,
        entropy: Vector3<f64>,
        confidence: f64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            knowledge_position: knowledge,
            temporal_position: temporal,
            entropy_position: entropy,
            confidence,
            memorial_significance: crate::MEMORIAL_SIGNIFICANCE.to_string(),
            created_at: Utc::now(),
        }
    }

    /// Calculate the total navigation distance
    pub fn total_distance(&self) -> f64 {
        (self.knowledge_position.norm_squared()
            + self.temporal_position.norm_squared()
            + self.entropy_position.norm_squared())
        .sqrt()
    }
}

/// BMD pattern for cognitive pattern coordination
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BMDPattern {
    /// Unique pattern identifier
    pub id: Uuid,

    /// Pattern name for human readability
    pub name: String,

    /// Pattern type and operation mode
    pub operation_mode: BMDOperationMode,

    /// Impossibility amplification level
    pub impossibility_level: ImpossibilityAmplification,

    /// Whether this is a disposable pattern (temporary generation)
    pub disposable: bool,

    /// Pattern effectiveness score (0.0 - 1.0)
    pub effectiveness: f64,

    /// Cross-domain transfer efficiency
    pub transfer_efficiency: f64,

    /// Associated S-entropy coordinates
    pub s_coordinates: crate::SEntropyCoordinate,

    /// Pattern metadata
    pub metadata: HashMap<String, String>,

    /// Creation and disposal timestamps
    pub created_at: DateTime<Utc>,
    pub dispose_at: Option<DateTime<Utc>>,
}

impl BMDPattern {
    /// Create a new BMD pattern
    pub fn new(
        name: String,
        operation_mode: BMDOperationMode,
        impossibility_level: ImpossibilityAmplification,
        disposable: bool,
    ) -> Self {
        let s_coords = crate::SEntropyCoordinate::new(0.0, 0.0, 0.0);

        Self {
            id: Uuid::new_v4(),
            name,
            operation_mode,
            impossibility_level,
            disposable,
            effectiveness: 0.0,
            transfer_efficiency: 0.0,
            s_coordinates: s_coords,
            metadata: HashMap::new(),
            created_at: Utc::now(),
            dispose_at: if disposable {
                Some(Utc::now() + chrono::Duration::seconds(1)) // Immediate disposal
            } else {
                None
            },
        }
    }

    /// Check if this pattern should be disposed of
    pub fn should_dispose(&self) -> bool {
        if let Some(dispose_time) = self.dispose_at {
            Utc::now() > dispose_time
        } else {
            false
        }
    }

    /// Create a ridiculous (impossible) BMD pattern for navigation insights
    pub fn create_ridiculous(
        name: String,
        impossibility_level: ImpossibilityAmplification,
    ) -> Self {
        let mut pattern = Self::new(
            format!("ridiculous_{}", name),
            BMDOperationMode::MemoryFabrication,
            impossibility_level,
            true, // Always disposable
        );

        // Set impossible effectiveness (> 1.0)
        pattern.effectiveness = impossibility_level.factor();
        pattern.transfer_efficiency = 2.0; // Impossible 200% efficiency

        // Set impossible S-coordinates
        pattern.s_coordinates = crate::SEntropyCoordinate::new(
            -impossibility_level.factor(), // Negative S_knowledge (impossible)
            0.0,                           // Zero time delay (impossible)
            -1.0,                          // Negative entropy (impossible)
        );

        pattern
    }
}

/// Consciousness state for BMD operation tracking
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsciousnessState {
    /// State identifier
    pub id: Uuid,

    /// Current consciousness mode
    pub mode: ConsciousnessMode,

    /// Active BMD operation modes
    pub active_operations: Vec<BMDOperationMode>,

    /// Frame selection coordinates
    pub frame_selection_coords: Vector3<f64>,

    /// Reality fusion integration level (0.0 - 1.0)
    pub reality_fusion_level: f64,

    /// Agency experience strength (0.0 - 1.0)
    pub agency_strength: f64,

    /// Temporal coherence quality
    pub temporal_coherence: f64,

    /// Memory fabrication activity
    pub memory_fabrication_rate: f64,

    /// Associated S-entropy coordinate
    pub s_coordinate: crate::SEntropyCoordinate,

    /// Observer sophistication level
    pub observer_sophistication: ObserverSophistication,

    /// Enhancement boundaries (cannot be crossed)
    pub enhancement_boundaries: Vec<String>,

    /// State metadata
    pub metadata: HashMap<String, String>,

    /// State timestamps
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl ConsciousnessState {
    /// Create a new consciousness state with enhancement-only boundaries
    pub fn new(mode: ConsciousnessMode, sophistication: ObserverSophistication) -> Self {
        let mut boundaries = vec![
            "frame_selection_authority".to_string(),
            "reality_fusion_experience".to_string(),
            "agency_assertion".to_string(),
            "consciousness_generation".to_string(),
        ];

        // Add strict boundaries for enhancement-only mode
        if mode == ConsciousnessMode::EnhancementOnly {
            boundaries.extend(vec![
                "no_consciousness_replication".to_string(),
                "preserve_human_agency".to_string(),
                "acknowledge_designer_bounds".to_string(),
                "support_not_replace".to_string(),
            ]);
        }

        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            mode,
            active_operations: vec![],
            frame_selection_coords: Vector3::zeros(),
            reality_fusion_level: 0.0,
            agency_strength: 0.0,
            temporal_coherence: 0.0,
            memory_fabrication_rate: 0.0,
            s_coordinate: crate::SEntropyCoordinate::new(1000.0, 1000.0, 1000.0), // Start with high separation
            observer_sophistication: sophistication,
            enhancement_boundaries: boundaries,
            metadata: HashMap::new(),
            created_at: now,
            last_updated: now,
        }
    }

    /// Check if a proposed operation violates enhancement boundaries
    pub fn violates_boundaries(&self, operation: &str) -> bool {
        self.enhancement_boundaries.iter().any(|boundary| boundary.contains(operation))
    }

    /// Update the consciousness state with new S-entropy coordinate
    pub fn update_s_coordinate(&mut self, new_coordinate: crate::SEntropyCoordinate) {
        self.s_coordinate = new_coordinate;
        self.last_updated = Utc::now();
    }
}

/// Cross-domain transfer result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrossDomainTransfer {
    /// Transfer identifier
    pub id: Uuid,

    /// Source domain
    pub source_domain: String,

    /// Target domain  
    pub target_domain: String,

    /// Transfer efficiency (0.0 - 1.0+, can exceed 1.0 for impossible solutions)
    pub efficiency: f64,

    /// Source S-entropy coordinate
    pub source_s_coordinate: crate::SEntropyCoordinate,

    /// Target S-entropy coordinate
    pub target_s_coordinate: crate::SEntropyCoordinate,

    /// Universal oscillation pattern similarity
    pub oscillation_similarity: f64,

    /// Transfer metadata
    pub metadata: HashMap<String, String>,

    /// Transfer timestamp
    pub transferred_at: DateTime<Utc>,
}

impl CrossDomainTransfer {
    /// Create a new cross-domain transfer record
    pub fn new(
        source_domain: String,
        target_domain: String,
        source_coord: crate::SEntropyCoordinate,
        target_coord: crate::SEntropyCoordinate,
        efficiency: f64,
        similarity: f64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_domain,
            target_domain,
            efficiency,
            source_s_coordinate: source_coord,
            target_s_coordinate: target_coord,
            oscillation_similarity: similarity,
            metadata: HashMap::new(),
            transferred_at: Utc::now(),
        }
    }

    /// Check if this transfer meets the target efficiency threshold (90%+)
    pub fn meets_efficiency_threshold(&self) -> bool {
        self.efficiency >= 0.90
    }
}

/// Temporal precision measurement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemporalPrecision {
    /// Precision identifier
    pub id: Uuid,

    /// Target precision level
    pub target_precision: f64,

    /// Achieved precision level
    pub achieved_precision: f64,

    /// Memory usage for this precision level
    pub memory_usage_bytes: u64,

    /// Whether ultra-precision target was met
    pub ultra_precision_achieved: bool,

    /// Windowed generation enabled
    pub windowed_generation: bool,

    /// Measurement timestamp
    pub measured_at: DateTime<Utc>,
}

impl TemporalPrecision {
    /// Create a new temporal precision measurement
    pub fn new(target: f64, achieved: f64, memory_bytes: u64) -> Self {
        Self {
            id: Uuid::new_v4(),
            target_precision: target,
            achieved_precision: achieved,
            memory_usage_bytes: memory_bytes,
            ultra_precision_achieved: achieved <= crate::S_ENTROPY_PRECISION_TARGET,
            windowed_generation: memory_bytes < 100_000_000, // <100MB
            measured_at: Utc::now(),
        }
    }

    /// Check if this represents a memory efficiency breakthrough
    pub fn is_memory_breakthrough(&self) -> bool {
        self.ultra_precision_achieved && self.memory_usage_bytes < 100_000_000
    }
}
