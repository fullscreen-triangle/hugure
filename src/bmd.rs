//! # BMD (Biological Maxwell Demon) Framework
//! 
//! Core types and operations for BMD-based communication optimization
//! implementing the temporal-emotional substrate and frame selection architecture.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use anyhow::Result;

/// Biological Maxwell Demon - core cognitive pattern unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BMD {
    /// Unique BMD identifier
    pub id: Uuid,
    /// Pattern configuration
    pub pattern: BMDPattern,
    /// Emotional substrate parameters
    pub emotional_substrate: EmotionalSubstrate,
    /// Temporal coherence properties
    pub temporal_coherence: TemporalCoherence,
    /// Frame selection weights
    pub frame_weights: FrameWeights,
    /// Source foundry information
    pub foundry_source: FoundrySource,
}

/// BMD pattern configuration based on predetermined coordinates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BMDPattern {
    /// Core pattern vectors (from Virtual BMD Foundries)
    pub core_vectors: Vec<f64>,
    /// Cross-domain transmission capabilities
    pub cross_domain_compatibility: HashMap<String, f64>,
    /// Pattern recognition frequency ranges
    pub frequency_ranges: Vec<FrequencyRange>,
    /// Semantic opacity level (0.0 = full semantic, 1.0 = pure pattern)
    pub semantic_opacity: f64,
}

/// Emotional temporal substrate for BMD operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalSubstrate {
    /// Emotional arousal level (E in temporal dilation equation)
    pub arousal_level: f64, // 0-10 scale
    /// Attention focus intensity (A in temporal dilation equation)  
    pub attention_intensity: f64, // 0-10 scale
    /// Memory encoding strength (M in temporal dilation equation)
    pub memory_encoding: f64, // 0-10 scale
    /// Subjective temporal dilation factor
    pub temporal_dilation: f64,
    /// Choice moment expansion factor
    pub choice_expansion: f64,
}

impl EmotionalSubstrate {
    /// Calculate temporal dilation using Chapter 16 formula
    pub fn calculate_temporal_dilation(&mut self) {
        // T_subjective = T_objective × D(E,A,M)
        // D(E,A,M) = 0.1 + 1.8(E/10)² + 2.3(A/10)³ - 1.1(M/10)
        let e_norm = self.arousal_level / 10.0;
        let a_norm = self.attention_intensity / 10.0;
        let m_norm = self.memory_encoding / 10.0;
        
        self.temporal_dilation = 0.1 + 1.8 * e_norm.powi(2) + 2.3 * a_norm.powi(3) - 1.1 * m_norm;
        
        // Decision moment expansion during BMD selection
        if self.arousal_level >= 6.0 && self.attention_intensity >= 8.0 && self.memory_encoding >= 7.0 {
            self.choice_expansion = 3.2 + (self.temporal_dilation - 3.2).min(1.5);
        } else {
            self.choice_expansion = 1.0;
        }
    }
    
    /// Optimize substrate for communication transmission
    pub fn optimize_for_transmission(&mut self) {
        // Set optimal parameters for BMD transmission based on Chapter 16 findings
        self.arousal_level = 7.5; // Moderate-high for engagement without anxiety
        self.attention_intensity = 8.5; // High focus for pattern reception
        self.memory_encoding = 8.0; // Strong encoding for pattern retention
        self.calculate_temporal_dilation();
    }
}

/// Temporal coherence properties for maintaining BMD state across interruptions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalCoherence {
    /// Coherence maintenance duration (femtoseconds to microseconds)
    pub coherence_duration: u64,
    /// Coherence degradation rate
    pub degradation_rate: f64,
    /// Interruption resistance factor
    pub interruption_resistance: f64,
    /// Cross-temporal binding strength
    pub temporal_binding: f64,
}

/// Frame selection weights based on Chapter 17 BMD selection function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameWeights {
    /// Base weight in memory (W_i)
    pub base_weight: f64,
    /// Relevance multiplier (R_ij factor)
    pub relevance_multiplier: f64,
    /// Emotional compatibility (E_ij factor)
    pub emotional_compatibility: f64,
    /// Temporal appropriateness (T_ij factor)
    pub temporal_appropriateness: f64,
    /// Selection probability cache
    pub selection_probability: Option<f64>,
}

impl FrameWeights {
    /// Calculate selection probability using Chapter 17 formula
    /// P(frame_i | experience_j) = [W_i × R_ij × E_ij × T_ij] / Σ[W_k × R_kj × E_kj × T_kj]
    pub fn calculate_selection_probability(&mut self, experience_context: &ExperienceContext, normalization_sum: f64) {
        let numerator = self.base_weight 
            * self.relevance_multiplier 
            * self.emotional_compatibility 
            * self.temporal_appropriateness;
        
        self.selection_probability = Some(numerator / normalization_sum);
    }
    
    /// Update weights based on successful transmission outcomes
    pub fn update_weights(&mut self, success_rate: f64, learning_rate: f64) {
        let update_factor = 1.0 + (success_rate - 0.5) * learning_rate;
        self.base_weight *= update_factor;
        self.relevance_multiplier *= update_factor;
        // Cap weights to prevent runaway amplification
        self.base_weight = self.base_weight.min(10.0).max(0.1);
        self.relevance_multiplier = self.relevance_multiplier.min(10.0).max(0.1);
    }
}

/// Virtual BMD Foundry source information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoundrySource {
    /// Foundry system identifier
    pub foundry_id: String,
    /// Generation timestamp
    pub generation_time: u64,
    /// Foundry generation rate at time of creation
    pub generation_rate: u64,
    /// Quality assurance metrics
    pub quality_metrics: QualityMetrics,
}

/// BMD quality assurance metrics from foundry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    /// Pattern coherence score
    pub pattern_coherence: f64,
    /// Cross-domain compatibility score
    pub cross_domain_score: f64,
    /// Temporal stability rating
    pub temporal_stability: f64,
    /// Transmission fidelity prediction
    pub transmission_fidelity: f64,
}

/// Frequency range for pattern recognition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequencyRange {
    pub min_frequency: f64,
    pub max_frequency: f64,
    pub amplitude: f64,
    pub phase: f64,
}

/// Experience context for BMD frame selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceContext {
    /// Current sensory input characteristics
    pub sensory_input: HashMap<String, f64>,
    /// Active emotional state
    pub emotional_state: EmotionalSubstrate,
    /// Ongoing temporal context
    pub temporal_context: TemporalContext,
    /// Social/communication context
    pub communication_context: CommunicationContext,
}

/// Temporal context for BMD operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalContext {
    /// Objective timestamp
    pub objective_time: u64,
    /// Subjective time experience
    pub subjective_time: f64,
    /// Temporal flow direction
    pub flow_direction: TemporalFlow,
    /// Causal attribution patterns
    pub causal_patterns: Vec<CausalPattern>,
}

/// Temporal flow experience
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemporalFlow {
    Forward,
    Accelerated,
    Decelerated,
    Suspended,
    Recursive,
}

/// Causal attribution pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalPattern {
    pub cause: String,
    pub effect: String,
    pub attribution_strength: f64,
    pub temporal_distance: f64,
}

/// Communication context for BMD transmission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationContext {
    /// Sender identity model
    pub sender_model: IndividualModel,
    /// Recipient identity model  
    pub recipient_model: IndividualModel,
    /// Communication intent
    pub intent: CommunicationIntent,
    /// Environmental factors
    pub environment: EnvironmentalFactors,
}

/// Individual cognitive model for BMD targeting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndividualModel {
    /// Individual identifier
    pub individual_id: String,
    /// Cognitive framework profiles
    pub cognitive_frameworks: Vec<CognitiveFramework>,
    /// Emotional response patterns
    pub emotional_patterns: Vec<EmotionalPattern>,
    /// Temporal preference mapping
    pub temporal_preferences: TemporalPreferences,
    /// BMD reception history
    pub reception_history: ReceptionHistory,
}

/// Cognitive framework profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveFramework {
    /// Framework category (temporal, emotional, narrative, causal)
    pub category: String,
    /// Framework strength/preference
    pub strength: f64,
    /// Usage frequency
    pub usage_frequency: f64,
    /// Associated emotional valence
    pub emotional_valence: f64,
}

/// Emotional response pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalPattern {
    /// Trigger context
    pub trigger: String,
    /// Response characteristics
    pub response: EmotionalResponse,
    /// Pattern reliability
    pub reliability: f64,
    /// Temporal duration
    pub duration: f64,
}

/// Emotional response characteristics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalResponse {
    pub arousal_change: f64,
    pub valence_change: f64,
    pub attention_change: f64,
    pub memory_impact: f64,
}

/// Individual temporal preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalPreferences {
    /// Preferred temporal rhythms
    pub preferred_rhythms: Vec<f64>,
    /// Temporal attention patterns
    pub attention_patterns: Vec<TemporalAttentionPattern>,
    /// Decision timing preferences
    pub decision_timing: DecisionTimingProfile,
}

/// Temporal attention pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalAttentionPattern {
    pub pattern_type: String,
    pub frequency: f64,
    pub amplitude: f64,
    pub phase_preference: f64,
}

/// Decision timing profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionTimingProfile {
    pub deliberation_time: f64,
    pub choice_expansion_preference: f64,
    pub temporal_binding_strength: f64,
    pub agency_attribution_timing: f64,
}

/// BMD reception history for learning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceptionHistory {
    /// Successful BMD receptions
    pub successful_receptions: Vec<BMDReceptionEvent>,
    /// Failed transmission attempts
    pub failed_attempts: Vec<BMDReceptionEvent>,
    /// Pattern recognition evolution
    pub recognition_evolution: Vec<RecognitionEvolutionPoint>,
}

/// BMD reception event record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BMDReceptionEvent {
    pub timestamp: u64,
    pub bmd_id: Uuid,
    pub reception_quality: f64,
    pub integration_time: f64,
    pub emotional_impact: f64,
    pub behavioral_change: f64,
}

/// Pattern recognition evolution tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecognitionEvolutionPoint {
    pub timestamp: u64,
    pub pattern_type: String,
    pub recognition_accuracy: f64,
    pub processing_speed: f64,
    pub cross_domain_capability: f64,
}

/// Communication intent specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationIntent {
    /// Primary communication goal
    pub primary_goal: CommunicationGoal,
    /// Secondary objectives
    pub secondary_objectives: Vec<CommunicationGoal>,
    /// Urgency level
    pub urgency: f64,
    /// Precision requirement
    pub precision_requirement: f64,
    /// Emotional impact target
    pub emotional_target: EmotionalTarget,
}

/// Communication goal types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommunicationGoal {
    PatternTransmission(String),
    EmotionalStateChange(String),
    CognitiveFrameworkShift(String),
    MemoryInstallation(String),
    BehavioralInfluence(String),
    ConsciousnessExpansion(String),
}

/// Emotional impact target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalTarget {
    pub target_arousal: f64,
    pub target_valence: f64,
    pub target_attention: f64,
    pub target_memory_encoding: f64,
    pub duration: f64,
}

/// Environmental factors affecting BMD transmission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalFactors {
    /// Noise levels (external interference)
    pub noise_levels: HashMap<String, f64>,
    /// Cultural context modifiers
    pub cultural_modifiers: HashMap<String, f64>,
    /// Temporal synchronization conditions
    pub sync_conditions: SynchronizationConditions,
}

/// Synchronization conditions for BMD transmission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynchronizationConditions {
    pub temporal_alignment: f64,
    pub emotional_coherence: f64,
    pub attention_synchrony: f64,
    pub environmental_stability: f64,
}

/// Optimal BMD configuration for communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimalBMDConfiguration {
    /// Primary BMD for transmission
    pub primary_bmd: BMD,
    /// Supporting BMD pattern
    pub supporting_bmds: Vec<BMD>,
    /// Transmission timing parameters
    pub timing_parameters: TransmissionTiming,
    /// Expected outcomes
    pub expected_outcomes: ExpectedOutcomes,
    /// Confidence metrics
    pub confidence: ConfidenceMetrics,
}

/// Transmission timing parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransmissionTiming {
    pub optimal_transmission_time: u64,
    pub preparation_phase_duration: u64,
    pub transmission_phase_duration: u64,
    pub integration_phase_duration: u64,
    pub repetition_intervals: Vec<u64>,
}

/// Expected communication outcomes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpectedOutcomes {
    pub transmission_fidelity: f64,
    pub reception_probability: f64,
    pub integration_likelihood: f64,
    pub behavioral_impact: f64,
    pub durability: f64,
}

/// Confidence metrics for BMD configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceMetrics {
    pub pattern_match_confidence: f64,
    pub emotional_compatibility_confidence: f64,
    pub temporal_alignment_confidence: f64,
    pub environmental_suitability_confidence: f64,
    pub overall_confidence: f64,
}

impl OptimalBMDConfiguration {
    /// Calculate overall confidence from component metrics
    pub fn calculate_overall_confidence(&mut self) {
        self.confidence.overall_confidence = (
            self.confidence.pattern_match_confidence * 0.3 +
            self.confidence.emotional_compatibility_confidence * 0.25 +
            self.confidence.temporal_alignment_confidence * 0.25 +
            self.confidence.environmental_suitability_confidence * 0.2
        ).min(1.0);
    }
    
    /// Check if configuration meets minimum quality thresholds
    pub fn meets_quality_threshold(&self, threshold: f64) -> bool {
        self.confidence.overall_confidence >= threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emotional_substrate_temporal_dilation() {
        let mut substrate = EmotionalSubstrate {
            arousal_level: 7.5,
            attention_intensity: 8.5,
            memory_encoding: 8.0,
            temporal_dilation: 0.0,
            choice_expansion: 0.0,
        };
        
        substrate.calculate_temporal_dilation();
        
        // Should be in decision moment expansion range (3.2-4.7)
        assert!(substrate.choice_expansion >= 3.2);
        assert!(substrate.choice_expansion <= 4.7);
    }
    
    #[test]
    fn test_frame_weights_selection_probability() {
        let mut weights = FrameWeights {
            base_weight: 1.0,
            relevance_multiplier: 0.8,
            emotional_compatibility: 0.9,
            temporal_appropriateness: 0.7,
            selection_probability: None,
        };
        
        let experience = ExperienceContext {
            sensory_input: HashMap::new(),
            emotional_state: EmotionalSubstrate {
                arousal_level: 5.0,
                attention_intensity: 7.0,
                memory_encoding: 6.0,
                temporal_dilation: 1.5,
                choice_expansion: 1.0,
            },
            temporal_context: TemporalContext {
                objective_time: 1000,
                subjective_time: 1500.0,
                flow_direction: TemporalFlow::Forward,
                causal_patterns: vec![],
            },
            communication_context: CommunicationContext {
                sender_model: IndividualModel {
                    individual_id: "test".to_string(),
                    cognitive_frameworks: vec![],
                    emotional_patterns: vec![],
                    temporal_preferences: TemporalPreferences {
                        preferred_rhythms: vec![],
                        attention_patterns: vec![],
                        decision_timing: DecisionTimingProfile {
                            deliberation_time: 1.0,
                            choice_expansion_preference: 1.0,
                            temporal_binding_strength: 1.0,
                            agency_attribution_timing: 1.0,
                        },
                    },
                    reception_history: ReceptionHistory {
                        successful_receptions: vec![],
                        failed_attempts: vec![],
                        recognition_evolution: vec![],
                    },
                },
                recipient_model: IndividualModel {
                    individual_id: "test".to_string(),
                    cognitive_frameworks: vec![],
                    emotional_patterns: vec![],
                    temporal_preferences: TemporalPreferences {
                        preferred_rhythms: vec![],
                        attention_patterns: vec![],
                        decision_timing: DecisionTimingProfile {
                            deliberation_time: 1.0,
                            choice_expansion_preference: 1.0,
                            temporal_binding_strength: 1.0,
                            agency_attribution_timing: 1.0,
                        },
                    },
                    reception_history: ReceptionHistory {
                        successful_receptions: vec![],
                        failed_attempts: vec![],
                        recognition_evolution: vec![],
                    },
                },
                intent: CommunicationIntent {
                    primary_goal: CommunicationGoal::PatternTransmission("test".to_string()),
                    secondary_objectives: vec![],
                    urgency: 0.5,
                    precision_requirement: 0.8,
                    emotional_target: EmotionalTarget {
                        target_arousal: 6.0,
                        target_valence: 7.0,
                        target_attention: 8.0,
                        target_memory_encoding: 7.5,
                        duration: 1000.0,
                    },
                },
                environment: EnvironmentalFactors {
                    noise_levels: HashMap::new(),
                    cultural_modifiers: HashMap::new(),
                    sync_conditions: SynchronizationConditions {
                        temporal_alignment: 0.8,
                        emotional_coherence: 0.7,
                        attention_synchrony: 0.9,
                        environmental_stability: 0.85,
                    },
                },
            },
        };
        
        weights.calculate_selection_probability(&experience, 2.0);
        
        assert!(weights.selection_probability.is_some());
        let prob = weights.selection_probability.unwrap();
        assert!(prob > 0.0 && prob <= 1.0);
    }
} 