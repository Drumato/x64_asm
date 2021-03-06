use crate::assembler::resource::Opcode;

/// An implementation of x64 instruction.
#[allow(dead_code)]
#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
}

#[allow(dead_code)]
impl Instruction {
    // assembling for each instructions.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut codes = Vec::new();

        if let Some(rex_prefix) = self.opcode.rex_prefix() {
            codes.push(rex_prefix.to_byte());
        }

        codes.append(&mut self.opcode.to_bytes());

        if let Some(modrm) = self.opcode.modrm() {
            codes.push(modrm.to_byte());
        }

        if let Some(sib_byte) = self.opcode.sib_bite() {
            codes.push(sib_byte.to_byte());
        }

        if let Some(disp) = self.opcode.get_displacement() {
            codes.append(&mut disp.to_bytes());
        }

        if let Some(imm) = self.opcode.get_immediate() {
            codes.append(&mut imm.to_bytes());
        }
        codes
    }
}
