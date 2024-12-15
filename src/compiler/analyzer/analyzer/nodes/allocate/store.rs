use crate::compiler::{
    analyzer::{analyzer::handle_call, FunctionCtx, IRValue, ProgramCtx},
    errors::Location,
    parser::{Expression, ExpressionInfo},
    types::Type,
};

use super::read::handle_read;

pub fn handle_store(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: &Location,
    destination: &String,
    data_type: &Type,
    info: ExpressionInfo,
) {
    match info.expression {
        Expression::Array(items) => handle_array_store(
            program,
            function,
            location,
            items,
            destination,
            data_type,
            0,
        ),
        Expression::Call(path, arguments) => {
            handle_call(program, function, Some((&destination, data_type)), location, path, arguments);
        }
        Expression::Value(_) => {
            function.operations.allocate(destination, &data_type.convert());
            let value = handle_read(program, function, location, data_type, info);
            function
                .operations
                .store(&data_type.convert(), &value, &destination);
        },
        | Expression::Minus(_)
        | Expression::Not(_)
        | Expression::Index(_, _)
        | Expression::BinaryOperation(_, _, _)
        | Expression::CompareOperation(_, _, _) => {
            let value = handle_read(program, function, location, data_type, info);
            function
                .operations
                .store(&data_type.convert(), &value, &destination);
        }
        _ => todo!(),
    }
}

pub fn handle_array_store(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: &Location,
    items: Vec<ExpressionInfo>,
    destination: &String,
    data_type: &Type,
    offset: usize,
) {
    let (item_type, _) = data_type.array_info();
    let item_size = item_type.bytes();

    if item_type.base.is_array() {
        for (index, item) in items.into_iter().enumerate() {
            let items = match item.expression {
                Expression::Array(items) => items,
                _ => panic!("Expected array"),
            };

            handle_array_store(
                program,
                function,
                location,
                items,
                destination,
                item_type,
                offset + index * item_size,
            )
        }
        return;
    }

    for (index, item) in items.into_iter().enumerate() {
        let key_ptr = function.variables.increment();

        function.operations.getelementptr_inbounds(
            &key_ptr,
            &data_type.convert(),
            destination,
            &IRValue::IntLiteral(format!("{}", index * item_size + offset)),
        );

         // handle_store(program, function, location, &key_ptr, &item_type, item);
         
        let value = handle_read(program, function, location, item_type, item);
        function
            .operations
            .store(&item_type.convert(), &value, &key_ptr);
    }
}
