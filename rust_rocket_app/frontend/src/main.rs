use gloo::net::http::Request;
use serde::{Deserialize, Serialize};
use shared::User as SharedUser;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[derive(Deserialize)]
struct AuthResponse {
    token: String,
}

#[function_component(App)]
fn app() -> Html {
    let user_state = use_state(|| ("".to_string(), "".to_string(), None as Option<i32>));
    let message = use_state(|| "".to_string());
    let users = use_state(Vec::new);
    let token = use_state(|| None as Option<String>);
    let login_state = use_state(|| ("".to_string(), "".to_string()));

    let login = {
        let token = token.clone();
        let message = message.clone();
        let login_state = login_state.clone();

        Callback::from(move |_| {
            let token = token.clone();
            let message = message.clone();
            let login_state = login_state.clone();
            let (email, password) = (*login_state).clone();

            spawn_local(async move {
                let body = serde_json::json!({
                    "email": email,
                    "password": password
                });

                match Request::post("/api/login")
                    .header("Content-Type", "application/json")
                    .body(body.to_string())
                    .unwrap()
                    .send()
                    .await
                {
                    Ok(resp) if resp.ok() => {
                        let auth: AuthResponse = resp.json().await.unwrap();
                        token.set(Some(auth.token));
                        message.set("Logged in successfully".into());
                    }
                    _ => {
                        message.set("Login failed".into());
                    }
                }

                login_state.set(("".to_string(), "".to_string()));
            });
        })
    };

    let logout = {
        let token = token.clone();
        let message = message.clone();
        Callback::from(move |_| {
            token.set(None);
            message.set("Logged out".into());
        })
    };

    let get_users = {
        let users = users.clone();
        let message = message.clone();
        Callback::from(move |_| {
            let users = users.clone();
            let message = message.clone();
            spawn_local(async move {
                match Request::get("/api/get_users").send().await {
                    Ok(resp) if resp.ok() => {
                        let fetched_users: Vec<SharedUser> = resp.json().await.unwrap_or_default();
                        users.set(fetched_users);
                    }
                    _ => message.set("Failed to fetch users".into()),
                }
            });
        })
    };

    let create_user = {
        let user_state = user_state.clone();
        let message = message.clone();
        let get_users = get_users.clone();
        Callback::from(move |_| {
            let (name, email, _) = (*user_state).clone();
            let user_state = user_state.clone();
            let message = message.clone();
            let get_users = get_users.clone();

            spawn_local(async move {
                let user_data = serde_json::json!({"name": name, "email": email});

                let response = Request::post("/api/add_user")
                    .header("Content-Type", "application/json")
                    .body(user_data.to_string())
                    .unwrap()
                    .send()
                    .await;

                match response {
                    Ok(resp) if resp.ok() => {
                        message.set("User created successfully".into());
                        get_users.emit(());
                    }
                    _ => message.set("Failed to create user".into()),
                }

                user_state.set(("".to_string(), "".to_string(), None));
            });
        })
    };

    let update_user = {
        let user_state = user_state.clone();
        let message = message.clone();
        let get_users = get_users.clone();
        let token = token.clone();

        Callback::from(move |_| {
            let (name, email, editing_user_id) = (*user_state).clone();
            let user_state = user_state.clone();
            let message = message.clone();
            let get_users = get_users.clone();
            let token = (*token).clone();

            if let Some(id) = editing_user_id {
                spawn_local(async move {
                    let auth_header = match &token {
                        Some(t) => format!("Bearer {}", t),
                        None => {
                            message.set("Unauthorized to update user".into());
                            return;
                        }
                    };

                    let response = Request::put(&format!("/api/update_user/{}", id))
                        .header("Content-Type", "application/json")
                        .header("Authorization", &auth_header)
                        .body(serde_json::to_string(&(name.as_str(), email.as_str())).unwrap())
                        .unwrap()
                        .send()
                        .await;

                    match response {
                        Ok(resp) if resp.ok() => {
                            message.set("User updated successfully".into());
                            get_users.emit(());
                        }
                        _ => message.set("Failed to update user".into()),
                    }

                    user_state.set(("".to_string(), "".to_string(), None));
                });
            }
        })
    };

    let delete_user = {
        let message = message.clone();
        let get_users = get_users.clone();
        let token = token.clone();

        Callback::from(move |id: i32| {
            let message = message.clone();
            let get_users = get_users.clone();
            let token = (*token).clone();

            spawn_local(async move {
                let auth_header = match &token {
                    Some(t) => format!("Bearer {}", t),
                    None => {
                        message.set("Unauthorized to delete".into());
                        return;
                    }
                };

                let response = Request::delete(&format!("/api/delete_user/{}", id))
                    .header("Authorization", &auth_header)
                    .send()
                    .await;

                match response {
                    Ok(resp) if resp.ok() => {
                        message.set("User deleted successfully".into());
                        get_users.emit(());
                    }
                    _ => message.set("Failed to delete user".into()),
                }
            });
        })
    };

    let edit_user = {
        let user_state = user_state.clone();
        let users = users.clone();

        Callback::from(move |id: i32| {
            if let Some(user) = users.iter().find(|u| u.id == Some(id)) {
                user_state.set((user.name.clone(), user.email.clone(), Some(id)));
            }
        })
    };

    html! {
        <div class="container mx-auto p-4">
            <h1 class="text-4xl font-bold text-blue-500 mb-4">{ "User Management" }</h1>

            // Section login
            {
                if token.is_none() {
                    html! {
                        <div class="mb-6 p-4 border rounded bg-gray-50">
                            <h2 class="text-xl font-bold mb-2">{ "Login" }</h2>
                            <input
                                placeholder="Email"
                                value={login_state.0.clone()}
                                oninput={Callback::from({
                                    let login_state = login_state.clone();
                                    move |e: InputEvent| {
                                        let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                        login_state.set((input.value(), login_state.1.clone()));
                                    }
                                })}
                                class="border rounded px-4 py-2 mr-2"
                            />
                            <input
                                type="password"
                                placeholder="Password"
                                value={login_state.1.clone()}
                                oninput={Callback::from({
                                    let login_state = login_state.clone();
                                    move |e: InputEvent| {
                                        let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                        login_state.set((login_state.0.clone(), input.value()));
                                    }
                                })}
                                class="border rounded px-4 py-2 mr-2"
                            />
                            <button
                                onclick={login}
                                class="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded"
                            >
                                { "Login" }
                            </button>
                        </div>
                    }
                } else {
                    html! {
                        <div class="mb-6 p-4 border rounded bg-green-50">
                            <p class="text-green-700 font-semibold">{ "✅ Logged in" }</p>
                            <button
                                onclick={logout}
                                class="mt-2 bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-3 rounded"
                            >
                                { "Logout" }
                            </button>
                        </div>
                    }
                }
            }

            // Section gestion users
            <div class="mb-4">
                <input
                    placeholder="Name"
                    value={user_state.0.clone()}
                    oninput={Callback::from({
                        let user_state = user_state.clone();
                        move |e: InputEvent| {
                            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                            user_state.set((input.value(), user_state.1.clone(), user_state.2));
                        }
                    })}
                    class="border rounded px-4 py-2 mr-2"
                />
                <input
                    placeholder="Email"
                    value={user_state.1.clone()}
                    oninput={Callback::from({
                        let user_state = user_state.clone();
                        move |e: InputEvent| {
                            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                            user_state.set((user_state.0.clone(), input.value(), user_state.2));
                        }
                    })}
                    class="border rounded px-4 py-2 mr-2"
                />
                <button
                    onclick={if user_state.2.is_some() { update_user.clone() } else { create_user.clone() }}
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                >
                    { if user_state.2.is_some() { "Update User" } else { "Create User" } }
                </button>
                if !message.is_empty() {
                    <p class="text-green-500 mt-2">{ &*message }</p>
                }
            </div>

            <button
                onclick={get_users.reform(|_| ())}
                class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded mb-4"
            >
                { "Fetch User List" }
            </button>

            <h2 class="text-2xl font-bold text-gray-700 mb-2">{ "User List" }</h2>

            <ul class="list-disc pl-5">
                { for (*users).iter().map(|user| {
                    let user_id = user.id;
                    html! {
                        <li class="mb-2">
                            <span class="font-semibold">{ format!("ID: {}, Name: {}, Email: {}", user.id.unwrap_or(0), user.name, user.email) }</span>
                            <button
                                onclick={delete_user.clone().reform(move |_| user_id.unwrap_or(0))}
                                class="ml-4 bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-2 rounded"
                            >
                                { "Delete" }
                            </button>
                            <button
                                onclick={edit_user.clone().reform(move |_| user_id.unwrap_or(0))}
                                class="ml-4 bg-yellow-500 hover:bg-yellow-700 text-white font-bold py-1 px-2 rounded"
                            >
                                { "Edit" }
                            </button>
                        </li>
                    }
                })}
            </ul>
        </div>
    }
}
