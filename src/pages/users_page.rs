use std::ops::Deref;

use gloo_console::log;
use serde::{ Deserialize, Serialize };
use yew::{ function_component, prelude::* };
use yewdux::{ use_store, Store };
use crate::components::user_info::UserInfo;

use crate::{ api::{ self, ApiCourseInfoData, User }, components::course_card::CourseCard };
pub struct UsersList {
    users: Vec<User>,
}
pub enum Msg {
    FetchUsers,
    SetUsers(Vec<User>),
}

impl Component for UsersList {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        let data = Self {
            users: Vec::new(),
        };
        _ctx.link().send_message(Msg::FetchUsers);
        data
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log!(self.users.len());
        let users = self.users.iter().map(|data| {
            html! { <UserInfo email={data.email.clone()} name={data.first_name.clone()} surname={data.last_name.clone()} role={data.role.clone().to_string()}/> }
        });
        html! {
        <div class="container ">
        <div class="box">
            <div class="fixed-grid has-1-cols">
            <div class="grid">
                <div class="cell is-info">
                <div class="columns">
                    <div class="column">{"Имя"}</div>
                    <div class="column">{"Фамилия"}</div>
                    <div class="column">{"Роль"}</div>
                    <div class="column">{"Почта"}</div>
                    <div class="column">{""}</div>
                </div>
                </div>
                {
                    for users
                }
                // <UserInfo email={"ivanick@gmail.com"} name={"Иван"} surname={"Иванов"} role={"Администратор"}/>
            </div>
            </div>
            </div>
                    </div>
                }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchUsers => {
                ctx.link().send_future(async {
                    let resp = api::api_get_users().await;
                    return Msg::SetUsers(resp);
                });

            }
            Msg::SetUsers(data) => {
                self.users.extend(data);
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        self.users.clear()
    }
}

// #[function_component]
// pub fn MyCourses() -> Html {
//     let mut data = Vec::<ApiCourseInfoData>::new();
//         wasm_bindgen_futures::spawn_local(async move {
//             let resp = api::api_get_courses().await;
//             data.extend(resp);
//         });

//         let course_cards = data.into_iter().map(|data| {
//             html! {<CourseCard id={data.id as i128} description={data.description.unwrap()} difficulty=1 title={data.title} image_url="https://i.ytimg.com/vi/Y_tdU51NGq8/mqdefault.jpg"/>}
//         });
//     html! {
//         <div class="fixed-grid has-4-cols">
//             <div class="grid">
//             {
//                 for course_cards
//             }
//             // <CourseCard id = 0 title="Мастерство долбоебизма" description="Курс подойдет ебанатам и долбоебам, полным хуесосам и хуеглотам, идите нахуй" difficulty=1 image_url="https://i.ytimg.com/vi/Y_tdU51NGq8/mqdefault.jpg"/>
//             // <CourseCard id = 1 title="blya_suc" description="epta disk" difficulty=1 image_url="https://user-images.githubusercontent.com/5607145/195447840-72047b6e-81a4-419e-96c6-3460d229b2ad.png"/>
//         </div>
//         </div>
//     }
// }
