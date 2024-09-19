use std::{io::Read, path::PathBuf, process::exit};

// mod analyzer;
mod builder;
mod lexer;
mod parser;

pub use builder::build;

pub const FILE_EXTENSION: &str = "eclipse";

pub fn open_file(path: &PathBuf) -> Result<std::fs::File, BuildError> {
    let file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(error) => return Err(BuildError::OpenFile(error)),
    };
    return Ok(file);
}

pub fn read_file(path: &PathBuf) -> Result<String, BuildError> {
    let mut file = open_file(path)?;

    let mut buf = String::new();

    match file.read_to_string(&mut buf) {
        Ok(_) => {}
        Err(error) => return Err(BuildError::OpenFile(error)),
    }

    return Ok(buf);
}

pub fn execute(command: String) -> Result<String, String> {
    let cmd = match std::process::Command::new("cmd")
        .args(["/C", &command])
        .output()
    {
        Ok(a) => a,
        Err(a) => return Err(a.to_string()),
    };

    if cmd.stderr.len() > 0 {
        let mut result = command.clone();
        result.push_str("\n");
        result.push_str(String::from_utf8(cmd.stderr).unwrap().as_str());
        return Err(result);
    }

    return Ok(String::from_utf8(cmd.stdout).unwrap());
}

// #[derive(Debug)]
// pub enum BuildError {
//     AlreadyDefined(String),
//     NotDefined(String),
//     NotMutable(String),
//     WrongMutableType(String),
//     ModuleNotFound,
//     NoNodeFound,
//     TooFewOrManyArguments,
//     WrongReturnType,
//     WrongType,
//     Unkown
// }

// #[derive(Debug)]
// pub enum BuildError {
//     Unkown(String),
//     Tokenize(String),
//     DuplicateModifier(TokenInfo),
//     TokensExpectedGot(Vec<Token>, TokenInfo),
//     AlreadyImported(String),
//     CannotFindModules([PathBuf; 2]),
//     ImportInBlock,
//     ExpressionExpected,
//     Peekfail,
//     NoTokenFound,
// }
// impl BuildError {
//     fn stringify(self) -> String {
//         return match self {
//             BuildError::TokensExpectedGot(expected, got) => format!(
//                 "expected: {:?} got: {:?}:{}:{}",
//                 expected, got.token, got.line, got.column
//             ),
//             token => format!("{:?}", token),
//         };
//     }
// }

// #[derive(Debug)]
// pub struct BuildProblem {
//     relative_path: PathBuf,
//     lines: Range<usize>,
//     column: usize,
//     error: BuildError,
// }
// impl BuildProblem {
//     pub fn new(error: BuildError, relative_path: PathBuf, lines: Range<usize>, column: usize) -> Self {
//         Self {
//             relative_path,
//             lines,
//             column,
//             error,
//         }
//     }
//     pub fn print(self) {
//         println!("error: {}", self.error.stringify());
//         println!(
//             "   --> {}:{}",
//             self.relative_path.to_string_lossy(),
//             self.lines.start
//         );
//         exit(1)
//     }
// }

// #[derive(Debug)]
// pub enum ParseError {
//     EarlyEndOfFile
// }

pub type ParseResult<T> = Result<T, CompileError>;

#[derive(Debug)]
pub struct CompileError {
    error: String,
    line: usize
}
impl CompileError {
    pub fn new(error: String, line: usize) -> Self {
        Self {
            line,
            error
        }
    }
    fn print(&self) {
        println!("error: {}", self.error);
        println!("line: {:?}", self.line);

        exit(1)
    }
}

pub enum BuildError {
    OpenFile(std::io::Error),
    CompileError(CompileError),
    // ParseError(ParseError),
    GCC(String),
    NASM(String),
}
impl BuildError { 
    pub fn print(self) {
        match self {
            BuildError::CompileError(problem) => problem.print(),
            BuildError::GCC(msg) => panic!("{}", msg),
            BuildError::NASM(msg) => panic!("{}", msg),
            BuildError::OpenFile(error) => panic!("{:?}", error),
        }
        exit(1);
    }
}

// macro_rules! warn {
//     () => {
//         $crate::print!("\n")
//     };
//     ($($arg:tt)*) => {{
//         $crate::io::_print($crate::format_args_nl!($($arg)*));
//     }};
// }

// macro_rules! log {
//     ($($arg:tt)*) => {{

//         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
//         res
//     }}
// }
