extern crate const_random;
extern crate proc_macro;
extern crate razy_importer;

use const_random::const_random;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;

use syn::{parse_macro_input, AttributeArgs, Ident, ItemFn, Lit, Meta, NestedMeta};
#[proc_macro_attribute]
pub fn dy_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(item as ItemFn);

    // Extract the function signature and name
    let fn_name = &input.sig.ident;
    let fn_name_str = fn_name.to_string();
    let fn_return_type = &input.sig.output;
    let fn_inputs = &input.sig.inputs;

    let args = parse_macro_input!(attr as AttributeArgs);

    let proc_name = args
        .iter()
        .find_map(|arg| {
            if let NestedMeta::Meta(Meta::NameValue(nv)) = arg {
                if nv.path.is_ident("name") {
                    if let Lit::Str(litstr) = &nv.lit {
                        return Some(litstr.value());
                    }
                }
            }
            None
        })
        .unwrap_or(fn_name_str);

    let input_names = fn_inputs
        .iter()
        .map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(ident) = &*pat_type.pat {
                    return ident.ident.clone();
                }
            }
            panic!("Unsupported argument pattern");
        })
        .collect::<Vec<_>>();

    let randomu32: u32 = const_random!(u32);
    let cache_ident = Ident::new(
        format!("dynCached_{}", proc_name).as_str(),
        Span::call_site(),
    );

    let enc_key: u64 = const_random!(u64);
    let roll_random: u32 = (const_random!(u32) % (50 - 5)) + 5;

    let expanded = quote! {

        pub static #cache_ident:std::sync::Mutex<u64> = std::sync::Mutex::new(0);

        pub unsafe fn #fn_name(#fn_inputs) #fn_return_type {

            unsafe
            {
                let hash: razy_importer::OffsetHashPair = razy_importer::hash::khash(
                    #proc_name.as_bytes(),
                    #randomu32,
                );

                let mut encrypted_address = #cache_ident.lock().unwrap();

                if *encrypted_address == 0
                {
                    let imp_address = razy_importer::get_export_forwarded(hash);
                    if imp_address == 0
                    {
                        panic!("imp_address == 0");
                    }
                    *encrypted_address =  !(imp_address ^ #enc_key).rotate_left(#roll_random);
                }

                let decrypted_address =   ((!*encrypted_address).rotate_right(#roll_random) ^ #enc_key);

                std::mem::transmute::<_,unsafe extern "system" fn(#fn_inputs) #fn_return_type>(decrypted_address)(#(#input_names),*)
            }
        }
    };

    TokenStream::from(expanded)
}
