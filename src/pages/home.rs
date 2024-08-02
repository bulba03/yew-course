use yew::{ function_component, html, Html };
use yew_router::components::Link;

use crate::app::Route;

#[function_component]
pub fn HomePage() -> Html {
    html! {
    <>
    <div class="container ">
      <div class="fixed-grid has-1-cols"> 
      <div class="grid">
      <div class="cell">
      <section class="hero is-medium is-dark">
  <div class="hero-body">
    <p class="title">{"Учебный центр IT-компании"}</p>
    <p class="subtitle">{"Пройдите к регистрации или входу"}</p>
  </div>
</section>
  </div>
  <div class="cell">
  <div class="buttons">
    
                <a class="button is-dark has-text-white">
                <Link<Route> to={Route::Register} classes ="has-text-white">
                {"Регистрация"}
                </Link<Route>>
              </a>
              <a class="button is-dark has-text-black">
              <Link<Route> to={Route::Login} classes ="has-text-black">
                {"Вход"}
                </Link<Route>>
              </a>
    </div>
    </div>       
  </div>
  </div>
</div>
    </>
    
    }
}
