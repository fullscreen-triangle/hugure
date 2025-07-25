use hugure::{Hugure, HugureConfig};
use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("hugure=info")
        .init();

    info!("Initializing Hugure: Virtual BMD Orchestration Framework");

    // Load configuration
    let config = HugureConfig::default();
    info!("Target exploration rate: {} BMDs/second", config.exploration_rate);
    info!("Amplification depth: {}", config.amplification_depth);
    info!("Emergence threshold: {}", config.emergence_threshold);

    // Initialize Hugure system
    let hugure = match Hugure::new(config).await {
        Ok(system) => {
            info!("Hugure system initialized successfully");
            system
        },
        Err(e) => {
            error!("Failed to initialize Hugure: {}", e);
            return Err(e);
        }
    };

    // Register as Kambuzuma neural task
    info!("Registering Hugure as Kambuzuma neural communication task");
    if let Err(e) = hugure.initialize_as_neural_task().await {
        error!("Failed to register with Kambuzuma: {}", e);
        return Err(e);
    }

    info!("Hugure neural task registration successful");

    // Display initial performance metrics
    match hugure.get_performance_metrics().await {
        Ok(metrics) => {
            info!("System Performance Metrics:");
            info!("  Exploration Rate: {} BMDs/second", metrics.exploration_rate);
            info!("  Amplification Factor: {}", metrics.amplification_factor);
            info!("  Emergence Accuracy: {:.4}%", metrics.emergence_accuracy * 100.0);
            info!("  Adaptation Efficiency: {:.4}", metrics.adaptation_efficiency);
            info!("  Neural Allocation: {:?}", metrics.neural_allocation);
        },
        Err(e) => {
            error!("Failed to retrieve performance metrics: {}", e);
        }
    }

    info!("Hugure communication orchestrator ready for BMD exploration");
    info!("System now available for Virtual BMD Foundry coordination");

    // Keep the system running for neural task coordination
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        
        // Periodic health check
        if let Ok(metrics) = hugure.get_performance_metrics().await {
            if metrics.exploration_rate < 1_000_000_000_000_000 {
                error!("Exploration rate below target: {} BMDs/second", metrics.exploration_rate);
            }
        }
    }
} 