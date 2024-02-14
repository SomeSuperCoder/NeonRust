pub mod seed_words;

pub const NEON_PARTS: u32 = 1_000_000;

pub const SLOT_LENGTH: std::time::Duration = std::time::Duration::from_secs(3);
pub const EPOCH_SLOTS: i16 = 100;

pub const SYSTEM_PROGRAM_ADDRESS: &'static str = "System";

pub const VERSION: i8 = 1;
