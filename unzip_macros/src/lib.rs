use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, Index, LitInt};

fn tys(count: usize) -> Vec<Ident> {
    (0..count)
        .map(|i| Ident::new(&format!("T{i}"), Span::call_site()))
        .collect()
}

#[proc_macro]
pub fn impl_ext(input: TokenStream) -> TokenStream {
    let count = parse_macro_input!(input as LitInt).base10_parse().unwrap();

    let tys = tys(count);
    let iters = tys
        .iter()
        .zip(0..count)
        .map(|(t, i)| quote!(UnzipIter<I, #t, #i>));
    let unzips_vals = (0..count).map(|_| quote!(UnzipIter::new(&unzipped)));
    quote! {
        impl<I, #(#tys),*> UnzipExt<(#(#tys),*)> for I
        where
            I: Iterator<Item = (#(#tys),*)>,
        {
            type Iters = (#(#iters),*);
            fn unzip_iter(self) -> Self::Iters {
                let unzipped = Unzipped::new(self);
                (#(#unzips_vals),*)
            }
        }
    }
    .into()
}

#[proc_macro]
pub fn impl_get(input: TokenStream) -> TokenStream {
    let count = parse_macro_input!(input as LitInt).base10_parse().unwrap();

    let tys = tys(count);

    let impls = (0..count).map(|target_in| {
        let target_ty = &tys[target_in];
        let implementee = quote!((#(#tys),*));
        let target_accessor = syn::Index::from(target_in);
        quote! {
            impl<#(#tys),*> Get<#target_ty, (#(#tys),*), #target_in> for #implementee {
                fn get(&mut self) ->  &mut #target_ty {
                    &mut self.#target_accessor
                }
            }
        }
    });

    quote! {
        #(#impls)*
    }
    .into()
}

#[proc_macro]
pub fn impl_wrap(input: TokenStream) -> TokenStream {
    let count = parse_macro_input!(input as LitInt).base10_parse().unwrap();

    let tys = tys(count);

    let implementee = quote!((#(#tys),*));
    let wrapped = quote!((#(VecDeque<#tys>),*));
    quote! {
        impl<#(#tys),*> Wrap for #implementee {
            type Wrapped = #wrapped;
        }
    }
    .into()
}

#[proc_macro]
pub fn impl_splat(input: TokenStream) -> TokenStream {
    let count = parse_macro_input!(input as LitInt).base10_parse().unwrap();

    let tys = tys(count);

    let implementee = quote!((#(#tys),*));
    let impls = (0..count).map(|ex| {
        let ex_ty = &tys[ex];
        let all = (0..count)
            .filter(|&n| n != ex)
            .map(Index::from)
            .map(|n| quote!(to.#n.push_back(self.#n);));
        let ex_i = Index::from(ex);
        quote! {
            impl<#(#tys),*> Splat<#ex_ty, #ex> for #implementee {
                fn splat(self, to: &mut Self::Wrapped) -> #ex_ty {
                    #(#all)*
                    self.#ex_i
                }
            }
        }
    });
    quote!(#(#impls)*).into()
}

#[proc_macro]
pub fn impl_unzipped(input: TokenStream) -> TokenStream {
    let count = parse_macro_input!(input as LitInt).base10_parse().unwrap();

    let tys = tys(count);

    let impls = (0..count).map(|target_in| {
        let target_ty = &tys[target_in];
        let ret_ty = quote!(Option<#target_ty>);
        quote! {
            impl<I, #(#tys),*> UnzippedImpl<#target_ty, (#(#tys),*), #target_in> for Unzipped<I>
            where
                I: UnzipExt<(#(#tys),*)>,
                I: Iterator<Item = (#(#tys),*)>,
                (#(#tys),*): Wrap,
                <(#(#tys),*) as Wrap>::Wrapped: Get<VecDeque<#target_ty>, <(#(#tys),*) as Wrap>::Wrapped, #target_in>
            {
                fn get(&mut self) -> #ret_ty {
                    match self.cache.get().pop_front() {
                        None => {
                            if let Some(next) = self.inner.next() {
                                Some(next.splat(&mut self.cache))
                            } else {
                                None
                            }
                        }
                        r => r,
                    }
                }
            }
        }
    });
    quote!(#(#impls)*).into()
}
