use anchor_lang::prelude::*;

#[error_code]
pub enum ProgramError {
    #[msg("Metadata account does not exist")]
    MetadataDoesntExist,
    #[msg("Edition account does not exist")]
    EditionDoesntExist,
    #[msg("Derived key invalid")]
    DerivedKeyInvalid,
    #[msg("Invalid pair type")]
    InvalidPairType,
    #[msg("Invalid bonding curve")]
    InvalidBondingCurve,
    #[msg("Invalid delta")]
    MetadataDeserializeError,
    #[msg("Invalid collection")]
    InvalidCollection,
    #[msg("Invalid collection mints")]
    InvalidCollectionMint,
    #[msg("Invalid collection details")]
    InvalidCollectionDetails,
    #[msg("Invalid funding amount")]
    InvalidFundingAmount,
    #[msg("Invalid owner")]
    InvalidOwner,
    #[msg("Invalid quote token mint")]
    InvalidQuoteTokenMint,
    #[msg("Nft is not part of a verified collection")]
    NftNotVerified,
    #[msg("Pair is not active")]
    PairNotActive,
    #[msg("fee param must be between 0 and 10000")]
    InvalidFee,
    #[msg("delta param must be between 0 and 10000 for bonding curve 1")]
    InvalidDelta,
    #[msg("Fee too large")]
    FeeTooLarge,
}
