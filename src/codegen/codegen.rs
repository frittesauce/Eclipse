use std::path::PathBuf;

use crate::{analyzer::IRProgram, codegen::writer::Writer, execute, CompileError};

const NAME: &str = "app";

pub fn generate(program: IRProgram, build_path: PathBuf) -> Result<PathBuf, CompileError> {
    let mut writer = Writer::new();

    writer.push_str("main:\n");
    writer.add_operation_str("push rbp");
    writer.add_operation_str("mov rbp, rsp");
    writer.add_operation_str("sub rsp, 16");
    writer.add_operation_str("mov rcx, 0");
    writer.add_operation_str("call exit");

    let assembly_file = build_path.clone().join("app.s");
    match std::fs::write(&assembly_file, writer.body) {
        Ok(()) => {}
        Err(error) => return Err(CompileError::OpenFile(error)),
    };

    match execute(format!(
        "nasm -f win64 {}",
        String::from(assembly_file.to_str().unwrap())
    )) {
        Ok(_out) => {}
        Err(error) => return Err(CompileError::NASM(error)),
    }

    let object_file = String::from(build_path.join(format!("{}.obj", &NAME)).to_str().unwrap());
    let executable = String::from(build_path.join(format!("{}", &NAME)).to_str().unwrap());

    match execute(format!("gcc -o {}.exe {} -m64", executable, object_file)) {
        Ok(_out) => {}
        Err(error) => return Err(CompileError::GCC(error)),
    }

    return Ok(build_path.join(format!("{}.exe", NAME)));
}
