//! Common Types for zkVM Operations in Poost
use ere_dockerized::zkVMKind;
use ere_zkvm_interface::zkVM;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
#[serde(transparent)]
pub struct ProgramID(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[allow(non_camel_case_types)]
pub enum zkVMVendor {
    Airbender,
    Jolt,
    Miden,
    Nexus,
    Openvm,
    Pico,
    Risc0,
    SP1,
    Ziren,
    Zisk,
}

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct zkVMInstance {
    pub vendor: zkVMVendor,
    #[allow(dead_code)]
    pub vm: Arc<dyn zkVM + Send + Sync>,
}

impl std::fmt::Debug for zkVMInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("zkVMInstance")
            .field("vendor", &self.vendor)
            .field("vm", &"<dyn zkVM>")
            .finish()
    }
}

impl std::fmt::Display for zkVMVendor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            zkVMVendor::Airbender => write!(f, "airbender"),
            zkVMVendor::Jolt => write!(f, "jolt"),
            zkVMVendor::Miden => write!(f, "miden"),
            zkVMVendor::Nexus => write!(f, "nexus"),
            zkVMVendor::Openvm => write!(f, "openvm"),
            zkVMVendor::Pico => write!(f, "pico"),
            zkVMVendor::Risc0 => write!(f, "risc0"),
            zkVMVendor::SP1 => write!(f, "sp1"),
            zkVMVendor::Ziren => write!(f, "ziren"),
            zkVMVendor::Zisk => write!(f, "zisk"),
        }
    }
}

impl From<zkVMVendor> for zkVMKind {
    fn from(vendor: zkVMVendor) -> Self {
        match vendor {
            zkVMVendor::Airbender => zkVMKind::Airbender,
            zkVMVendor::Jolt => zkVMKind::Jolt,
            zkVMVendor::Miden => zkVMKind::Miden,
            zkVMVendor::Nexus => zkVMKind::Nexus,
            zkVMVendor::Openvm => zkVMKind::OpenVM,
            zkVMVendor::Pico => zkVMKind::Pico,
            zkVMVendor::Risc0 => zkVMKind::Risc0,
            zkVMVendor::SP1 => zkVMKind::SP1,
            zkVMVendor::Ziren => zkVMKind::Ziren,
            zkVMVendor::Zisk => zkVMKind::Zisk,
        }
    }
}

impl std::str::FromStr for zkVMVendor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "airbender" => Ok(zkVMVendor::Airbender),
            "jolt" => Ok(zkVMVendor::Jolt),
            "miden" => Ok(zkVMVendor::Miden),
            "nexus" => Ok(zkVMVendor::Nexus),
            "openvm" => Ok(zkVMVendor::Openvm),
            "pico" => Ok(zkVMVendor::Pico),
            "risc0" => Ok(zkVMVendor::Risc0),
            "sp1" => Ok(zkVMVendor::SP1),
            "ziren" => Ok(zkVMVendor::Ziren),
            "zisk" => Ok(zkVMVendor::Zisk),
            _ => Err(format!(
                "Unsupported zkVM type: {}. Supported types are: risc0, sp1",
                s
            )),
        }
    }
}

impl zkVMInstance {
    pub fn new(vendor: zkVMVendor, vm: Arc<dyn zkVM + Send + Sync>) -> Self {
        Self { vendor, vm }
    }
}

impl ProgramID {
    pub fn new(zkvm_vendor_name: String, elf_bytes: Vec<u8>) -> Result<Self, anyhow::Error> {
        let mut hasher = Sha256::new();
        hasher.update(zkvm_vendor_name.as_bytes());
        hasher.update(&elf_bytes);

        let hash_result = hasher.finalize();
        let hash_hex = hex::encode(hash_result);

        Ok(Self(hash_hex))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zkvm_type_parsing() {
        // Test all variants in enum definition order
        assert_eq!(
            "airbender".parse::<zkVMVendor>().unwrap(),
            zkVMVendor::Airbender
        );
        assert_eq!("jolt".parse::<zkVMVendor>().unwrap(), zkVMVendor::Jolt);
        assert_eq!("miden".parse::<zkVMVendor>().unwrap(), zkVMVendor::Miden);
        assert_eq!("nexus".parse::<zkVMVendor>().unwrap(), zkVMVendor::Nexus);
        assert_eq!("openvm".parse::<zkVMVendor>().unwrap(), zkVMVendor::Openvm);
        assert_eq!("pico".parse::<zkVMVendor>().unwrap(), zkVMVendor::Pico);
        assert_eq!("risc0".parse::<zkVMVendor>().unwrap(), zkVMVendor::Risc0);
        assert_eq!("sp1".parse::<zkVMVendor>().unwrap(), zkVMVendor::SP1);
        assert_eq!("ziren".parse::<zkVMVendor>().unwrap(), zkVMVendor::Ziren);
        assert_eq!("zisk".parse::<zkVMVendor>().unwrap(), zkVMVendor::Zisk);

        // Test case insensitivity
        assert_eq!("RISC0".parse::<zkVMVendor>().unwrap(), zkVMVendor::Risc0);
        assert_eq!("SP1".parse::<zkVMVendor>().unwrap(), zkVMVendor::SP1);
        assert_eq!(
            "Airbender".parse::<zkVMVendor>().unwrap(),
            zkVMVendor::Airbender
        );

        // Test invalid inputs
        assert!("invalid".parse::<zkVMVendor>().is_err());
        assert!("".parse::<zkVMVendor>().is_err());
    }
}
