#[cfg(feature = "std")]
use thiserror::Error;

#[cfg_attr(feature = "std", derive(Error))]
#[derive(Debug, PartialEq)]
pub enum Groth16Error {
    #[cfg_attr(feature = "std", error("Incompatible Verifying Key with number of public inputs"))]
    IncompatibleVerifyingKeyWithNrPublicInputs,
    #[cfg_attr(feature = "std", error("ProofVerificationFailed"))]
    ProofVerificationFailed,
    #[cfg_attr(feature = "std", error("PreparingInputsG1AdditionFailed"))]
    PreparingInputsG1AdditionFailed,
    #[cfg_attr(feature = "std", error("PreparingInputsG1MulFailed"))]
    PreparingInputsG1MulFailed,
    #[cfg_attr(feature = "std", error("InvalidG1Length"))]
    InvalidG1Length,
    #[cfg_attr(feature = "std", error("InvalidG2Length"))]
    InvalidG2Length,
    #[cfg_attr(feature = "std", error("InvalidPublicInputsLength"))]
    InvalidPublicInputsLength,
    #[cfg_attr(feature = "std", error("DecompressingG1Failed"))]
    DecompressingG1Failed,
    #[cfg_attr(feature = "std", error("DecompressingG2Failed"))]
    DecompressingG2Failed,
    #[cfg_attr(feature = "std", error("PublicInputGreaterThanFieldSize"))]
    PublicInputGreaterThanFieldSize,
    #[cfg_attr(feature = "std", error("Failed to convert proof component to byte array"))]
    ProofConversionError,
    #[cfg(feature = "circom")]
    #[cfg_attr(feature = "std", error("Arkworks serialization error"))]
    ArkworksSerializationError,
}

#[cfg(feature = "circom")]
impl From<ark_serialize::SerializationError> for Groth16Error {
    fn from(_e: ark_serialize::SerializationError) -> Self {
        Groth16Error::ArkworksSerializationError
    }
}

impl From<Groth16Error> for u32 {
    fn from(error: Groth16Error) -> Self {
        match error {
            Groth16Error::IncompatibleVerifyingKeyWithNrPublicInputs => 0,
            Groth16Error::ProofVerificationFailed => 1,
            Groth16Error::PreparingInputsG1AdditionFailed => 2,
            Groth16Error::PreparingInputsG1MulFailed => 3,
            Groth16Error::InvalidG1Length => 4,
            Groth16Error::InvalidG2Length => 5,
            Groth16Error::InvalidPublicInputsLength => 6,
            Groth16Error::DecompressingG1Failed => 7,
            Groth16Error::DecompressingG2Failed => 8,
            Groth16Error::PublicInputGreaterThanFieldSize => 9,
            Groth16Error::ProofConversionError => 10,
            #[cfg(feature = "circom")]
            Groth16Error::ArkworksSerializationError => 11,
        }
    }
}
