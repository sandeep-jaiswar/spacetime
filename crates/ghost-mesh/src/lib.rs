use domain::{CoreError, Result};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgentRole {
    Architect,
    Coder,
    QA,
    Security,
    Lead,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PassportPayload {
    pub agent_id: String,
    pub role: AgentRole,
    pub issued_at: u64,
    pub expires_at: u64,
}

impl PassportPayload {
    pub fn new(agent_id: impl Into<String>, role: AgentRole, ttl_seconds: u64) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Self {
            agent_id: agent_id.into(),
            role,
            issued_at: now,
            expires_at: now + ttl_seconds,
        }
    }

    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now > self.expires_at
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Passport {
    pub payload: PassportPayload,
    pub signature: Vec<u8>,
}

impl Passport {
    /// Serialize the payload to bytes using bincode for deterministic signing
    pub fn serialize_payload(&self) -> Result<Vec<u8>> {
        bincode::serialize(&self.payload)
            .map_err(|e| CoreError::AuthError(format!("Failed to serialize payload: {}", e)))
    }
}

pub struct PassportAuthority {
    signing_key: SigningKey,
}

impl Default for PassportAuthority {
    fn default() -> Self {
        Self::new()
    }
}

impl PassportAuthority {
    /// Initialize a new authority with a securely generated random keypair
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        Self { signing_key }
    }

    /// Retrieve the public verifying key to distribute to the mesh
    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    /// Issue a new cryptographically signed passport
    pub fn issue(&self, payload: PassportPayload) -> Result<Passport> {
        let mut passport = Passport {
            payload,
            signature: vec![],
        };

        let bytes = passport.serialize_payload()?;
        let signature = self.signing_key.sign(&bytes);
        passport.signature = signature.to_bytes().to_vec();

        Ok(passport)
    }

    /// Verify an arbitrary passport using a provided verifying key
    pub fn verify(passport: &Passport, public_key: &VerifyingKey) -> Result<()> {
        if passport.payload.is_expired() {
            return Err(CoreError::AuthError("Passport has expired".to_string()));
        }

        let bytes = passport.serialize_payload()?;
        let sig_bytes: [u8; 64] = passport
            .signature
            .as_slice()
            .try_into()
            .map_err(|_| CoreError::AuthError("Invalid signature length".to_string()))?;

        let signature = Signature::from_bytes(&sig_bytes);

        public_key
            .verify(&bytes, &signature)
            .map_err(|_| CoreError::AuthError("Signature verification failed".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_issue_and_verify_passport() {
        let authority = PassportAuthority::new();
        let verifying_key = authority.verifying_key();

        let payload = PassportPayload::new("agent-007", AgentRole::Lead, 3600);
        let passport = authority.issue(payload).expect("Failed to issue passport");

        // Verification should succeed
        assert!(PassportAuthority::verify(&passport, &verifying_key).is_ok());
    }

    #[test]
    fn test_expired_passport() {
        let authority = PassportAuthority::new();
        let verifying_key = authority.verifying_key();

        let mut payload = PassportPayload::new("agent-007", AgentRole::Coder, 10);
        // Manually expire the payload
        payload.expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - 100;

        let passport = authority.issue(payload).unwrap();

        // Verification should fail because it's expired
        let result = PassportAuthority::verify(&passport, &verifying_key);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Authentication Failed: Passport has expired"
        );
    }

    #[test]
    fn test_tampered_passport() {
        let authority = PassportAuthority::new();
        let verifying_key = authority.verifying_key();

        let payload = PassportPayload::new("agent-007", AgentRole::Architect, 3600);
        let mut passport = authority.issue(payload).unwrap();

        // Tamper with the passport payload
        passport.payload.role = AgentRole::Lead;

        // Verification should fail
        let result = PassportAuthority::verify(&passport, &verifying_key);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Authentication Failed: Signature verification failed"
        );
    }
}
