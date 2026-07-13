//! Arrow Alignment Verification Engine.
//!
//! This provider requires the client to rotate an arrow to point at a specific
//! shape distributed along the perimeter of a circle.
//!
//! Anti-Bot Mechanisms:
//! 1. Telemetry verification ensures the rotational movement velocity is humanly realistic.
//! 2. Obfuscation offsets require the client to mathematically shift their final answer,
//!    preventing visual AI models from blindly submitting screen-scraped coordinates.

use super::VerificationProvider;
use crate::config::VerificationConfig;
use async_trait::async_trait;
use oclink_human_verification_crypto::CryptoEngine;
use rand::seq::IndexedRandom;
use rand::RngExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tonic::Status;
use tracing::info;

/// The central dictionary of recognizable shapes and their corresponding localized nouns.
const SHAPE_DICTIONARY: &[(&str, &str)] = &[
    ("🍎", "Apple"),
    ("🚗", "Car"),
    ("🚀", "Rocket"),
    ("🐶", "Dog"),
    ("⚽", "Soccer Ball"),
    ("🎸", "Guitar"),
    ("🍕", "Pizza"),
    ("⭐", "Star"),
    ("🎈", "Balloon"),
    ("💎", "Diamond"),
];

/// Tracks potential type of clock anomalies for bot heuristic tracking.
#[derive(Debug, PartialEq)]
enum ClockAnomaly {
    None,
    TimeReversal,
    InstantJump,
}

/// Instructs the client UI on where to render a specific emoji/shape.
#[derive(Serialize)]
pub struct ShapePlacement {
    pub emoji: String,
    pub visual_angle: f32,
}

/// The outbound challenge payload sent to the client to initialize the puzzle UI.
#[derive(Serialize)]
pub struct ArrowChallengePayload {
    pub prompt: String,
    pub shapes: Vec<ShapePlacement>,
    pub start_angle: f32,
    /// A required mathematical shift the client must apply to their final answer.
    pub obfuscation_offset: f32,
    pub state_token: String,
}

/// The internal stateless truth sealed inside the HMAC-SHA256 `state_token`.
#[derive(Serialize, Deserialize)]
pub struct ExpectedState {
    pub prompt: String,
    pub target_visual_angle: f32,
    pub start_angle: f32,
    /// The mathematically shifted clockwise rotation the client is expected to submit.
    pub expected_final_answer: f32,
}

/// A snapshot in time of the client's continuous rotational input.
#[derive(Deserialize, Debug)]
pub struct ArrowTick {
    /// The cumulative clockwise rotation applied by the user at this tick.
    pub rotation: f32,
    /// Seconds elapsed since the drag interaction began.
    pub t: f64,
}

/// The payload expected from the client upon puzzle completion.
#[derive(Deserialize)]
pub struct ArrowVerifyPayload {
    pub state_token: String,
    /// The final clockwise rotation mathematically shifted by the `obfuscation_offset`.
    pub final_answer: f32,
    pub ticks: Vec<ArrowTick>,
}

/// Service managing the lifecycle of Arrow Alignment verifications.
pub struct ArrowAlignmentProvider {
    config: Arc<VerificationConfig>,
    crypto: Arc<CryptoEngine>,
}

impl ArrowAlignmentProvider {
    /// Initializes the provider with the injected environment configuration and cryptographic engine.
    pub fn new(config: Arc<VerificationConfig>, crypto: Arc<CryptoEngine>) -> Self {
        Self { config, crypto }
    }
}

#[async_trait]
impl VerificationProvider for ArrowAlignmentProvider {
    fn generate_challenge(&self, _edition_id: &str) -> Result<Value, Status> {
        let mut rng = rand::rng();

        let total_shapes = std::cmp::min(self.config.shape_count, SHAPE_DICTIONARY.len());

        let selected_shapes: Vec<(&str, &str)> = SHAPE_DICTIONARY
            .sample(&mut rng, total_shapes)
            .copied()
            .collect();

        let mut placements = Vec::with_capacity(total_shapes);
        let min_separation = self.config.min_arrow_icon_degrees;

        // Distribute the selected shapes unpredictably around the circle, ensuring they don't overlap.
        for &(emoji, _noun) in selected_shapes.iter() {
            let mut angle = 0.0;

            // Attempt to find a valid random spot up to 100 times to prevent infinite loops
            for _ in 0..100 {
                angle = rng.random_range(0.0..360.0);
                let valid = placements.iter().all(|p: &ShapePlacement| {
                    let diff = (p.visual_angle - angle).abs();
                    let circ_diff = f32::min(diff, 360.0 - diff);
                    circ_diff >= min_separation
                });

                if valid {
                    break;
                }
            }

            placements.push(ShapePlacement {
                emoji: emoji.to_string(),
                visual_angle: angle,
            });
        }

        // Select the specific shape the user is required to target.
        let target_shape = selected_shapes.choose(&mut rng).unwrap();
        let target_visual_angle = placements
            .iter()
            .find(|p| p.emoji == target_shape.0)
            .map(|p| p.visual_angle)
            .unwrap();

        let prompt = format!("Align the arrow with the {}.", target_shape.1);

        // Generate randomized starting parameters to ensure no two puzzles are identical.
        let start_angle: f32 = rng.random_range(0.0..360.0);
        let obfuscation_offset: f32 = rng.random_range(0.0..360.0);

        let clockwise_rotation_required = (target_visual_angle - start_angle + 360.0) % 360.0;
        let expected_final_answer = (clockwise_rotation_required + obfuscation_offset) % 360.0;

        if self.config.debug_mode {
            info!(
                    "\n=== ARROW ALIGNMENT GENERATED ===\nTarget Shape: {}\nTarget Visual Angle: {:.2}\nStart Angle: {:.2}\nRequired Slider Rotation (Base Offset): {:.2}\nObfuscation Offset: {:.2}\n--> EXPECTED FINAL SUBMISSION: {:.2}\n=================================",
                    target_shape.1, target_visual_angle, start_angle, clockwise_rotation_required, obfuscation_offset, expected_final_answer
                );
        }

        let expected_state = ExpectedState {
            prompt: prompt.clone(),
            target_visual_angle,
            start_angle,
            expected_final_answer,
        };

        // Cryptographically seal the expected answer so the server remains strictly stateless.
        let state_token = self
            .crypto
            .sign_state(&expected_state, self.config.token_timeout_secs)
            .unwrap();

        let payload = ArrowChallengePayload {
            prompt,
            shapes: placements,
            start_angle,
            obfuscation_offset,
            state_token,
        };

        Ok(serde_json::to_value(payload).unwrap())
    }

    async fn verify(
        &self,
        payload: &Value,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let data: ArrowVerifyPayload = serde_json::from_value(payload.clone())?;

        if self.config.debug_mode {
            info!(
                "\n=== ARROW VERIFICATION INBOUND ===\nClient Final Answer: {:.4}\nClient Tick Count: {}\n==================================",
                data.final_answer, data.ticks.len()
            );
        }

        // Structural sanity verification: ensure too much or too little data wasn't provided.
        if data.ticks.len() > self.config.max_ticks {
            if self.config.debug_mode {
                info!(
                    "Bot Flagged: Excessive telemetry stream packet limit exceeded. ({} > {})",
                    data.ticks.len(),
                    self.config.max_ticks
                );
            }
            return Ok(false);
        }
        if data.ticks.len() < self.config.min_ticks {
            if self.config.debug_mode {
                info!(
                    "Bot Flagged: Insufficient telemetry data stream density ({} < {}).",
                    data.ticks.len(),
                    self.config.min_ticks
                );
            }
            return Ok(false);
        }

        // Ensure cryptographic payload is valid and has not expired.
        let expected_state: ExpectedState = match self.crypto.verify_state(&data.state_token) {
            Some(state) => state,
            None => {
                if self.config.debug_mode {
                    info!("Verification Blocked: Cryptographic payload signature check failed.");
                }
                return Ok(false);
            }
        };

        // Exact match prevention. Blocks bots that perfectly snap to the target.
        let absolute_diff = (data.final_answer - expected_state.expected_final_answer).abs();
        let circular_diff = f32::min(absolute_diff, 360.0 - absolute_diff);

        if circular_diff < self.config.exact_match_threshold {
            if self.config.debug_mode {
                info!("Bot Flagged: Supernatural precision detected. Perfect floating point target snap.");
            }
            return Ok(false);
        }

        // =========================================================================
        // ADVANCED REACTION TIMING ANALYSIS
        // =========================================================================
        // Determine exactly when physical slider interaction ceased relative to submission.
        let mut last_movement_time = data.ticks[0].t;
        let final_tick = data.ticks.last().unwrap();

        for tick in data.ticks.iter().rev() {
            if (tick.rotation - final_tick.rotation).abs() > self.config.exact_match_threshold {
                last_movement_time = tick.t;
                break;
            }
        }

        let total_elapsed_time = final_tick.t;
        let submit_reaction_delay = total_elapsed_time - last_movement_time;

        // Reject instant programmatic actions.
        // In practice humans require time to switch context
        // from shifting the slider to locating and clicking 'Submit'.
        if submit_reaction_delay < self.config.submit_reaction_min_delay {
            if self.config.debug_mode {
                info!(
                    "Bot Flagged: Instantaneous interaction transition detected. Humanly impossible click reflex. (Delay: {:.4}s < {}s)",
                    submit_reaction_delay, self.config.submit_reaction_min_delay
                );
            }
            return Ok(false);
        }

        // Anti-spam displacement heuristic.
        // Calculate the total absolute angular distance traveled across the stream.
        // This ensures the user actually interacted with and spun the wheel.
        let total_displacement: f32 = data
            .ticks
            .windows(2)
            .map(|window| {
                let diff = (window[1].rotation - window[0].rotation).abs();
                f32::min(diff, 360.0 - diff) // Handle circular wrapping limits.
            })
            .sum();

        if total_displacement < self.config.min_total_rotation {
            if self.config.debug_mode {
                info!(
                    "Bot Flagged: Insufficient physical engagement. Total slider arc movement below minimum requirement. ({:.4}° < {}°)",
                    total_displacement, self.config.min_total_rotation
                );
            }
            return Ok(false);
        }

        // Map vector streams and calculate mathematical behavioral scores.
        let mut zero_velocity_ticks = 0;
        let mut exact_tick_intervals = 0;
        let mut velocity_curve_violations = 0;

        let mut clock_status = ClockAnomaly::None;
        let mut offending_dt = 0.0;
        let total_ticks_count = data.ticks.len();

        let velocities: Vec<f32> = data
            .ticks
            .windows(2)
            .filter_map(|window| {
                let t1 = &window[0];
                let t2 = &window[1];
                let dt = t2.t - t1.t;

                // --- Stripped timing integrity check ---
                // Only validate formatting errors (time travel) or programmatic instant actions.
                // Mid-movement pauses, hesitations, and stutters are intentionally ignored.
                if dt < 0.0 {
                    clock_status = ClockAnomaly::TimeReversal;
                    offending_dt = dt;
                    return None;
                } else if dt <= self.config.min_delta_time {
                    clock_status = ClockAnomaly::InstantJump;
                    offending_dt = dt;
                    return None;
                }

                let rot_diff = (t2.rotation - t1.rotation).abs();
                let circ_diff = f32::min(rot_diff, 360.0 - rot_diff);

                // Track geometric structure formatting to discover clean looped synthetic updates.
                if circ_diff < self.config.exact_match_threshold {
                    zero_velocity_ticks += 1;
                }
                if (circ_diff - circ_diff.round()).abs() < self.config.exact_match_threshold {
                    exact_tick_intervals += 1;
                }

                // Calculate velocity. If a human paused during this window, dt is large,
                // causing current_velocity to drop cleanly near 0.0—which perfectly passes velocity caps!
                let current_velocity = circ_diff / (dt as f32);

                // --- Biometric control filter (Fitt's Law) ---
                // Validates target acquisition times against the physical distance traveled.
                // Uses Fitts's Law models to verify that the acceleration and deceleration phases
                // align with biological motor control limits rather than instantaneous bot positioning.
                let distance_to_finish = (t2.rotation - data.final_answer).abs();
                let circ_distance_to_finish =
                    f32::min(distance_to_finish, 360.0 - distance_to_finish);

                let allowed_speed_limit = if circ_distance_to_finish
                    > self.config.approach_threshold_degrees
                {
                    self.config.human_start_max_velocity
                } else {
                    let progress = circ_distance_to_finish / self.config.approach_threshold_degrees;
                    self.config.human_final_max_velocity
                        + (self.config.human_start_max_velocity
                            - self.config.human_final_max_velocity)
                            * progress
                };

                if current_velocity > allowed_speed_limit {
                    velocity_curve_violations += 1;
                }

                Some(current_velocity)
            })
            .collect();

        // Time integrity evaluation.
        if clock_status != ClockAnomaly::None {
            if self.config.debug_mode {
                let mut found_index = 0;
                for (index, window) in data.ticks.windows(2).enumerate() {
                    let dt = window[1].t - window[0].t;
                    if (clock_status == ClockAnomaly::TimeReversal && dt < 0.0)
                        || (clock_status == ClockAnomaly::InstantJump
                            && dt <= self.config.min_delta_time)
                    {
                        found_index = index;
                        break;
                    }
                }

                let location_str =
                    format!("Window Index: [{} -> {}]", found_index, found_index + 1);

                match clock_status {
                    ClockAnomaly::TimeReversal => {
                        info!(
                            "Bot Flagged: Explicit time-reversal timestamp hack detected. (dt: {:.4}s) | Total Ticks: {} | Location: {}",
                            offending_dt, total_ticks_count, location_str
                        );
                    }
                    ClockAnomaly::InstantJump => {
                        info!(
                            "Bot Flagged: Teleportation / Instant macro execution detected. Frames grouped too closely. (dt: {:.4}s) | Total Ticks: {} | Location: {}",
                            offending_dt, total_ticks_count, location_str
                        );
                    }
                    _ => {}
                }
            }
            return Ok(false);
        }

        if velocities.is_empty() {
            return Ok(false);
        }

        // Heuristic processing rules evaluation.

        // Test A: Target approach deceleration curve check.
        // Verifies that input velocity dynamically scales down as the cursor nears the target.
        // Humans naturally slow down for fine-motor adjustments during the final aiming phase,
        // whereas bots often maintain unnaturally uniform speeds.
        if velocity_curve_violations > self.config.max_spike_tolerance_count {
            if self.config.debug_mode {
                info!(
                    "Bot Flagged: Violated biometric curve deceleration requirements {} times.",
                    velocity_curve_violations
                );
            }
            return Ok(false);
        }

        // Test B: Speed uniformity check.
        // Flags mathematical, robotic lerps, or constant speeds.
        let avg_velocity: f32 = velocities.iter().sum::<f32>() / velocities.len() as f32;
        let variance: f32 = velocities
            .iter()
            .map(|&v| (v - avg_velocity).powi(2))
            .sum::<f32>()
            / velocities.len() as f32;

        if variance < self.config.human_min_velocity_variance {
            if self.config.debug_mode {
                info!("Bot Flagged: Motion path variance too uniform ({:.6}). Lack of organic input jitter.", variance);
            }
            return Ok(false);
        }

        // Test C: Rigid loop increments filter.
        // Tests if too many ticks were exact integral increments.
        if exact_tick_intervals == velocities.len() {
            if self.config.debug_mode {
                info!("Bot Flagged: Perfect integer coordinate changes detected. Formatted sequence loop.");
            }
            return Ok(false);
        }

        // Test D: Padding stream detection.
        // Tests for too many telemetry ticks with zero velocity.
        if zero_velocity_ticks > (velocities.len() / 2) {
            if self.config.debug_mode {
                info!(
                    "Bot Flagged: Artificial padding sequence. Too many zero velocity adjustments."
                );
            }
            return Ok(false);
        }

        // Final target validity check. Check if answer is within tolerance.
        let is_match = circular_diff <= self.config.tolerance_degrees;

        if self.config.debug_mode {
            if is_match {
                info!(
                    "Arrow Alignment Passed! Final Accuracy Offset: {:.4} degrees.",
                    circular_diff
                );
            } else {
                info!("Arrow Alignment Failed! Final Accuracy Offset missing target by {:.4} degrees.", circular_diff);
            }
        }

        Ok(is_match)
    }
}
