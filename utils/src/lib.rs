struct Generic<T> {
    field1: T,
    field2: T,
}

fn lookup<T: PartialEq>(orig_struct: Vec<Generic<T>>, field: Option<T>) -> Vec<&Generic<T>> {
    let result: Vec<&Generic<T>> = if let Some(target_field) = field {
        orig_struct
            .iter()
            .filter(|o| o.field1 == target_field)
            .collect()
    } else {
        orig_struct.iter().collect()
    };

    result
}
