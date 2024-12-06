use std::collections::HashMap;

use crate::compiler::{
    counter::NameCounter,
    errors::{CompileCtx, CompileResult, Location},
    parser::{Node, ParsedFile},
    path::Path,
    program::ParsedProgram,
    types::{BaseType, Type},
};

#[derive(Debug)]
pub struct CustomEnum {
    pub fields: Vec<String>,
}

#[derive(Debug)]
pub struct CustomStruct {}

#[derive(Debug)]
pub enum CustomTypes {
    Enum(CustomEnum),
    Struct(CustomStruct),
}

#[derive(Debug)]
pub struct FileTypes {
    functions: HashMap<String, Function>,
    types: HashMap<String, CustomTypes>,
    imports: HashMap<String, FileTypes>,
    is_module: bool,
    // pub types: HashMap<String, Type>
    // export: bool,
}
impl FileTypes {
    pub fn get_function(&self, relative_path: &Path, static_path: &Path) -> Option<&Function> {
        let mut components = static_path.components();
        let name = components.pop().unwrap();

        let mut new_path = relative_path.clone();
        while components.len() > 0 {
            let key = components.pop().unwrap();
            match &key[..] {
                "root" => new_path.clear(),
                "super" => {
                    new_path.pop();
                }
                _ => new_path.push(key),
            }
        }

        let file = {
            let mut path_components = new_path.components();
            path_components.reverse();
            path_components.pop();

            let mut file = self;
            while path_components.len() > 0 {
                let key = path_components.pop().unwrap();
                let f = match file.imports.get(&key) {
                    Some(f) => f,
                    None => return None,
                };
                if f.is_module {
                    file = match f.imports.get(&key) {
                        Some(f) => f,
                        None => f,
                    }
                } else {
                    file = f;
                }
            }
            file
        };

        return file.functions.get(&name);
    }
}

#[derive(Debug)]
pub struct Function {
    pub key: String,
    pub parameters: Vec<Type>,
    pub return_type: Type,
}

pub fn parse_types(
    debug: &mut CompileCtx,
    count: &mut NameCounter,
    program: &ParsedProgram,
) -> CompileResult<FileTypes> {
    let mut main = handle_file(debug, count, &program.main)?;

    let mut src = FileTypes {
        imports: HashMap::new(),
        functions: HashMap::new(),
        is_module: true,
        types: HashMap::new(), // export: true
    };

    main.functions.insert(
        "print".to_string(),
        Function {
            key: "print".to_string(),
            parameters: vec![(Type::new(BaseType::Int(32)))],
            return_type: Type::void(),
        },
    );

    main.functions.insert(
        "sleep".to_string(),
        Function {
            key: "sleep".to_string(),
            parameters: vec![(Type::new(BaseType::Int(32)))],
            return_type: Type::new(BaseType::Int(32)),
        },
    );

    main.functions.insert(
        "usleep".to_string(),
        Function {
            key: "usleep".to_string(),
            parameters: vec![(Type::new(BaseType::Int(32)))],
            return_type: Type::new(BaseType::Int(32)),
        },
    );

    src.imports.insert(String::from("main"), main);

    return Ok(src);
}

fn handle_file(
    debug: &mut CompileCtx,
    count: &mut NameCounter,
    file: &ParsedFile,
) -> CompileResult<FileTypes> {
    let mut types = FileTypes {
        imports: HashMap::new(),
        functions: HashMap::new(),
        is_module: file.is_module,
        types: HashMap::new(),
    };

    for (name, import) in &file.imports {
        let file = handle_file(debug, count, import)?;
        if types.imports.insert(name.clone(), file).is_some() {
            debug.error(Location::void(), format!("'{}' is already imported", name));
        };
    }

    for info in &file.body {
        match &info.node {
            Node::Enum { name, fields } => {
                let custom_enum = CustomEnum {
                    fields: fields.clone(),
                };

                types
                    .types
                    .insert(name.clone(), CustomTypes::Enum(custom_enum));
            }
            Node::Function {
                export: _,
                name,
                key,
                parameters,
                return_type,
                body: _,
            } => {
                let is_main_function =
                    file.relative_file_path == Path::from("src").join("main") && name.eq("main");
                let key = if is_main_function {
                    String::from("main")
                } else {
                    key.clone()
                };

                types.functions.insert(
                    name.clone(),
                    Function {
                        key,
                        parameters: parameters
                            .iter()
                            .map(|parameter| parameter.data_type.clone())
                            .collect::<Vec<Type>>(),
                        return_type: return_type.clone(),
                    },
                );
            }
            _ => continue,
        }
    }

    return Ok(types);
}
