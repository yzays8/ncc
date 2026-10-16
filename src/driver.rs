use std::io;
use std::path::Path;
use std::process::Command;

use clap::Parser as ClapParser;

use crate::codegen;
use crate::emit::Emitter;
use crate::error::Result;
use crate::lexer;
use crate::parser::Parser;

#[derive(Debug, ClapParser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(help = "Path to the source file")]
    pub source: String,

    #[arg(long, conflicts_with_all = ["parse", "codegen"], help = "Run the lexer, but stop before parsing")]
    pub lex: bool,

    #[arg(
        long,
        conflicts_with_all = ["lex", "codegen"],
        help = "Run the lexer and parser, but stop before assembly generation"
    )]
    pub parse: bool,

    #[arg(
        long,
        conflicts_with_all = ["lex", "parse"],
        help = "Perform lexing, parsing, and assembly generation, but stop before code emission"
    )]
    pub codegen: bool,
}

#[derive(Debug)]
pub struct Driver {
    args: Args,
}

impl Driver {
    pub fn new() -> Self {
        Self {
            args: Args::parse(),
        }
    }

    pub fn run(&self) -> Result<()> {
        let source_path = Path::new(&self.args.source);
        let source = self.preprocess(source_path)?;

        let tokens = lexer::lex(&source)?;
        if self.args.lex {
            println!("{:?}", tokens);
            return Ok(());
        }

        let ast = Parser::new(&tokens).parse()?;
        if self.args.parse {
            println!("{:?}", ast);
            return Ok(());
        }

        let asm_ast = codegen::generate(&ast)?;
        if self.args.codegen {
            println!("{:?}", asm_ast);
            return Ok(());
        }

        let mut emitter = Emitter::new();
        self.compile(source_path, &emitter.emit(asm_ast)?)?;

        Ok(())
    }

    fn preprocess(&self, path: &Path) -> Result<String> {
        let output = Command::new("gcc").arg("-E").arg("-P").arg(path).output()?;

        if !output.status.success() {
            return Err(
                io::Error::other(String::from_utf8_lossy(&output.stderr).into_owned()).into(),
            );
        }

        String::from_utf8(output.stdout).map_err(|e| io::Error::other(e).into())
    }

    fn compile(&self, source_path: &Path, asm: &str) -> Result<()> {
        let asm_path = source_path.with_extension("s");
        let exe_path = source_path.with_extension("");

        std::fs::write(&asm_path, asm)?;

        let status = Command::new("gcc")
            .arg(&asm_path)
            .arg("-o")
            .arg(&exe_path)
            .status()?;
        if !status.success() {
            return Err(io::Error::other("Failed to compile").into());
        }

        std::fs::remove_file(asm_path)?;

        Ok(())
    }
}

impl Default for Driver {
    fn default() -> Self {
        Self::new()
    }
}
