use std::iter::FromIterator;

#[derive(FromIterator)]
struct Generic<T> {
    field1: T,
    field2: T,
}

fn lookup<T: PartialEq>(
    orig_struct: Vec<Generic<T>>,
    target_stuct: Vec<Generic<T>>,
    field: Option<T>,
) {
    let result: Vec<Generic<T>> = if let Some(target_field) = field {
        orig_struct
            .iter()
            .filter(|o| o.field1 == target_field)
            .collect()
    } else {
        orig_struct.collect();
    };
}
