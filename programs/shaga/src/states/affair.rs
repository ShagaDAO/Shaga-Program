// states/affair.rs

use crate::errors::ShagaErrorCode;
use crate::seeds::SEED_AFFAIR;
use anchor_lang::prelude::*;
use solana_program::blake3::hashv as blake3_hashv;
use solana_program::hash::hashv as sha256_hashv;
use solana_program::keccak::hashv as keccak_hashv;

#[derive(InitSpace, Debug, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AffairState {
    Unavailable,
    Available,
}

impl Default for AffairState {
    fn default() -> Self {
        AffairState::Available
    }
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Affair {
    pub authority: Pubkey,
    pub client: Pubkey,
    pub rental: Option<Pubkey>,
    // the shaga coordinates system.
    // built around online maps coordinates system.
    // precision does not have to be accurate to the dot that is why only three decimal points are used.
    // values can be negative. with the format of: ±DD.DDD,±DDD.DDD (lat,long)
    #[max_len(17)]
    pub coordinates: String,
    #[max_len(15)]
    pub ip_address: String,
    #[max_len(64)]
    pub cpu_name: String,
    #[max_len(64)]
    pub gpu_name: String,
    pub total_ram_mb: u32,
    // in LAMPORTS_PER_SOL
    pub sol_per_hour: u64,
    pub affair_state: AffairState,
    pub affair_termination_time: u64,
    pub active_rental_start_time: u64,
    pub due_rent_amount: u64,
    pub hash_algorithm: HashAlgorithm,
    #[max_len(33)]
    pub private_pair_hash: Option<Vec<u8>>,
}

#[derive(InitSpace, Debug, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum HashAlgorithm {
    None,
    Sha256,
    Keccak,
    Blake3,
}

impl Default for Affair {
    fn default() -> Self {
        Self {
            authority: Pubkey::default(),
            client: Pubkey::default(),
            rental: Option::from(Pubkey::default()),
            coordinates: "".to_string(),
            ip_address: "".to_string(),
            cpu_name: "".to_string(),
            gpu_name: "".to_string(),
            total_ram_mb: 0,
            sol_per_hour: 0,
            affair_state: AffairState::default(),
            affair_termination_time: 0,
            active_rental_start_time: 0,
            due_rent_amount: 0,
            hash_algorithm: HashAlgorithm::None,
            private_pair_hash: None,
        }
    }
}

impl<'a> Affair {
    pub const HASH_PREFIX: &'a str = "Shaga Private Pair Hash";
    pub fn join(&mut self, rental_key: Pubkey) -> Result<()> {
        if self.affair_state != AffairState::Available {
            msg!("Affair is not available for joining.");
            return Err(ShagaErrorCode::AffairAlreadyJoined.into());
        }

        self.rental = Some(rental_key);
        self.affair_state = AffairState::Unavailable;
        Ok(())
    }

    pub fn pda(owner: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED_AFFAIR, owner.as_ref()], &crate::ID)
    }

    pub fn size() -> usize {
        8 + Affair::INIT_SPACE
    }

    pub fn can_join(&self) -> bool {
        self.affair_state == AffairState::Available
    }

    pub fn deserialize_data(src: &[u8]) -> Result<Affair> {
        let mut p = src;
        let affair = Affair::try_deserialize(&mut p)?;
        Ok(affair)
    }

    pub fn verify_private_pair_hash(&self, private_pair_code: &str) -> Result<()> {
        let binding = Self::HASH_PREFIX.to_owned() + private_pair_code;
        let private_pair_code_bytes = binding.as_bytes();

        let computed_private_pair_hash = match self.hash_algorithm {
            HashAlgorithm::Sha256 => sha256_hashv(&[private_pair_code_bytes]).to_bytes().to_vec(),
            HashAlgorithm::Keccak => keccak_hashv(&[private_pair_code_bytes]).to_bytes().to_vec(),
            HashAlgorithm::Blake3 => blake3_hashv(&[private_pair_code_bytes]).to_bytes().to_vec(),
            HashAlgorithm::None => {
                msg!("Hash algorithm not set.");
                return Err(ShagaErrorCode::HashAlgoNotSet.into());
            }
        };

        if let Some(expected_private_pair_hash) = &self.private_pair_hash {
            if computed_private_pair_hash != *expected_private_pair_hash {
                msg!("Private pair code mismatch.");
                return Err(ShagaErrorCode::CodeMismatch.into());
            }
        } else {
            msg!("No private pair hash set in affair.");
            return Err(ShagaErrorCode::MissingPrivatePairHash.into());
        }

        Ok(())
    }
}
