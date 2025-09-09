//! # CHIM-Lang: The Chimera Intent Language DSL
//!
//! This module defines the core data structures for expressing user intents within the Chimera network.
//! The primary structure is the `Intent`, which captures the user's desired outcome without
//! specifying the execution steps.

/// Represents the decentralized exchanges (DEXs) that can be specified in an intent's constraints.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Dex {
    UniswapV3,
    CurveFi,
    PancakeSwap,
}

/// Defines the constraints that must be adhered to when fulfilling an intent.
#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    /// Maximum portfolio drawdown allowed (e.g., 0.05 for 5%).
    MaxDrawdown(f32),
    /// A list of specific DEXs that are allowed for executing the intent.
    AllowedDexes(Vec<Dex>),
    /// A custom, string-based constraint for flexibility.
    Custom(String),
}

/// Defines the preferences that solvers should consider when fulfilling an intent.
/// These are soft requirements, unlike constraints.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Preference {
    /// Preference for solutions that are resistant to Maximal Extractable Value (MEV).
    MevResistant,
    /// Preference for solutions that are carbon-neutral.
    CarbonNeutral,
    /// A custom, string-based preference for flexibility.
    Custom(String),
}

/// Defines the expiration condition for an intent.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expiration {
    /// The intent expires at a specific block height.
    AtBlock(u64),
    /// The intent never expires.
    Never,
}

/// The core `Intent` structure in CHIM-Lang.
/// It declaratively represents a user's goal.
#[derive(Debug, Clone, PartialEq)]
pub struct Intent {
    /// A human-readable name for the intent, like "YieldFarming".
    pub name: String,
    /// The primary goal of the intent, expressed as a string for now.
    /// E.g., "Portfolio ROI >= 15% APY".
    pub target: String,
    /// A vector of hard constraints that must be met.
    pub constraints: Vec<Constraint>,
    /// A vector of soft preferences to guide solvers.
    pub preferences: Vec<Preference>,
    /// The condition under which the intent expires.
    pub expiration: Expiration,
}

impl Intent {
    /// A constructor function to create an example `YieldFarming` intent based on the whitepaper.
    ///
    /// This is useful for testing and demonstration purposes.
    pub fn new_yield_farming_intent(current_block_height: u64) -> Self {
        Self {
            name: "YieldFarming".to_string(),
            target: "Portfolio ROI >= 15% APY".to_string(),
            constraints: vec![
                Constraint::MaxDrawdown(0.05),
                Constraint::AllowedDexes(vec![
                    Dex::UniswapV3,
                    Dex::CurveFi,
                    Dex::PancakeSwap,
                ]),
            ],
            preferences: vec![
                Preference::MevResistant,
                Preference::CarbonNeutral,
            ],
            expiration: Expiration::AtBlock(current_block_height + 5000),
        }
    }
}
