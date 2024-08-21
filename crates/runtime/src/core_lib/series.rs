//! The `list` core library module

use super::{
    iterator::collect_pair,
    value_sort::{sort_by_key, sort_values},
};
use crate::prelude::*;
use std::{cmp::Ordering, ops::DerefMut};

/// Initializes the `list` core library module
pub fn make_module() -> KMap {
    let result = KMap::with_type("core.series");
    result.add_fn("get", |ctx| {
        let (list, index, default) = {
            let expected_error = "|List, Number|, or |List, Number, Any|";

            match ctx.instance_and_args(is_series, expected_error)? {
                (KValue::Series(list), [KValue::Number(n)]) => (list, n, &KValue::Null),
                (KValue::Series(list), [KValue::Number(n), default]) => (list, n, default),
                (instance, args) => {
                    return unexpected_args_after_instance(expected_error, instance, args)
                }
            }
        };

        match list.data().get::<usize>(index.into()) {
            Some(value) => Ok(value.clone()),
            None => Ok(default.clone()),
        }
    });


    result
}

fn is_series(value: &KValue) -> bool {
    matches!(value, KValue::Series(_))
}
