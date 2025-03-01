use anchor_lang::prelude::*;

#[error_code]
pub enum ShagaErrorCode {
    #[msg("Invalid Session")]
    InvalidAffair,
    #[msg("Invalid Lender")]
    InvalidLender,
    #[msg("Invalid Payload")]
    InvalidPayload,
    #[msg("Sessions List Full")]
    AffairListFull,
    #[msg("Client Already in a Session")]
    ClientAlreadyInAffair,
    #[msg("Insufficient Funds")]
    InsufficientFunds,
    #[msg("Invalid Rental Termination Time")]
    InvalidRentalTerminationTime,
    #[msg("Invalid Termination Time")]
    InvalidTerminationTime,
    #[msg("Session Occupied")]
    AffairAlreadyJoined,
    #[msg("Thread Initialization Failed")]
    ThreadInitializationFailed,
    #[msg("Missing Rental Context for Session Termination")]
    MissingRentalContext,
    #[msg("Wrong Rental Context for Session Termination")]
    InvalidRentalContext,
    #[msg("Only registered lenders can create affairs")]
    UnauthorizedAffairCreation,
    #[msg("Only authority can terminate affairs")]
    UnauthorizedAffairTerminator,
    #[msg("Invalid Signer")]
    InvalidSigner,
    #[msg("Invalid Termination Instruction.")]
    InvalidTerminationInstruction,
    #[msg("Rental Clockwork Key Mismatch.")]
    InvalidRentalClockworkKey,
    #[msg("Numerical Overflow.")]
    NumericalOverflow,
    #[msg("Hash algorithm is None")]
    HashAlgoNotSet,
    #[msg("Private pair code mismatch.")]
    CodeMismatch,
    #[msg("Private pair hash is missing in affair.")]
    MissingPrivatePairHash,
    #[msg("Private pair code is missing in instruction.")]
    MissingPrivatePairCode,
    #[msg("Shaga Paused.")]
    ShagaPaused,
    #[msg("Invalid Clockwork Key mismatch")]
    ClockWorkKeyMismatch,
}
