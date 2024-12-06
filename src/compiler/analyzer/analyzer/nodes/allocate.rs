use crate::compiler::{
    analyzer::{FunctionCtx, IRValue, ProgramCtx},
    errors::Location,
    parser::{Expression, ExpressionInfo, Value},
    types::Type,
};

pub fn handle_allocation(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: &Location,
    destination: &String,
    data_type: &Type,
    info: ExpressionInfo,
) {
    function
        .operations
        .allocate(&destination, &data_type.convert());

    handle_store(program, function, location, destination, data_type, info);
}

pub fn handle_store(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: &Location,
    destination: &String,
    data_type: &Type,
    info: ExpressionInfo,
) {
    return match info.expression {
        Expression::Value(value) => {
            let value = match value {
                Value::Integer(int) => IRValue::IntLiteral(int),
                Value::Boolean(bool) => IRValue::BoolLiteral(bool),
                Value::Float(float) => IRValue::FloatLiteral(float),
                _ => todo!(),
            };

            function
                .operations
                .store(&data_type.convert(), &value, &destination);
        }
        Expression::GetVariable(_) => {
            let value = handle_read(program, function, location, &data_type, info);
            function
                .operations
                .store(&data_type.convert(), &value, &destination);
        }
        Expression::Array(items) => {
            let (item_type, size) = data_type.array_info();

            for (index, item) in items.into_iter().enumerate() {
                let key_ptr = function.variables.increment();

                function.operations.getelementptr_inbounds(
                    &key_ptr,
                    &data_type.convert(),
                    &item_type.convert(),
                    destination,
                    &IRValue::IntLiteral(format!("{index}")),
                );
                handle_store(program, function, location, &key_ptr, &item_type, item);
            }
        }
        Expression::Index(_, _) | Expression::Call(_, _) => {
            let value = handle_read(program, function, location, data_type, info);
            function
                .operations
                .store(&data_type.convert(), &value, &destination);
        }
        _ => todo!(),
    };
}

pub fn handle_read(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: &Location,
    data_type: &Type,
    info: ExpressionInfo,
) -> IRValue {
    return match info.expression {
        Expression::Index(path, index) => {
            let name = path.components().pop().unwrap();
            let info = *index;

            let array_value_ptr = function.variables.increment();
            let result_ptr = function.variables.increment();

            let array = match function.variables.read(&name) {
                Some(var) => var.clone(),
                None => {
                    program.debug.error(
                        info.location.clone(),
                        format!("Could not find variable named: '{name}'"),
                    );
                    return IRValue::Null;
                }
            };

            let (inner_type, _) = array.data_type.clone().array_info();
            let value = handle_read(program, function, location, data_type, info);

            function.operations.getelementptr_inbounds(
                &array_value_ptr.clone(),
                &array.data_type.convert(),
                &inner_type.convert(),
                &array.key,
                &value,
            );
            function.operations.load(
                &result_ptr,
                &inner_type.convert(),
                &IRValue::Variable(array_value_ptr),
            );

            IRValue::Variable(result_ptr)
        }
        Expression::GetVariable(path) => {
            let name = path.first().unwrap();
            let load_destination = function.variables.increment();

            let variable = match function.variables.read(name) {
                Some(var) => var,
                None => {
                    program.debug.error(
                        location.clone(),
                        format!("Could not find variable named: '{name}'"),
                    );
                    return IRValue::Null;
                }
            };

            function.operations.load(
                &load_destination,
                &data_type.convert(),
                &IRValue::Variable(variable.key.clone()),
            );

            IRValue::Variable(load_destination)
        }
        Expression::Value(value) => match value {
            Value::Integer(int) => IRValue::IntLiteral(int),
            Value::Boolean(bool) => IRValue::BoolLiteral(bool),
            Value::Float(float) => IRValue::FloatLiteral(float),
            _ => todo!(),
        },
        Expression::Call(path, mut arguments) => {
            let result_key = function.variables.increment();
            let found = match program.types.get_function(function.relative_path, &path) {
                Some(f) => f,
                None => {
                    program.debug.error(
                        location.clone(),
                        format!("Could not find function: '{path}'"),
                    );
                    return IRValue::Null;
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
                return IRValue::Null;
            }

            arguments.reverse();

            let mut ir_arguments = Vec::new();
            for param_type in &found.parameters {
                let expression = arguments.pop().unwrap();
                let value = handle_read(
                    program,
                    function,
                    location,
                    param_type.as_ref().unwrap(),
                    expression,
                );

                ir_arguments.push((data_type.convert(), value));
            }

            function.operations.store_call(
                &result_key,
                &found.key,
                &found.return_type.convert(),
                IRValue::Arguments(ir_arguments),
            );

            IRValue::Variable(result_key)
        }

        // Expression::Value(value) if data_type.base.is_integer() => ,
        // Expression::Value(_) => {
        //     program.debug.error(
        //         info.location.clone(),
        //         format!("An integer is required to index in an array"),
        //     );
        //     IRValue::Null
        // }
        _ => todo!(),
    };
    /*
        let name = path.components().pop().unwrap();
    let key_ptr = function.variables.increment();

    let operation = ElemmentPointerOperation::Inbounds {
        data_type: data_type.convert(),
        value_type: inner_type.convert(),
        from: destination.clone(),
        index,
    };

    function.operations.push(Operation::GetElementPointer {
        destination: key_ptr.clone(),
        operation,
    }); */
}