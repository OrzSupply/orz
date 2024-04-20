use solana_program::{keccak::Hash, pubkey, pubkey::Pubkey};

/// The unix timestamp after which mining can begin.
pub const START_AT: i64 = 1712070600;

/// The reward rate to intialize the program with.
pub const INITIAL_REWARD_RATE: u64 = 10u64.pow(3u32);

/// The mining difficulty to initialize the program with.
pub const INITIAL_DIFFICULTY: Hash = Hash::new_from_array([
    0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
]);

/// The decimal precision of the ORE token.
/// Using SI prefixes, the smallest indivisible unit of ORE is a nanoORE.
/// 1 nanoORE = 0.000000001 ORE = one billionth of an ORE
pub const TOKEN_DECIMALS: u8 = 9;

/// One ORE token, denominated in units of nanoORE.
pub const ONE_ORE: u64 = 10u64.pow(TOKEN_DECIMALS as u32);

/// Capped supply at 21M.
pub const TOTAL_SUPPLY: u64 = 21000000 * ONE_ORE;

/// The duration of an epoch, in units of seconds.
pub const EPOCH_DURATION: i64 = 60;

/// The target quantity of ORE to be mined per epoch, in units of nanoORE.
/// Inflation rate ≈ 1 ORE / epoch (min 0, max 2)
pub const TARGET_EPOCH_REWARDS: u64 = ONE_ORE;

/// The maximum quantity of ORE that can be mined per epoch, in units of nanoORE.
pub const MAX_EPOCH_REWARDS: u64 = ONE_ORE.saturating_mul(2);

/// The quantity of ORE each bus is allowed to issue per epoch.
pub const BUS_EPOCH_REWARDS: u64 = MAX_EPOCH_REWARDS.saturating_div(BUS_COUNT as u64);

/// The number of bus accounts, for parallelizing mine operations.
pub const BUS_COUNT: usize = 8;

/// The smoothing factor for reward rate changes. The reward rate cannot change by more or less
/// than a factor of this constant from one epoch to the next.
pub const SMOOTHING_FACTOR: u64 = 2;

// Assert MAX_EPOCH_REWARDS is evenly divisible by BUS_COUNT.
static_assertions::const_assert!(
    (MAX_EPOCH_REWARDS / BUS_COUNT as u64) * BUS_COUNT as u64 == MAX_EPOCH_REWARDS
);

/// The seed of the bus account PDA.
pub const BUS: &[u8] = b"bus";

/// The seed of the metadata account PDA.
pub const METADATA: &[u8] = b"metadata";

/// The seed of the mint account PDA.
pub const MINT: &[u8] = b"mint";

/// The seed of proof account PDAs.
pub const PROOF: &[u8] = b"proof";

/// The seed of the treasury account PDA.
pub const TREASURY: &[u8] = b"treasury";

/// The name for token metadata.
pub const METADATA_NAME: &str = "Orz";

/// The ticker symbol for token metadata.
pub const METADATA_SYMBOL: &str = "ORZ";

/// The uri for token metdata.
pub const METADATA_URI: &str = "https://orz.supply/metadata.json";

/// Noise for deriving the mint PDA.
pub const MINT_NOISE: [u8; 16] = [
    166, 199, 85, 221, 225, 119, 21, 185, 160, 82, 242, 237, 194, 84, 250, 252,
];

/// The addresses of the bus accounts.
pub const BUS_ADDRESSES: [Pubkey; BUS_COUNT] = [
    pubkey!("38F53JbVmhu8ApqANgijxMsznrgVykts23qsbQRCR6NF"),
    pubkey!("JAQ9fh75riwJ56hq9MfZZYYskGEvT6jq3XG5gmSFnGVi"),
    pubkey!("CB18Ut9cXn6fwgTStdNnYpQWyWZ3VAwq9YDBD1azVKzX"),
    pubkey!("2M1CoMN7uyjCg3JYcnLs7CYxMRfnabC4aRuPp7QqTRV2"),
    pubkey!("8WETJQ2KcDcUA7PA2jMzBcPudko2RagQBf6pcXF6rVN4"),
    pubkey!("CQnmYUJBnTZGT4iQU1iEjBmM1oyMb4h98AoTETrsQqLB"),
    pubkey!("3WEH5D43hPVyQ2ZicR4ZYf3g8E3wr5na5BnWNEuLroPa"),
    pubkey!("GqwdPTLizBcDs856CRurPTryWPPiG86YgDd7WbatM2SA"),
];

/// The address of the mint metadata account.
pub const METADATA_ADDRESS: Pubkey = pubkey!("Eir4pmjx83z8MAUfTpCCxiAMCiumqKKcwtoZ7aqkPbgu");

/// The address of the mint account.
pub const MINT_ADDRESS: Pubkey = pubkey!("3eEUXWsvEF3Mrd2m7VWWz9Akk7PirNJi9uZNcG93KoSM");

/// The address of the treasury account.
pub const TREASURY_ADDRESS: Pubkey = pubkey!("12iUmMQWTzSpmgKxTP1vgWK4h7aBntNe114RdzEpeDz4");
