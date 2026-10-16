use crate::codegen;
use crate::error::Result;

#[derive(Debug)]
pub struct Emitter {
    asm: String,
}

impl Emitter {
    pub fn new() -> Self {
        Self { asm: String::new() }
    }

    pub fn emit(&mut self, asm_ast: codegen::Program) -> Result<String> {
        self.emit_program(&asm_ast);
        Ok(self.asm.clone())
    }

    fn emit_program(&mut self, ast: &codegen::Program) {
        match ast {
            codegen::Program::Main(func) => self.emit_function(func),
        }
        self.asm
            .push_str(".section .note.GNU-stack,\"\",@progbits\n");
    }

    fn emit_function(&mut self, func: &codegen::Function) {
        self.asm
            .push_str(format!(".globl {}\n", func.name).as_str());
        self.asm.push_str(format!("{}:\n", func.name).as_str());
        for instr in &func.body {
            self.emit_instruction(instr);
        }
    }

    fn emit_instruction(&mut self, instr: &codegen::Instruction) {
        match instr {
            codegen::Instruction::Mov { src, dest } => {
                let src_str = self.emit_operand(src);
                let dest_str = self.emit_operand(dest);
                self.asm
                    .push_str(format!("    mov {}, {}\n", src_str, dest_str).as_str());
            }
            codegen::Instruction::Ret => {
                self.asm.push_str("    ret\n");
            }
        }
    }

    fn emit_operand(&mut self, op: &codegen::Operand) -> String {
        match op {
            codegen::Operand::Register => "%eax".to_string(),
            codegen::Operand::Immediate(val) => format!("${}", val),
        }
    }
}

impl Default for Emitter {
    fn default() -> Self {
        Self::new()
    }
}
