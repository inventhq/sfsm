use proc_macro2::TokenStream;
use quote::quote;

pub fn format_log(sfsm_name: &str, action: &str, log: &str) -> String {
    if !log.is_empty() {
        format!("{}: {} - {}", sfsm_name, action, log)
    } else {
        format!("{}: {}", sfsm_name, action)
    }
}

#[cfg(not(feature = "trace"))]
pub fn trace(_str: String) -> TokenStream {
    quote! {}
}

#[cfg(not(feature = "trace-steps"))]
pub fn step(_str: String) -> TokenStream {
    quote! {}
}

#[cfg(not(feature = "trace-messages"))]
pub fn message(_str: String) -> TokenStream {
    quote! {}
}

#[cfg(feature = "trace")]
pub fn trace(str: String) -> TokenStream {
    proc_macro2::TokenStream::from(quote! {
        __sfsm_trace(#str);
    })
}

#[cfg(feature = "trace-steps")]
pub fn step(str: String) -> TokenStream {
    proc_macro2::TokenStream::from(quote! {
        __sfsm_trace(#str);
    })
}

#[cfg(feature = "trace-messages")]
pub fn message(str: String) -> TokenStream {
    proc_macro2::TokenStream::from(quote! {
        __sfsm_trace(#str);
    })
}
