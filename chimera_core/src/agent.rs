//! # Chimera Agent Structure
//!
//! This module defines the `Agent` and its related components, such as `Reputation`.
//! Agents are the off-chain actors in the Chimera network responsible for discovering
//! and fulfilling user intents.

use crate::dsl::Intent;

/// Represents the reputation of a zk-Agent.
///
/// The score is calculated based on various performance metrics, as described in the
/// Chimera whitepapers. A higher score indicates a more reliable and efficient agent.
#[derive(Debug, Clone, PartialEq)]
pub struct Reputation {
    /// The overall reputation score.
    pub score: f64,
    /// The accuracy of the agent's solutions (0.0 to 1.0).
    pub accuracy: f64,
    /// The gas efficiency of the agent's transactions.
    pub gas_efficiency: f64,
    /// The time taken to generate proofs.
    pub proof_time: f64,
    /// The agent's uptime (0.0 to 1.0).
    pub uptime: f64,
    /// The amount of stake the agent has.
    pub stake: f64,
}

impl Default for Reputation {
    /// Creates a default reputation for a new agent.
    fn default() -> Self {
        Self {
            score: 0.0,
            accuracy: 0.0,
            gas_efficiency: 0.0,
            proof_time: 0.0,
            uptime: 1.0, // Agents start with perfect uptime.
            stake: 0.0,
        }
    }
}

impl Reputation {
    /// Updates the reputation score based on the formula from the whitepaper.
    ///
    /// This uses the more detailed formula from the v2.1 whitepaper, with an
    /// adjustment for gas efficiency inspired by `teste.md`.
    pub fn update(&mut self) {
        // Formula: 0.4*Accuracy + 0.3*(1/GasEfficiency) + 0.2*Uptime + 0.1*Stake
        let gas_term = if self.gas_efficiency > 0.0 { 1.0 / self.gas_efficiency } else { 0.0 };
        self.score = 0.4 * self.accuracy + 0.3 * gas_term + 0.2 * self.uptime + 0.1 * self.stake;
    }
}

/// Represents a zk-Agent in the Chimera network.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Agent {
    /// A unique identifier for the agent.
    pub id: String,
    /// The agent's reputation.
    pub reputation: Reputation,
}

/// A trait defining the core behavior of an agent: processing intents.
pub trait IntentProcessor {
    /// The type of output produced after processing an intent.
    /// In a real scenario, this would be a complex proof or transaction data.
    type Output;

    /// Processes a given intent and returns a result.
    ///
    /// This is a placeholder for the complex logic an agent would perform,
    /// such as finding optimal solutions, generating ZK proofs, and submitting transactions.
    fn process_intent(&self, intent: &Intent) -> Self::Output;
}

impl IntentProcessor for Agent {
    type Output = String;

    fn process_intent(&self, intent: &Intent) -> Self::Output {
        // Placeholder logic for a "Single-Chain Agent".
        format!("Agent {} is processing intent: {}", self.id, intent.name)
    }
}
