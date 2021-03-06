use crate::assembler::resource::*;

#[allow(dead_code)]
const IDIVRM64: [Instruction; 1] = [Instruction {
    opcode: Opcode::IDIVRM64 {
        rm64: Operand::ADDRESSING {
            base: GeneralPurposeRegister::RAX,
            index: None,
            disp: None,
            scale: None,
        },
    },
}];

#[cfg(test)]
mod to_bytes_tests {
    use super::*;

    #[test]
    fn idivrm64_test() {
        let inst = &IDIVRM64[0];
        assert_eq!(inst.to_bytes(), vec![0x48, 0xf7, 0x38]);
    }
}
