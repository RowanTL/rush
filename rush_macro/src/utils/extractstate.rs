// The parsing will be same for canextractstate.rs but
// instead of chaining trues at the end, will just chain the values.

use crate::utils::parse_zero_or_more;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use std::cmp::PartialEq;
use syn::parse::{Parse, ParseStream};
