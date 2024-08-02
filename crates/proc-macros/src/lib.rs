use proc_macro::TokenStream;

fn resolve_asset_path(path: String) -> String {
    let path = path.strip_prefix("assets/").unwrap_or(&path);

    let assets_path = std::env::var("LUMINOL_ASSETS_PATH").expect("luminol asset path not present");
    let assets_path = std::path::PathBuf::from(assets_path);

    let asset_path = assets_path.join(path);
    asset_path.to_string_lossy().into_owned()
}

// TODO smarter asset system
// We should probably have an `include_asset_static!` and an `include_asset_runtime!` as well as a system for registering assets
#[proc_macro]
pub fn include_asset(input: TokenStream) -> TokenStream {
    let path: syn::LitStr = syn::parse(input).expect("Not a string literal");
    let path = path.value();

    let asset_path = resolve_asset_path(path);

    let tokens = quote::quote! {
        include_bytes!(#asset_path)
    };
    tokens.into()
}

#[proc_macro]
pub fn include_asset_str(input: TokenStream) -> TokenStream {
    let path: syn::LitStr = syn::parse(input).expect("Not a string literal");
    let path = path.value();

    let asset_path = resolve_asset_path(path);

    let tokens = quote::quote! {
        include_str!(#asset_path)
    };
    tokens.into()
}

#[proc_macro]
pub fn include_asset_dir(input: TokenStream) -> TokenStream {
    let path: syn::LitStr = syn::parse(input).expect("Not a string literal");
    let path = path.value();

    let asset_path = resolve_asset_path(path);
    let entries = std::fs::read_dir(asset_path).expect("Failed to read asset directory");

    let iter_tokens = entries.filter_map(Result::ok).map(|entry| {
        let path = entry.file_name();
        let path = path.to_string_lossy();

        let literal = syn::LitStr::new(&path, proc_macro::Span::call_site().into());

        quote::quote! {
            (#literal, include_bytes!(#path))
        }
    });

    let tokens = quote::quote! {
        [
            #(#iter_tokens),*
        ]
    };
    tokens.into()
}

#[proc_macro]
pub fn include_asset_dir_ids(input: TokenStream) -> TokenStream {
    let path: syn::LitStr = syn::parse(input).expect("Not a string literal");
    let path = path.value();

    let asset_path = resolve_asset_path(path);
    let entries = std::fs::read_dir(asset_path).expect("Failed to read asset directory");

    let iter_tokens = entries.filter_map(Result::ok).map(|entry| {
        let path = entry.path();

        let filename = path.file_stem().unwrap();
        let filename = filename.to_string_lossy();

        let path = path.to_string_lossy();

        let literal = syn::LitInt::new(&filename, proc_macro::Span::call_site().into());

        quote::quote! {
            (#literal, include_bytes!(#path).as_slice())
        }
    });

    let tokens = quote::quote! {
        [
            #(#iter_tokens),*
        ]
    };
    tokens.into()
}
