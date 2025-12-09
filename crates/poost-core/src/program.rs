use std::{fs, path::Path, str::FromStr, sync::Arc};
use ere_dockerized::{CompilerKind, DockerizedCompiler, DockerizedzkVM};
use serde::{Deserialize, Serialize};
use ere_zkvm_interface::{Input, ProverResourceType, compiler::Compiler};
use crate::common::{ProgramID, zkVMInstance, zkVMVendor};



#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(transparent)]
pub struct ProgramInput {
    pub input: Vec<u8>,
}

#[derive(Debug)]
pub struct ProgramInstance {
    pub name: String,
    pub program_id: ProgramID,
    pub zkvm_instance: zkVMInstance
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProgramInstanceResponse {
    pub name: String,
    pub program_id: ProgramID,
    pub zkvm_instance: zkVMVendor,
}

impl From<ProgramInput> for Input {
    fn from(value: ProgramInput) -> Self {
        let input = Input::new();
        input.with_prefixed_stdin(value.input)
    }
}

impl From<&ProgramInstance> for ProgramInstanceResponse {
    fn from(value: &ProgramInstance) -> Self {
        ProgramInstanceResponse {
            name: value.name.clone(),
            program_id: value.program_id.clone(),
            zkvm_instance: value.zkvm_instance.vendor.clone(),
        }
    }
}

impl ProgramInstance {
    /// This function compiles the program and create an instance of the program.
    /// TODO: allow to load already compiled program stored on disk
    pub fn new(name: String, program_path: impl AsRef<Path>, zkvm_vendor_name: String) -> Result<Self, anyhow::Error> {
        let zkvm_vendor = zkVMVendor::from_str(&zkvm_vendor_name).map_err(|e| anyhow::anyhow!("Failed to parse zkVM vendor name: {}", e))?;
        let zkvm_kind = zkvm_vendor.into();
        let compiler =
                DockerizedCompiler::new(zkvm_kind, CompilerKind::RustCustomized, &program_path)?;
        let program = compiler.compile(&program_path.as_ref())?;
        
        let program_vec = serde_json::to_vec(&program)?;
        let zkvm = DockerizedzkVM::new(zkvm_kind, program, ProverResourceType::Cpu)?;
        let zkvm_instance = zkVMInstance::new(zkvm_vendor, Arc::new(zkvm));
        let program_id = ProgramID::new(zkvm_vendor_name, program_vec)?;

        
        Ok(ProgramInstance {
            name,
            program_id,
            zkvm_instance,
        })
    }
}

pub fn load_elf_from_disk(program_path: impl AsRef<Path>) -> Result<Vec<u8>, anyhow::Error> {
    let path = program_path.as_ref();
    let elf_bytes = fs::read(path)
            .map_err(|e| format!("Failed to read SP1 ELF from '{}': {}", path.display(), e)).map_err(anyhow::Error::msg)?;
    
    Ok(elf_bytes)
}

