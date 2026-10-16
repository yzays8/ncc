use std::io;
use std::path::Path;
use std::process::Command;

use clap::Parser as ClapParser;

use crate::codegen;
use crate::error::Result;
use crate::lexer;
use crate::parser::Parser;

#[derive(Debug, ClapParser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long, conflicts_with_all = ["parse", "codegen"], help = "Run the lexer, but stop before parsing")]
    pub lex: Option<String>,

    #[arg(
        long,
        conflicts_with_all = ["lex", "codegen"],
        help = "Run the lexer and parser, but stop before assembly generation"
    )]
    pub parse: Option<String>,

    #[arg(
        long,
        conflicts_with_all = ["lex", "parse"],
        help = "Perform lexing, parsing, and assembly generation, but stop before code emission"
    )]
    pub codegen: Option<String>,
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
        match (&self.args.lex, &self.args.parse, &self.args.codegen) {
            (Some(_), None, None) => {
                let source_path = Path::new(self.args.lex.as_ref().unwrap());
                let source = self.preprocess(source_path)?;
                let tokens = lexer::lex(&source)?;
                println!("{:?}", tokens);
                Ok(())
            }
            (None, Some(_), None) => {
                let source_path = Path::new(self.args.parse.as_ref().unwrap());
                let source = self.preprocess(source_path)?;
                let tokens = lexer::lex(&source)?;
                let ast = Parser::new(&tokens).parse()?;
                println!("{:?}", ast);
                Ok(())
            }
            (None, None, Some(_)) => {
                let source_path = Path::new(self.args.codegen.as_ref().unwrap());
                let source = self.preprocess(source_path)?;
                let tokens = lexer::lex(&source)?;
                let ast = Parser::new(&tokens).parse()?;
                let codegen = codegen::generate(&ast)?;
                println!("{:?}", codegen);
                Ok(())
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Must specify exactly one of --lex, --parse, or --codegen",
            )
            .into()),
        }
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
}

impl Default for Driver {
    fn default() -> Self {
        Self::new()
    }
}
