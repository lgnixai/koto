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
    result.add_fn("insert", |ctx| {
        let expected_error = "|List, Number, Any|";

        match ctx.instance_and_args(is_series, expected_error)? {
            (KValue::Series(l), [KValue::Number(n), value]) => {
                let index: usize = n.into();
                if *n < 0.0 || index > l.data().len() {
                    return runtime_error!("Index out of bounds");
                }

                l.data_mut().insert(index, value.clone());
                Ok(KValue::Series(l.clone()))
            }
            (instance, args) => unexpected_args_after_instance(expected_error, instance, args),
        }
    });

    result
}

fn is_series(value: &KValue) -> bool {
    matches!(value, KValue::Series(_))
}
