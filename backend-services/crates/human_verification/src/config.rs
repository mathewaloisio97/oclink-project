//! Environment-driven configuration for the human verification subsystem.

use std::env;

/// Configuration parameters for puzzle generation and telemetry analysis.
#[derive(Clone, Debug)]
pub struct VerificationConfig {
    /// Maximum lifespan of an issued verification token in seconds.
    pub token_timeout_secs: u64,

    /// Enables verbose stdout logging. Should be disabled in production
    /// to optimize performance and prevent telemetry data leaks.
    pub debug_mode: bool,

    // =========================================================================
    // Arrow Alignment Challenge Settings
    // =========================================================================
    /// Total number of shapes generated in the challenge interface.
    pub shape_count: usize,

    /// Minimum telemetry samples required to accept a submission.
    pub min_ticks: usize,

    /// Maximum telemetry samples allowed before rejecting a submission as synthetic.
    pub max_ticks: usize,

    /// Maximum allowable deviation (in degrees) from the target alignment.
    pub tolerance_degrees: f32,

    /// Minimum angular separation required between spawned challenge items to prevent overlaps.
    pub min_arrow_icon_degrees: f32,

    // =========================================================================
    // Anti-Bot Telemetry Heuristics
    // =========================================================================
    /// Minimum allowed time delta between sequential updates (seconds) to filter out instant actions.
    pub min_delta_time: f64,

    /// Minimum total rotation distance required to prove the user interacted with the UI.
    pub min_total_rotation: f32,

    /// Maximum speed allowed during the initial rapid adjustment phase (degrees/sec).
    pub human_start_max_velocity: f32,

    /// Maximum speed allowed as the user approaches the exact target (degrees/sec).
    pub human_final_max_velocity: f32,

    /// Angular distance from target where velocity scaling shifts from start to final constraints.
    pub approach_threshold_degrees: f32,

    /// Minimum variance in velocity to reject unnaturally uniform linear movements.
    pub human_min_velocity_variance: f32,

    /// Allowable number of speed spikes before failing verification (e.g., handling system stutter).
    pub max_spike_tolerance_count: usize,

    /// Value threshold used to detect exact integer inputs or perfect floating-point snaps.
    pub exact_match_threshold: f32,

    /// Minimum duration (seconds) required between final input movement and pressing submit.
    pub submit_reaction_min_delay: f64,
}

impl VerificationConfig {
    /// Loads parameters from environment variables, falling back to defaults if unset.
    /// Panics during initialization if values cannot be parsed into expected numeric types.
    pub fn from_env() -> Self {
        Self {
            token_timeout_secs: env::var("HUMAN_VERIFICATION_TOKEN_TIMEOUT")
                .unwrap_or_else(|_| "900".to_string())
                .parse()
                .expect("Invalid token timeout"),
            debug_mode: env::var("HUMAN_VERIFICATION_DEBUG_MODE")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),

            // --- Arrow Alignment Challenge Settings ---
            shape_count: env::var("ARROW_VERIFICATION_SHAPE_COUNT")
                .unwrap_or_else(|_| "6".to_string())
                .parse()
                .expect("Invalid shape count"),
            min_ticks: env::var("ARROW_VERIFICATION_MIN_TICKS")
                .unwrap_or_else(|_| "6".to_string())
                .parse()
                .expect("Invalid min ticks"),
            max_ticks: env::var("ARROW_VERIFICATION_MAX_TICKS")
                .unwrap_or_else(|_| "64".to_string())
                .parse()
                .expect("Invalid max ticks"),
            tolerance_degrees: env::var("ARROW_VERIFICATION_TOLERANCE_DEGREES")
                .unwrap_or_else(|_| "5.0".to_string())
                .parse()
                .expect("Invalid tolerance degrees"),
            min_arrow_icon_degrees: env::var("ARROW_MIN_ARROW_ICON_DEGREES")
                .unwrap_or_else(|_| "35.0".to_string())
                .parse()
                .expect("Invalid arrow icon spacing degrees"),

            // --- Anti-Bot Telemetry Heuristics ---
            min_delta_time: env::var("ARROW_MIN_DELTA_TIME")
                .unwrap_or_else(|_| "0.08".to_string())
                .parse()
                .expect("Invalid min delta time"),
            min_total_rotation: env::var("ARROW_MIN_TOTAL_ROTATION")
                .unwrap_or_else(|_| "0.1".to_string())
                .parse()
                .expect("Invalid min total rotation"),
            human_start_max_velocity: env::var("ARROW_START_MAX_VELOCITY")
                .unwrap_or_else(|_| "2500.0".to_string())
                .parse()
                .expect("Invalid start max velocity"),
            human_final_max_velocity: env::var("ARROW_FINAL_MAX_VELOCITY")
                .unwrap_or_else(|_| "700.0".to_string())
                .parse()
                .expect("Invalid final max velocity"),
            approach_threshold_degrees: env::var("ARROW_APPROACH_THRESHOLD_DEGREES")
                .unwrap_or_else(|_| "45.0".to_string())
                .parse()
                .expect("Invalid approach threshold degrees"),
            human_min_velocity_variance: env::var("ARROW_MIN_VELOCITY_VARIANCE")
                .unwrap_or_else(|_| "0.1".to_string())
                .parse()
                .expect("Invalid human min velocity variance"),
            max_spike_tolerance_count: env::var("ARROW_MAX_SPIKE_TOLERANCE_COUNT")
                .unwrap_or_else(|_| "2".to_string())
                .parse()
                .expect("Invalid spike tolerance count"),
            exact_match_threshold: env::var("ARROW_EXACT_MATCH_THRESHOLD")
                .unwrap_or_else(|_| "0.0001".to_string())
                .parse()
                .expect("Invalid exact match threshold"),
            submit_reaction_min_delay: env::var("ARROW_SUBMIT_REACTION_MIN_DELAY")
                .unwrap_or_else(|_| "0.045".to_string())
                .parse()
                .expect("Invalid submit reaction minimum delay"),
        }
    }
}
