use std::default;

use gloo_console::log;
use serde::{Deserialize, Serialize};
use yew_router::prelude::*;
use yew::prelude::*;
use yewdux::{use_store, Store};
use crate::pages::course_edit::CourseEdit;
use crate::pages::register::RegisterPage;
use crate::pages::courses_list::CoursesList;
use crate::pages::login::{LoginPage, LoginState};
use crate::pages::course_info::CourseInfo;
use crate::pages::course_material::CourseMaterial;
use crate::pages::home::HomePage;
use crate::pages::my_courses::MyCoursesList;
use crate::pages::profile_info::ProfileInfo;
use crate::pages::users_page::UsersList;
use crate::pages::admin_courses::AdminCourses;
use crate::pages::material_creation::MaterialCreation;
use crate::pages::certificates::Certificates;
// const COURSE_IMG: &str = "./assets/img.png";
pub const SERVER_URL: &str = "http://127.0.0.1:8081";

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
    // #[default]
    #[at("/reg")]
    Register,
    #[at("/login")]
    Login,
    #[at("/my_courses")]
    MyCourses,
    #[at("/course_info/:id")]
    CourseInfo {id: i128},
    #[at("/mymy_courses")]
    MyMyCourses,
    #[at("/lesson/:course_id/:lesson_id")]
    Lesson {course_id: i32, lesson_id:i32},
    #[at("/profile")]
    Profile,
    #[at("/users_list")]
    UsersList,
    #[at("/admin_course")]
    AdminCourseCard,
    #[at("/course_edit/:id")]
    CourseEdit{id:i32},
    #[at("/lesson/new")]
    CreateMaterial,
    #[at("/certs")]
    Certificates
}

#[derive(Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
  Admin,
  User,
  Teacher,
  #[default]
  None
}

impl Role {
  pub fn to_string(&self) -> String{
    match self {
        Role::Admin => "Администратор".to_string(),
        Role::Teacher => "Преподаватель".to_string(),
        Role::User => "Пользователь".to_string(),
        _ => unreachable!()
    }
  }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage/> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        Route::Register => html! { <RegisterPage/> },
        Route::MyCourses => html! { <CoursesList/>},
        Route::Login => html! {<LoginPage/>},
        Route::CourseInfo{id} => html! {<CourseInfo id={id}/>},
        Route::Lesson { course_id, lesson_id } => html!{<CourseMaterial course_id={course_id} lesson_id={lesson_id}/>},
        Route::MyMyCourses => html! {<MyCoursesList/>},
        Route::Profile => html! {<ProfileInfo/>},
        Route::UsersList => html!{<UsersList/>},
        Route::CourseEdit{id} => html! {<CourseEdit id={id}/>},
        Route::AdminCourseCard => html!{<AdminCourses />},
        Route::Certificates => html!{<Certificates />},
        Route::CreateMaterial => html! {<MaterialCreation />}
    }
}
#[function_component(Secure)]
fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}
#[function_component(App)]
pub fn app() -> Html {
  let (state, dispatch) = use_store::<LoginState>();
    let handle_exit = Callback::from(move|_| {
      dispatch.reduce_mut(|state| state.token =String::new());
    });
    log!(state.token.clone());
    html! {
    <BrowserRouter>
        <nav class="navbar" role="navigation" aria-label="main navigation">
  <div class="navbar-brand">
    <a class="navbar-item" href="/">
      <p>{"Главная"}</p>
      

    </a>

    <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample">
      <span aria-hidden="true"></span>
      <span aria-hidden="true"></span>
      <span aria-hidden="true"></span>
      <span aria-hidden="true"></span>
    </a>
  </div>
 { 
  if state.role == Role::User{
    html! {<div id="navbarBasicExample" class="navbar-menu">
      <div class="navbar-start">
        <a class="navbar-item">
          <Link<Route> to={Route::MyCourses} classes ="has-text-white">
              {"Курсы"}
              </Link<Route>>
        </a>
  
        <a class="navbar-item">
          <Link<Route> to={Route::MyMyCourses} classes ="has-text-white">
            {"Мои курсы"}
              </Link<Route>>
         
        </a>
        <a class="navbar-item">
          <Link<Route> to={Route::Certificates} classes ="has-text-white">
            {"Сертификаты"}
              </Link<Route>>
         
        </a>
      </div>
      </div>}
  }
  else {
    html! {}
  }
  }
  
      if state.role == Role::Admin{
<a class="navbar-item">
  <Link<Route> to={Route::UsersList} classes ="has-text-white">
    {"Пользователи"}
      </Link<Route>>
 
</a>
<div class="navbar-item has-dropdown is-hoverable">
  <a class="navbar-link">
    {"Курсы"}
  </a>

  <div class="navbar-dropdown">
    <a class="navbar-item">
      <Link<Route> to={Route::AdminCourseCard} classes ="has-text-white">
          {"Список курсов"}
          </Link<Route>>
    </a>
    <a class="navbar-item">
      <Link<Route> to={Route::CreateMaterial} classes ="has-text-white">
          {"Добавить урок к курсу"}
          </Link<Route>>
    </a>
    <a class="navbar-item">
      <Link<Route> to={Route::CourseEdit{id: -1}} classes ="has-text-white">
          {"Создать новый курс"}
          </Link<Route>>
    </a>
    <hr class="navbar-divider"/>
    <a class="navbar-item">
      {"Report an issue"}
    </a>
  </div>
</div>

      }
      
  

    <div class="navbar-end">
      <div class="navbar-item">
        <div class="buttons">
        {
          
            if state.token.clone().is_empty() {
              html!{
                <>
                <a class="button is-dark has-test-white">
                <Link<Route> to={Route::Register} classes ="has-text-white">
                {"Регистрация"}
                </Link<Route>>
              </a>
              <a class="button is-dark has-text-black">
              <Link<Route> to={Route::Login} classes ="has-text-black">
                {"Вход"}
                </Link<Route>>
              </a>
              </>
            }
           
            }
            else {
              html!{
                <>
                <a class="button is-dark has-text-black">
                  <Link<Route> to={Route::Profile} classes ="has-text-black">
                    {"Профиль"}
                    </Link<Route>>
                  </a>

                    <button class="button is-success" onclick={handle_exit}>
                      {"Выход"}
                    </button>

                    </>
              }
            }
        }
          
        </div>
      </div>
    </div>
</nav>

        <body>
                <Switch<Route> render={switch} />

        </body>

    </BrowserRouter>
    }
}
