use sycamore::prelude::*;

#[derive(Prop, Debug)]
pub struct ConditionalProps<'a, G: GenericNode, F>
where
    F: Fn(BoundedScope<'_, 'a>) -> View<G> + 'a,
{
    cond: &'a ReadSignal<bool>,
    view: F,
}

#[component]
pub fn Conditional<'a, G: GenericNode, F>(
    cx: Scope<'a>,
    props: ConditionalProps<'a, G, F>,
) -> View<G>
where
    F: Fn(BoundedScope<'_, 'a>) -> View<G> + 'a,
{
    let ConditionalProps { cond, view } = props;

    let mapped: &ReadSignal<View<G>> = cond.map(cx, move |c| {
        if *c {
            view(cx)
        } else {
            view! { cx, }
        }
    });

    View::new_dyn(cx, || mapped.get().as_ref().clone())
}
