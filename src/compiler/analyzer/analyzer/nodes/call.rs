use crate::compiler::{
    analyzer::{analyzer::handle_expression, FunctionCtx, IRValue, Operation, ProgramCtx},
    errors::Location,
    parser::ExpressionInfo,
    path::Path,
};

pub fn handle_call(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: Location,
    path: Path,
    mut arguments: Vec<ExpressionInfo>,
) {
    let found = match program.types.get_function(function.relative_path, &path) {
        Some(f) => f,
        None => {
            program
                .debug
                .error(location, format!("Could not find function: '{path}'"));
            return;
        }
    };

    if arguments.len() != found.parameters.len() {
        program.debug.error(
            location.clone(),
            format!(
                "Expected {} arguments, but got {}",
                found.parameters.len(),
                arguments.len()
            ),
        );
        return; // Ok((IRValue::Null, expected_type.clone()));
    }

    arguments.reverse();

    let mut ir_arguments = Vec::new();
    for param_type in &found.parameters {
        let expression = arguments.pop();
        let (value, data_type) = handle_expression(program, function, param_type, expression);

        ir_arguments.push((data_type.convert(), value));
    }

    function.operations.push(Operation::Call {
        function: found.key.clone(),
        return_type: found.return_type.convert(),
        arguments: IRValue::Arguments(ir_arguments),
    });
}