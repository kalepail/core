// #![no_std]

mod admin;
mod allowance;
mod balance;
mod contract;
mod event;
mod metadata;
mod storage_types;
mod test;

pub use crate::newtoken::contract::TokenClient;
pub use crate::newtoken::contract::Token;
pub use crate::newtoken::contract::TokenTrait;
