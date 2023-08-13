use leptos::{For, *};

#[component]
pub fn LargeTable(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="translation-table-wrapper" role="region" tabindex="0">
        <div class="w-auto translation-table px-4 hide-x-scrollbar">
            <table class="mb-32">
                <thead>
                    <tr class="bg-primary">
                        <th class="bg-primary">
                            Keys
                        </th>
                        // <Locales />
                    </tr>
                </thead>
                <tbody>

                    <For
                        each=move || ((0..10_000).collect::<Vec<_>>())
                        // a unique key for each item
                        key=|item| *item
                        // renders each item to a view
                        view=|cx, item| {
                            view!{cx,
                                <tr>
                                    <th>{item}</th>
                                    // <td>Cy Ganderton</td>
                                    // <td>Quality Control Specialist</td>
                                    // <td>Littel, Schaden and Vandervort</td>
                                    // <td>Canada</td>
                                    // <td>12/16/2020</td>
                                    // <td>Blue</td>
                                </tr>
                            }
                        }
                    />
                </tbody>
            </table>
            </div>
        </div>
    }
}
