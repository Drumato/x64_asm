#[cfg(test)]
mod to_intel_tests {
    use crate::*;

    #[test]
    fn movrm8r8_test() {
        // mov bh, ah
        let inst = Instruction {
            opcode: Opcode::MOVRM8R8 {
                rm8: Operand::GENERALREGISTER(GeneralPurposeRegister::BH),
                r8: Operand::GENERALREGISTER(GeneralPurposeRegister::AH),
            }
        };

        assert_eq!(inst.to_intel_string(), "mov bh, ah");

        // mov BYTE PTR [rax], bh
        let inst = Instruction {
            opcode: Opcode::MOVRM8R8 {
                rm8: Operand::ADDRESSING {
                    base_reg: GeneralPurposeRegister::AL,
                    index_reg: None,
                    displacement: None,
                    scale: None,
                },
                r8: Operand::GENERALREGISTER(GeneralPurposeRegister::BH),
            }
        };

        assert_eq!(inst.to_intel_string(), "mov BYTE PTR [rax], bh");
    }

    #[test]
    fn movrm64r64_test() {
        // mov rax, rcx
        let inst = Instruction {
            opcode: Opcode::MOVRM64R64 {
                rm64: Operand::GENERALREGISTER(GeneralPurposeRegister::RAX),
                r64: Operand::GENERALREGISTER(GeneralPurposeRegister::RCX),
            }
        };

        assert_eq!(inst.to_intel_string(), "mov rax, rcx");

        // mov [rax + rbx * 4], rcx
        let inst = Instruction {
            opcode: Opcode::MOVRM64R64 {
                rm64: Operand::ADDRESSING {
                    base_reg: GeneralPurposeRegister::RAX,
                    index_reg: Some(GeneralPurposeRegister::RBX),
                    displacement: None,
                    scale: Some(0x4),
                },
                r64: Operand::GENERALREGISTER(GeneralPurposeRegister::RCX),
            }
        };

        assert_eq!(inst.to_intel_string(), "mov QWORD PTR [rax + rbx * 4], rcx");
    }

    #[test]
    fn movrm64imm32_test() {
        // mov QWORD PTR [rax], 60
        let inst = Instruction {
            opcode: Opcode::MOVRM64IMM32 {
                rm64: Operand::ADDRESSING {
                    base_reg: GeneralPurposeRegister::RAX,
                    index_reg: None,
                    displacement: None,
                    scale: None,
                },
                imm: Immediate::I32(60),
            }
        };
        assert_eq!(inst.to_intel_string(), "mov QWORD PTR [rax], 60")
    }
}

#[cfg(test)]
mod to_at_tests {
    use crate::*;

    #[test]
    fn movrm8r8_test() {
        // mov bh, ah
        let inst = Instruction {
            opcode: Opcode::MOVRM8R8 {
                rm8: Operand::GENERALREGISTER(GeneralPurposeRegister::BH),
                r8: Operand::GENERALREGISTER(GeneralPurposeRegister::AH),
            }
        };

        assert_eq!(inst.to_at_string(), "movb %ah, %bh");

        // mov BYTE PTR [rax], bh
        let inst = Instruction {
            opcode: Opcode::MOVRM8R8 {
                rm8: Operand::ADDRESSING {
                    base_reg: GeneralPurposeRegister::AL,
                    index_reg: None,
                    displacement: None,
                    scale: None,
                },
                r8: Operand::GENERALREGISTER(GeneralPurposeRegister::BH),
            }
        };

        assert_eq!(inst.to_at_string(), "movb %bh, (%rax)");
    }

    #[test]
    fn movrm64r64_test() {
        // mov rax, rcx
        let inst = Instruction {
            opcode: Opcode::MOVRM64R64 {
                rm64: Operand::GENERALREGISTER(GeneralPurposeRegister::RAX),
                r64: Operand::GENERALREGISTER(GeneralPurposeRegister::RCX),
            }
        };

        assert_eq!(inst.to_at_string(), "movq %rcx, %rax");

        // mov [rax + rbx * 4], rcx
        let inst = Instruction {
            opcode: Opcode::MOVRM64R64 {
                rm64: Operand::ADDRESSING {
                    base_reg: GeneralPurposeRegister::RAX,
                    index_reg: Some(GeneralPurposeRegister::RBX),
                    displacement: None,
                    scale: Some(0x4),
                },
                r64: Operand::GENERALREGISTER(GeneralPurposeRegister::RCX),
            }
        };

        assert_eq!(inst.to_at_string(), "movq %rcx, (%rax, %rbx, 4)");
    }

    #[test]
    fn movrm64imm32_test() {
        // mov QWORD PTR [rax], 60
        let inst = Instruction {
            opcode: Opcode::MOVRM64IMM32 {
                rm64: Operand::ADDRESSING {
                    base_reg: GeneralPurposeRegister::RAX,
                    index_reg: None,
                    displacement: None,
                    scale: None,
                },
                imm: Immediate::I32(60),
            }
        };
        assert_eq!(inst.to_at_string(), "movq $60, (%rax)")
    }
}

#[cfg(test)]
mod to_bytes_tests {
    use crate::*;

    #[test]
    fn movrm8r8_test() {
        // mov bh, ah
        let inst = Instruction {
            opcode: Opcode::MOVRM8R8 {
                rm8: Operand::GENERALREGISTER(GeneralPurposeRegister::BH),
                r8: Operand::GENERALREGISTER(GeneralPurposeRegister::AH),
            }
        };

        assert_eq!(inst.to_bytes(), vec![0x88, 0xe7]);

        // mov BYTE PTR [rax], bh
        let inst = Instruction {
            opcode: Opcode::MOVRM8R8 {
                rm8: Operand::ADDRESSING {
                    base_reg: GeneralPurposeRegister::AL,
                    index_reg: None,
                    displacement: None,
                    scale: None,
                },
                r8: Operand::GENERALREGISTER(GeneralPurposeRegister::BH),
            }
        };

        assert_eq!(inst.to_bytes(), vec![0x88, 0x38]);
    }

    #[test]
    fn movrm64r64_test() {
        // mov rax, rcx
        let inst = Instruction {
            opcode: Opcode::MOVRM64R64 {
                rm64: Operand::GENERALREGISTER(GeneralPurposeRegister::RAX),
                r64: Operand::GENERALREGISTER(GeneralPurposeRegister::RCX),
            }
        };

        assert_eq!(inst.to_bytes(), vec![0x48, 0x89, 0xc8]);

        // mov [rax + rbx * 4], rcx
        let inst = Instruction {
            opcode: Opcode::MOVRM64R64 {
                rm64: Operand::ADDRESSING {
                    base_reg: GeneralPurposeRegister::RAX,
                    index_reg: Some(GeneralPurposeRegister::RBX),
                    displacement: None,
                    scale: Some(0x4),
                },
                r64: Operand::GENERALREGISTER(GeneralPurposeRegister::RCX),
            }
        };

        assert_eq!(inst.to_bytes(), vec![0x48, 0x89, 0x0c, 0x98]);
    }

    #[test]
    fn movrm64imm32_test() {
        // mov QWORD PTR [rax], 60
        let inst = Instruction {
            opcode: Opcode::MOVRM64IMM32 {
                rm64: Operand::ADDRESSING {
                    base_reg: GeneralPurposeRegister::RAX,
                    index_reg: None,
                    displacement: None,
                    scale: None,
                },
                imm: Immediate::I32(60),
            }
        };
        assert_eq!(inst.to_bytes(), vec![0x48, 0xc7, 0x00, 0x3c, 0x00, 0x00, 0x00])
    }
}