//! # Diffie-Hellman (DH) based Key Encapsulation Mechanisms (KEM)
//!
//! This crate provides a KEM interface for DH protocols as specified in
//! [RFC9180](https://datatracker.ietf.org/doc/html/rfc9180#name-dh-based-kem-dhkem)
//! without the shared secret extraction process. In particular, `Encaps(pk)` in the
//! RFC returns the encapsulated key and an extracted shared secret, while our
//! implementation leaves the extraction process up to the user. This type of KEM
//! construction is currently being used in HPKE, as per the RFC, and in the current
//! draft of the [TLS KEM
//! combiner](https://datatracker.ietf.org/doc/html/draft-ietf-tls-hybrid-design-10).

use kem::{Decapsulate, Encapsulate};
use rand_core::CryptoRngCore;

/// Newtype for a piece of data that may be encapsulated
pub struct Encapsulator<X>(X);
/// Newtype for a piece of data that may be decapsulated
pub struct Decapsulator<X>(X);

#[cfg(test)]
pub trait SecretBytes {
    fn as_slice(&self) -> &[u8];
}

/// This is a trait that all KEM models should implement, and should probably be
/// promoted to the kem crate itself. It specifies the types of encapsulating and
/// decapsulating keys created by key generation, the shared secret type, and the
/// encapsulated key type
pub trait DhKem {
    /// The type that will implement [`Decapsulate`]
    type DecapsulatingKey: Decapsulate<Self::EncapsulatedKey, Self::SharedSecret>;

    /// The type that will implement [`Encapsulate`]
    type EncapsulatingKey: Encapsulate<Self::EncapsulatedKey, Self::SharedSecret>;

    /// The type of the encapsulated key
    type EncapsulatedKey;

    #[cfg(not(test))]
    /// The type of the shared secret
    type SharedSecret;

    #[cfg(test)]
    type SharedSecret: SecretBytes;

    /// Generates a new (decapsulating key, encapsulating key) keypair for the KEM
    /// model
    fn random_keypair(
        rng: &mut impl CryptoRngCore,
    ) -> (Self::DecapsulatingKey, Self::EncapsulatingKey);
}

#[cfg(feature = "arithmetic")]
pub mod arithmetic;

#[cfg(feature = "x25519")]
mod x25519_kem;
#[cfg(feature = "x25519")]
pub use x25519_kem::X25519;

#[cfg(feature = "bign256")]
pub type BignP256 = arithmetic::ArithmeticKem<bign256::BignP256>;
#[cfg(feature = "k256")]
pub type Secp256k1 = arithmetic::ArithmeticKem<k256::Secp256k1>;
#[cfg(feature = "p192")]
pub type NistP192 = arithmetic::ArithmeticKem<p192::NistP192>;
#[cfg(feature = "p224")]
pub type NistP224 = arithmetic::ArithmeticKem<p224::NistP224>;
#[cfg(feature = "p256")]
pub type NistP256 = arithmetic::ArithmeticKem<p256::NistP256>;
// include an additional alias Secp256r1 = NistP256
#[cfg(feature = "p256")]
pub type Secp256r1 = arithmetic::ArithmeticKem<p256::NistP256>;
#[cfg(feature = "p384")]
pub type NistP384 = arithmetic::ArithmeticKem<p384::NistP384>;
#[cfg(feature = "p521")]
pub type NistP521 = arithmetic::ArithmeticKem<p521::NistP521>;
#[cfg(feature = "sm2")]
pub type Sm2 = arithmetic::ArithmeticKem<sm2::Sm2>;

#[cfg(test)]
mod tests;
