use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn app(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn = syn::parse_macro_input!(item as syn::ItemFn);

    let item_fn_name = &item_fn.sig.ident;

    quote! {
        pub fn main() {
            let runtime = genezio::tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .unwrap();

            let app = #item_fn_name();

            #[cfg(genezio_with_lambda)]
            {
                println!(
                    "trap ready to receive events {}",
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis()
                );
                 runtime.block_on(async {
                     genezio::lambda_http::run(app).await.unwrap();
                 });
            }

            #[cfg(not(genezio_with_lambda))]
            {
                 runtime.block_on(async {
                     let listener = genezio::tokio::net::TcpListener::bind("127.0.0.1:3000")
                         .await
                         .unwrap();
                     println!("listening on {}", listener.local_addr().unwrap());

                     genezio::axum::serve(listener, app).await.unwrap();
                 });
            }
        }

        #item_fn
    }
    .into()
}
