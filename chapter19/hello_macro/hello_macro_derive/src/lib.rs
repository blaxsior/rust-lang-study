use proc_macro::TokenStream; // 코드 수준에서 러스트 코드를 읽고 조작할 수 있게 하는 컴파일러 API

// 코드 파서 기능
use quote::quote; // syn 데이터 구조를 다시 러스트 코드로 변환
use syn; // 러스트 코드를 문자열에서 연산 가능한 데이터 코드로 파싱

// 토큰 스트림 파싱
// 스트림을 파싱하여 원하는 조작을 수행한 후 다시 코드로 변환하여 삽입
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 연산 가능한 데이터 구조로 변환
    let ast = syn::parse(input).unwrap();

    // ast 트리 변환
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // 반환할 코드 정의
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
// stringify!() : 문자열 리터럴로 변환