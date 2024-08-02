use gloo_console::log;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::{function_component, html::IntoHtmlResult, prelude::*};
use yewdux::{dispatch, use_store, Store};
use wasm_bindgen::JsCast;

use crate::{api, app::Role};
#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
#[store(storage = "local")]
pub struct LoginState {
    pub email: String,
    pub password: String,
    pub success: bool,
    pub token: String,
    pub role: Role
}


#[function_component]
pub fn LoginPage() -> Html {
  // let str = pulldown_cmark::Parser::new(MARKDOWN_SOURCE);
  //   let mut html_output = String::new();
  //   pulldown_cmark::html::push_html(&mut html_output, str);
  //   let val = Html::from_html_unchecked(html_output.into());
    let (state, dispatch) = use_store::<LoginState>();
  let handle_form_submit = dispatch.future_callback(|dispatch| async move {
    // log!("eta");
    // let (l_state, l_dispatch) = use_store::<State>();
      wasm_bindgen_futures::spawn_local(async move {
          let resp = api::api_login(
              dispatch.get().email.clone(),
              dispatch.get().password.clone(),
          ).await;
          
          dispatch.reduce_mut(|state| {
            state.token = resp.as_ref().unwrap().token.clone();
            state.role = resp.unwrap().role;
          })
      });
  });
    let (state, dispatch) = use_store::<LoginState>();
    let (global_state, global_dispatch) = use_store::<LoginState>();
    let email_changed = Callback::from(move |event: Event| {
        let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
        dispatch.reduce_mut(|state| {
            state.email = value;
        })
        
    });
    let (state, dispatch) = use_store::<LoginState>();
    let password_changed = Callback::from(move |event: Event| {
        let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
        dispatch.reduce_mut(|state| {
            state.password = value;
        })
    });

    // let (state, dispatch) = use_store::<State>();
    let (state, dispatch) = use_store::<LoginState>();
   
    html! {
    <>
    <div class="container reg-container">
    <div class="field">
  <p class="control has-icons-left has-icons-right">
    <label class="label">{"Email"}</label>
    <input class="input" type="email" placeholder="Email" onchange = {email_changed}/>
    <span class="icon is-small is-left">
      <i class="fas fa-envelope"></i>
    </span>
    <span class="icon is-small is-right">
      <i class="fas fa-check"></i>
    </span>
  </p>
</div>
<div class="field">
  <p class="control has-icons-left">
    <label class="label">{"Пароль"}</label>
    <input class="input" type="password" placeholder="Password" onchange = {password_changed}/>
    <span class="icon is-small is-left">
      <i class="fas fa-lock"></i>
    </span>
  </p>
</div>
{
  if state.success {
    html! {
    <div class="notification is-danger is-centered is-text-centered">
      {"Неверный логин или пароль!"}
    </div>
    }
  }
  else {
    html!{}
  }
}
<div class="field">
  <p class="control">
    <button class="button is-success" onclick={handle_form_submit}>
      {"Вход"}
    </button>
  </p>
</div>
</div>
    </>
    }
}