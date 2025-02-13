# Analysis of the `#[app::event]` Macro in Calimero SDK

## **1. Introduction**

This document analyzes the `#[app::event]` macro used in the Calimero SDK, which is implemented in `crates/sdk/macros/src/lib.rs`. The goal is to understand its transformation process, dependencies, and how it integrates into the SDK.

## **2. Macro Entry Point**

The macro is defined as follows:

```rust
#[proc_macro_attribute]
pub fn event(args: TokenStream, input: TokenStream) -> TokenStream {
    reserved::init();
    let _args = parse_macro_input!({ input } => args as Empty);
    let item = parse_macro_input!(input as StructOrEnumItem);
    let tokens = match EventImpl::try_from(EventImplInput { item: &item }) {
        Ok(data) => data.to_token_stream(),
        Err(err) => err.to_compile_error(),
    };
    tokens.into()
}
```

### **2.1 Observations:**

- `reserved::init();` initializes reserved keywords or names.
- `parse_macro_input!` extracts the input structure or enum.
- `EventImpl::try_from` processes the parsed input and converts it into a valid Rust token stream.
- If parsing fails, `err.to_compile_error()` generates a compile-time error.

## **3. Core Macro Implementation (`EventImpl`)**

The main transformation is performed inside `EventImpl`, which implements `ToTokens`:

```rust
impl ToTokens for EventImpl<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let EventImpl { ident, generics: source_generics, orig } = *self;
        let mut generics = source_generics.clone();

        for generic_ty in source_generics.type_params() {
            generics.make_where_clause().predicates.push(parse_quote!(#generic_ty: ::calimero_sdk::serde::Serialize));
        }

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        quote! {
            #[derive(::calimero_sdk::serde::Serialize)]
            #[serde(crate = "::calimero_sdk::serde")]
            #[serde(tag = "kind", content = "data")]
            #orig

            impl #impl_generics ::calimero_sdk::event::AppEvent for #ident #ty_generics #where_clause {
                fn kind(&self) -> ::std::borrow::Cow<str> {
                    match ::calimero_sdk::serde_json::to_value(self) {
                        Ok(data) => ::std::borrow::Cow::Owned(data["kind"].as_str().expect("Failed to get event kind").to_string()),
                        Err(err) => ::calimero_sdk::env::panic_str(&format!("Failed to serialize event: {:?}", err)),
                    }
                }
                fn data(&self) -> ::std::borrow::Cow<[u8]> {
                    match ::calimero_sdk::serde_json::to_value(self) {
                        Ok(data) => ::std::borrow::Cow::Owned(::calimero_sdk::serde_json::to_vec(&data["data"]).expect("Failed to serialize event data")),
                        Err(err) => ::calimero_sdk::env::panic_str(&format!("Failed to serialize event: {:?}", err)),
                    }
                }
            }

            impl #impl_generics ::calimero_sdk::event::AppEventExt for #ident #ty_generics #where_clause {}
        }
        .to_tokens(tokens);
    }
}
```

### **3.1 Observations:**

- **Adds Serde Serialization**: `#[derive(Serialize)]` enables JSON serialization.
- **Implements `AppEvent` Trait**:
  - `kind()` extracts the event type.
  - `data()` serializes the event.
- **Adds `AppEventExt`**: Possibly extends event functionality.

## **4. Validation & Error Handling**

The macro ensures that events are properly formatted before processing.

```rust
match vis {
    Visibility::Public(_) => {}
    Visibility::Inherited => {
        return Err(errors.finish(SynError::new_spanned(ident, ParseError::NoPrivateEvent)));
    }
    Visibility::Restricted(spec) => {
        return Err(errors.finish(SynError::new_spanned(spec, ParseError::NoComplexVisibility)));
    }
}
```

### **4.1 Observations:**

- **Restricts Event Visibility**:
  - Must be `pub`, preventing private event definitions.
  - `pub(crate)`, `pub(in module)` are not allowed.
- **Prevents Reserved Keywords**:
  - Uses `reserved::init()` to check for reserved identifiers.

## **5. Dependencies & Missing Parts**

While most of the macro is in `lib.rs`, some **external dependencies** contribute to its behavior:

### **5.1 Key Dependencies**

| **Dependency**     | **Functionality**                                   |
| ------------------ | --------------------------------------------------- |
| `StructOrEnumItem` | Parses the input struct/enum before transformation. |
| `AppEventExt`      | Likely provides additional event-related methods.   |
| `reserved::init()` | Ensures reserved names are not used.                |
| `Errors` Struct    | Improves macro error handling.                      |

### **5.2 Open Questions**

To fully analyze the macro, we should investigate:

1. \*\*What is `StructOrEnum
