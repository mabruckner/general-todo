use yew::{platform::{pinned::oneshot, spawn_local}, prelude::*};
use todo_shared::*;
use reqwest;
use web_sys::{self, HtmlInputElement, wasm_bindgen::JsCast};


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

// this will cause problems if it isn't used on an input type="text"
fn textinput_callback(handle: UseStateHandle<String>) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let target: HtmlInputElement = e.target().unwrap().unchecked_into();
        handle.set(target.value());
    })
}

#[derive(Properties, PartialEq)]
pub struct LoginProps {
    pub on_user_change: Callback<CurrentUserResponse>
}

#[component]
fn LogIn(props: &LoginProps) -> Html {
    let user_change = props.on_user_change.clone();
    let username_handle = use_state(String::default);
    let username = (*username_handle).clone();
    let password_handle = use_state(String::default);
    let password = (*password_handle).clone();
    let password_changed = textinput_callback(password_handle.clone());
    let username_changed = textinput_callback(username_handle.clone());
    let login = {
        let user_change = user_change.clone();
        let username_handle= username_handle.clone();
        let password_handle = password_handle.clone();
        move |_| {
            let user_change = user_change.clone();
            let username_handle= username_handle.clone();
            let password_handle = password_handle.clone();
            spawn_local(async move {
                let res = request_login(Credentials {
                    username: (*username_handle).clone(),
                    password: (*password_handle).clone()
                }).await;
                if let Ok(user) = res {
                    user_change.emit(user);
                }
                username_handle.set("".into());
                password_handle.set("".into());
            })
        }
    };
    let register = {
        move |_| {
            let user_change = user_change.clone();
            let username_handle= username_handle.clone();
            let password_handle = password_handle.clone();
            spawn_local(async move {
                let res = request_register(Credentials {
                    username: (*username_handle).clone(),
                    password: (*password_handle).clone()
                }).await;
                if let Ok(user) = res {
                    user_change.emit(user);
                }
                username_handle.set("".into());
                password_handle.set("".into());
            })
        }
    };
    html! {
        <div class="card col-4 mx-auto">
            <div class="card-body">
                <h5 class="card-title">{ "Log In or Register" }</h5>
                <form>
                    <div class="mb-3">
                        <input type="text" class="form-control" placeholder="username" onchange={ username_changed } value={ username }/>
                    </div>
                    <div class="mb-3">
                        <input type="password" class="form-control" placeholder="password" onchange={ password_changed } value={ password }/>
                    </div>
                    <div class="d-grid gap-2">
                        <button type="button" class="btn btn-primary btn-lg" onclick={login}>{ "Log In" }</button>
                        <button type="button" class="btn btn-secondary" onclick={register}>{ "Register" }</button>
                    </div>
                </form>
            </div>
        </div>
    }
}

#[component]
fn TaskAdd() -> Html {
    html! {
        <form class="row g-2">
            <div class="col-auto">
                <input type="text" class="form-control" placeholder="task content"/>
            </div>
            <div class="col-auto">
                <button type="submit" class="btn btn-primary mb-3">{"ADD"}</button>
            </div>
        </form>
    }
}

#[component]
fn TaskView() -> Html {
    html! {
        <div class="container-md">
            <div class="card">
                <TaskAdd />
            </div>
        </div>
    }

}

fn set_origin(relative_url: &str) -> String {
    let origin = web_sys::window().unwrap().location().origin().unwrap();
    format!("{}{}", origin, relative_url).into()

}

pub async fn add_task(contents: String) -> reqwest::Result<Task> {
    reqwest::Client::new()
        .post(set_origin("/api/tasks"))
        .json(&contents)
        .send().await?
        .json().await
}
pub async fn get_all_tasks() -> reqwest::Result<Vec<Task>> {
    reqwest::Client::new()
        .get(set_origin("/api/tasks"))
        .send().await?
        .json().await
}
pub async fn get_current_user() -> reqwest::Result<CurrentUserResponse> {
    reqwest::get(set_origin("/api/user")).await?
        .json().await
}
pub async fn request_login(creds: Credentials) -> reqwest::Result<CurrentUserResponse> {
    reqwest::Client::new()
        .post(set_origin("/api/login"))
        .json(&creds)
        .send().await?
        .json().await
}
pub async fn request_register(creds: Credentials) -> reqwest::Result<CurrentUserResponse> {
    reqwest::Client::new()
        .post(set_origin("/api/register"))
        .json(&creds)
        .send().await?
        .json().await
}

#[component]
fn App() -> Html {
    let user_handle = use_state(|| { CurrentUserResponse::LoggedOut });
    let user_changed = {
        let user_handle = user_handle.clone();
        Callback::from(move |user| {
            user_handle.set(user);
        })
    };
    {
        let user_changed = user_changed.clone();
        use_state(|| {
            spawn_local(async move {
                    let res = get_current_user().await;
                    if let Ok(user) = res {
                        user_changed.emit(user);
                    };
                })
        });
    }
    html! {
        <div>
            if let CurrentUserResponse::User { name, id } = (*user_handle).clone() {
                <TopBar logged_in=true />
                <TaskView />
            } else {
                <TopBar logged_in=false />
                <LogIn on_user_change={user_changed}/>
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
