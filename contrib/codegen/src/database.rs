use proc_macro::{TokenStream, Diagnostic};
use syn::{DataStruct, Fields, Data, Type, LitStr, DeriveInput};

use spanned::Spanned;

type Result<T> = ::std::result::Result<T, Diagnostic>;

#[derive(Debug)]
struct DatabaseInvocation {
    /// The database name as passed in via #[database('database name')].
    db_name: String,
    /// The entire structure that the `database` attribute was called on.
    structure: DataStruct,
    /// The type inside the structure: struct MyDb(ThisType).
    connection_type: Type
}

const EXAMPLE: &str = "example: `struct MyDatabase(diesel::SqliteConnection);`";
const ONLY_ON_STRUCTS_MSG: &str = "`database` attribute can only be used on structs";
const ONLY_UNNAMED_FIELDS: &str = "`database` attribute can only be applied to structs with \
    exactly one unnamed field";

fn parse_invocation(attr: TokenStream, input: TokenStream) -> Result<DatabaseInvocation> {
    let attr_stream2 = ::proc_macro2::TokenStream::from(attr);
    let attr_span = attr_stream2.span();
    let string_lit = ::syn::parse2::<LitStr>(attr_stream2)
        .map_err(|_| attr_span.error("expected string literal"))?;

    let input = ::syn::parse::<DeriveInput>(input).unwrap();
    let structure = match input.data {
        Data::Struct(s) => s,
        _ => return Err(input.span().error(ONLY_ON_STRUCTS_MSG))
    };

    let inner_type = match structure.fields {
        Fields::Unnamed(ref fields) if fields.unnamed.len() == 1 => {
            let first = fields.unnamed.first().expect("checked length");
            first.value().ty.clone()
        }
        _ => return Err(structure.fields.span().error(ONLY_UNNAMED_FIELDS).help(EXAMPLE))
    };

    Ok(DatabaseInvocation {
        db_name: string_lit.value(),
        structure: structure,
        connection_type: inner_type
    })
}

pub fn database_attr(attr: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let invocation = parse_invocation(attr, input)?;
    println!("Invocation: {:?}", invocation.connection_type);
    unimplemented!("the database attribute has not yet been implemented")
}
