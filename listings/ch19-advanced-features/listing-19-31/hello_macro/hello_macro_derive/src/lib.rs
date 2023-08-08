use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 조작 가능한 구문 트리로 러스트 코드의 표현을
    // 구성합니다
    let ast = syn::parse(input).unwrap();

    // 트레이트 구현체를 생성합니다
    impl_hello_macro(&ast)
}
