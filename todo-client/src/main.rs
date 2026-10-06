use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct NavProps {
    pub logged_in: bool,
}

#[component]
fn TopBar(&NavProps { logged_in }: &NavProps) -> Html {
    html! {
        <nav class="navbar">
            <div class="container-fluid">
                <div class="navbar-brand">{ "Much Todo" }</div>
                if logged_in {
                    { "YOU ARE LOGGED IN" }
                } else {
                    { "YOU ARE NOT LOGGED IN" }
                }
            </div>
        </nav>
    }
}

#[component]
fn App() -> Html {
    html! {
        <div>
            <TopBar logged_in=true />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
