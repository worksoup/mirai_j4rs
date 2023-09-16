extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(GetInstanceDerive)]
pub fn get_instance_derive(input: TokenStream) -> TokenStream {
    let ast: &syn::DeriveInput = &syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl crate::env::GetEnvTrait for #name {
            fn get_instance(&self) -> j4rs::Instance{
                Jvm::attach_thread().unwrap().clone_instance(&self.instance).unwrap()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(GetBotDerive)]
pub fn get_bot_derive(input: TokenStream) -> TokenStream {
    let ast: &syn::DeriveInput = &syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl crate::env::GetBotTrait for #name {
            fn get_bot(&self) -> crate::contact::bot::Bot {
                Bot{bot:Jvm::attach_thread().unwrap().clone_instance(&self.bot).unwrap(),id:0}
            }
        }
    };
    gen.into()
}

/// 获取 java 中的 Class 对象。
#[proc_macro_derive(GetClassTypeDerive)]
pub fn get_class_type_derive(input: TokenStream) -> TokenStream {
    let ast: &syn::DeriveInput = &syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl crate::env::GetClassTypeTrait for #name {
            fn get_class_type() -> j4rs::Instance {
                Jvm::attach_thread()
                    .unwrap()
                    .invoke_static(
                        "rt.lea.LumiaUtils",
                        "forName",
                        &[j4rs::InvocationArg::try_from(
                            #name::get_class_name(),
                        )
                        .unwrap()],
                    )
                    .unwrap()
            }
        }
    };
    gen.into()
}
