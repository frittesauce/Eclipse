use crate::compiler::{
    analyzer::{analyzer::what_type, FunctionCtx, ProgramCtx},
    errors::Location,
    parser::ExpressionInfo,
    types::Type,
};

use super::{handle_read, handle_store};

pub fn handle_return(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: Location,
    return_type: &Option<Type>,
    expression: Option<ExpressionInfo>,
) {
    let expression = match expression {
        Some(e) => e,
        None => {
            function.operations.void_return();
            return;
        }
    };
    let data_type = match return_type {
        Some(d) => what_type(program, function, &expression.location, Some(&d), &expression),
        None => Type::void(),
    };

    if data_type.base.is_basic() {
        let value = handle_read(program, function, &location, &data_type, expression);
        function.operations.r#return(&data_type.convert(), &value);
    } else {
        handle_store(program, function, &location,& "0".to_string(), &data_type, expression);
        function.operations.void_return();
    }
}
