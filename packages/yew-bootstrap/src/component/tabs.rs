use yew::prelude::*;

#[function_component]
fn TabHeader(props: &TabsProps) -> Html {
    let mut classes = classes!("nav");
    if props.fill {
        classes.push("nav-fill");
    }
    if props.justify {
        classes.push("nav-justify");
    }
    if props.variant == "pills" {
        classes.push("nav-pills");
    } else {
        classes.push("nav-tabs");
    }
    html! {
        <ul class={classes} id={&props.id} role="tablist">
            {for props.children.iter().enumerate().map(|(idx, child)| {
                let mut classes = classes!("nav-link");
                if idx == 0 {
                    classes.push("active");
                }
                if child.props.disabled {
                    classes.push("disabled");
                }
                html!{
                    <li class="nav-item" role="presentation">
                        <button
                            class={classes}
                            data-bs-toggle="tab"
                            data-bs-target={format!("#{}", child.props.title)}
                            type="button"
                            role="tab"
                            aria-controls={&child.props.title}
                            aria-selected="false"
                            disabled={child.props.disabled}
                        >
                        {&child.props.title}
                        </button>
                    </li>
                }
            })}
        </ul>
    }
}

#[derive(Properties, Clone, PartialEq)]
struct TabContentProps {}

#[function_component]
fn TabContent(props: &TabContentProps) -> Html {
    todo!()
}

#[derive(Properties, Clone, PartialEq)]
pub struct TabItemProps {
    #[prop_or_default]
    pub active: bool,

    pub title: AttrValue,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn TabItem(props: &TabItemProps) -> Html {
    let mut classes = classes!{"tab-pane", "fade"};
    if props.active {
        classes.push("show");
        classes.push("active");
    }
    html!{
        <div class={classes} role="tabpanel">{for props.children.iter()}</div>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct TabsProps {
    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or(AttrValue::from("tabs"))]
    pub variant: AttrValue,

    #[prop_or_default]
    pub fill: bool,

    #[prop_or_default]
    pub justify: bool,

    #[prop_or_default]
    pub children: ChildrenWithProps<TabItem>,
}

#[function_component]
pub fn Tabs(props: &TabsProps) -> Html {
    html! {
        <>
            <TabHeader ..props.clone() />
            <div class="tab-content">
                {for props.children.iter()}
            </div>
        </>
    }
}
