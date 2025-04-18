use syn::parse::{Parse, ParseStream};

pub mod canextractstate;
pub mod extractidents;
mod extractstate;
pub mod varfunc;

fn parse_zero_or_more<T: Parse>(input: ParseStream) -> Vec<T> {
    let mut result = Vec::new();
    while let Ok(item) = input.parse() {
        result.push(item);
    }
    result
}
