//! # Chimera Core
//!
//! This crate provides the core data structures and logic for the Chimera network.
//! It includes the CHIM-Lang DSL for defining intents and the basic structures for zk-Agents.
//! This is the foundational layer for building applications and services on Chimera.

pub mod agent;
pub mod dsl;

#[cfg(test)]
mod tests {
    use super::agent::{Agent, IntentProcessor};
    use super::dsl::Intent;

    #[test]
    fn it_works() {
        let agent = Agent {
            id: "agent-007".to_string(),
            ..Default::default()
        };

        let intent = Intent::new_yield_farming_intent(100_000);
        let result = agent.process_intent(&intent);

        assert!(result.contains(&agent.id));
        assert!(result.contains(&intent.name));
    }
}
