use crate::error::Result;
use crate::parser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Program {
    Main(Function),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    pub body: Vec<Instruction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Mov { src: Operand, dest: Operand },
    Ret,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    Immediate(i128),
    Register,
}

pub fn generate(ast: &parser::Program) -> Result<Program> {
    generate_program(ast)
}

fn generate_program(ast: &parser::Program) -> Result<Program> {
    match ast {
        parser::Program::Main(func) => {
            let main_func = generate_function(func)?;
            Ok(Program::Main(main_func))
        }
    }
}

fn generate_function(func: &parser::Function) -> Result<Function> {
    let mut body = Vec::new();
    for stmt in &func.body {
        let instr = generate_statement(stmt)?;
        body.extend(instr);
    }
    Ok(Function {
        name: func.name.clone(),
        body,
    })
}

fn generate_statement(stmt: &parser::Statement) -> Result<Vec<Instruction>> {
    match stmt {
        parser::Statement::Return(expr) => {
            let expr = generate_expr(expr)?;
            let instructions = vec![
                Instruction::Mov {
                    src: expr,
                    dest: Operand::Register,
                },
                Instruction::Ret,
            ];
            Ok(instructions)
        }
    }
}

fn generate_expr(expr: &parser::Expression) -> Result<Operand> {
    match expr {
        parser::Expression::Int(value) => Ok(Operand::Immediate(*value)),
    }
}
