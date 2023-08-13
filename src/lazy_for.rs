use leptos::{component, IntoView, Scope, View};
use std::{hash::Hash, marker::PhantomData};

/// Iterates over children and displays them, keyed by the `key` function given.
/// this is supposed to be a more efficient version of the `For` component
/// as it only renders the items that are in the viewport + a buffer
/// ```
/// # use leptos::*;
/// #
/// # #[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// # struct Counter {
/// #   id: usize,
/// #   count: RwSignal<i32>
/// # }
///
/// #[component]
/// fn Counters(cx: Scope) -> impl IntoView {
///  let (counters, set_counters) = create_signal::<Vec<Counter>>(cx, vec![]);
///
///  view! {
///    cx,
///     <div>
///         <LazyFor
///             // a function that returns the items we're iterating over; a signal is fine
///             items=move || counters.get()
///             // a unique key for each item
///             key=|counter| counter.id
///             // renders each item to a view
///             view=move |cx, counter: Counter| {
///                 view! {
///                     cx,
///                     <button>"Value: " {move || counter.count.get()}</button>
///                 }
///            }
///        />
///    </div>   
///  }
///
/// }
///
/// ```

#[component]
pub fn LazyFor<IF, I, T, EF, N, KF, K>(
    cx: Scope,
    /// Items over which the component should iterate.
    items: IF,
    /// A key function that will be applied to each item.
    key: KF,
    /// The view that will be displayed for each item.
    view: EF,
) -> impl IntoView
where
    IF: Fn() -> I + 'static,
    I: IntoIterator<Item = T>,
    EF: Fn(Scope, T) -> N + 'static,
    N: IntoView,
    KF: Fn(&T) -> K + 'static,
    K: Eq + Hash + 'static,
{
}

// pub struct LazyFor<'a, T, I, N, K>
// where
//     T: 'a,
//     I: IntoIterator<Item = T>,
//     N: IntoView,
//     K: Eq + Hash + 'a,
// {
//     pub(crate) items_fn: fn() -> I,
//     pub(crate) each_fn: fn(Scope, T) -> N,
//     key_fn: fn(&T) -> K,
//     _marker: PhantomData<&'a T>,
// }

// impl<'a, T, I, N, K> IntoView for LazyFor<'a, T, I, N, K>
// where
//     T: 'a,
//     I: IntoIterator<Item = T>,
//     N: IntoView,
//     K: Eq + Hash + 'a,
// {
//     fn into_view(self, cx: Scope) -> View {
//         todo!()
//     }
// }
