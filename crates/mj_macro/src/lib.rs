extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(GetInstanceDerive)]
pub fn get_instance_derive(input: TokenStream) -> TokenStream {
    let ast: &syn::DeriveInput = &syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl mj_base::env::GetEnvTrait for #name {
            fn get_instance(&self) -> j4rs::Instance{
                j4rs::Jvm::attach_thread().unwrap().clone_instance(&self.instance).unwrap()
            }
        }
    };
    gen.into()
}
#[proc_macro_derive(FromInstanceDerive)]
pub fn from_instance_derive(input: TokenStream) -> TokenStream {
    let ast: &syn::DeriveInput = &syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl mj_base::env::FromInstance for #name {
            fn from_instance(instance: j4rs::Instance) -> Self{
                Self{instance}
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn java_type(type_name: TokenStream, input: TokenStream) -> TokenStream {
    let ast: &syn::DeriveInput = &syn::parse(input).unwrap();
    let type_name: &syn::LitStr = &syn::parse(type_name).expect("类型名称请用字符串表示！");
    let name = &ast.ident;
    let gen = quote! {
        #ast
        impl mj_base::env::GetClassTypeTrait for #name {
            fn get_class_type() -> j4rs::Instance {
                j4rs::Jvm::attach_thread()
                    .unwrap()
                    .invoke_static(
                        "rt.lea.LumiaUtils",
                        "forName",
                        &[j4rs::InvocationArg::try_from(
                            #type_name,
                        )
                        .unwrap()],
                    )
                    .unwrap()
            }
            fn cast_to_this_type(instance: j4rs::Instance) -> j4rs::Instance{
                let jvm = j4rs::Jvm::attach_thread()
                    .unwrap();
                jvm.cast(&instance, #type_name).unwrap()
            }
        }
    };
    gen.into()
}
