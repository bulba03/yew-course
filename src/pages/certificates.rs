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
pub fn Certificates() -> Html {

    html! {
    <>
    <div class="container">
    <div class="box">
    <p>
    {
      "Молодец Ты прошел курс по языку программирования Rust, Так держать!"
    }
    </p>
    <img
                src={"https://img-c.udemycdn.com/course/750x422/2999814_f337.jpg"}
                alt="Placeholder image"
            />
    <p>
    {"Учебный центр компании (Company_Name) подтверждает подлинность сертификата и добросовестное выполнение курса. Студент получил все необходимые навыки и в совершенстве владеет программой курса"}
    </p>
    <section class="hero is-primary">
  <div class="hero-body">
    <p class="title">{"Благодарим за выполнение"}</p>
    <p class="subtitle">{"Учебный центр (Company_Namy)"}</p>
  </div>
</section>
    </div>
    </div>
    
    </>
    }
}