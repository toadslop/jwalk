use leptos::logging::error;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, thiserror::Error, Serialize, Deserialize, Clone)]
pub enum Error {}

pub fn handle_batch_error<T, E>(
    iter: impl IntoIterator<Item = Result<T, E>>,
) -> impl Iterator<Item = T>
where
    T: Debug,
    E: std::error::Error,
{
    let (data, errors): (Vec<_>, Vec<_>) = iter.into_iter().partition(Result::is_ok);

    let errors: Vec<_> = errors
        .into_iter()
        .map(Result::unwrap_err)
        .inspect(|e| error!("{e}"))
        .collect();

    debug_assert!(errors.is_empty());

    data.into_iter().map(Result::unwrap)
}
