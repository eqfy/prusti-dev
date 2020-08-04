extern crate proc_macro;

#[cfg(not(feature = "prusti"))]
mod private {
    use proc_macro_hack::proc_macro_hack;

    /// A macro for writing a precondition on a function.
    pub use prusti_contracts_impl::requires;

    /// A macro for writing a postcondition on a function.
    pub use prusti_contracts_impl::ensures;

    /// A macro for writing a pledge on a function.
    pub use prusti_contracts_impl::after_expiry;

    /// A macro for writing a conditional pledge on a function.
    pub use prusti_contracts_impl::after_expiry_if;

    /// A macro for marking a function as pure.
    pub use prusti_contracts_impl::pure;

    /// A macro for writing a loop invariant.
    #[proc_macro_hack]
    pub use prusti_contracts_impl::invariant;

    /// A macro for writing a postcondition on a thread.
    #[proc_macro_hack]
    pub use prusti_contracts_impl::thread_ensures;

    // pub use prusti_contracts_impl::attr_test;
}

#[cfg(feature = "prusti")]
mod private {
    use proc_macro_hack::proc_macro_hack;

    /// A macro for writing a precondition on a function.
    pub use prusti_contracts_internal::requires;

    /// A macro for writing a postcondition on a function.
    pub use prusti_contracts_internal::ensures;

    /// A macro for writing a pledge on a function.
    pub use prusti_contracts_internal::after_expiry;

    /// A macro for writing a conditional pledge on a function.
    pub use prusti_contracts_internal::after_expiry_if;

    /// A macro for marking a function as pure.
    pub use prusti_contracts_internal::pure;

    /// A macro for writing a loop invariant.
    #[proc_macro_hack]
    pub use prusti_contracts_internal::invariant;

    /// A macro for writing a postcondition on a thread.
    #[proc_macro_hack]
    pub use prusti_contracts_internal::thread_ensures;

    pub use prusti_contracts_internal::attr_test;

    pub use prusti_contracts_internal::attr_test1;
}

pub use private::*;
