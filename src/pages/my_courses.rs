use std::ops::Deref;

use gloo_console::log;
use serde::{ Deserialize, Serialize };
use yew::{ function_component, prelude::* };
use yewdux::{ use_store, Store };

use crate::{ api::{ self, ApiCourseInfoData }, components::{course_card, my_course_card::MyCourseCard} };
pub struct MyCoursesList {
    courses: Option<ApiCourseInfoData>,
}
pub enum Msg {
    FetchCourses,
    SetCourses(ApiCourseInfoData),
}

impl Component for MyCoursesList {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        let data = Self {
            courses: None,
        };
        _ctx.link().send_message(Msg::FetchCourses);
        data
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // log!(self.courses.len());
        let data = self.courses.clone();
        log!(self.courses.is_some());
        html! {
        <div class="container is-fluid">
                    <div class="fixed-grid has-4-cols">
                    
                        {
                            if self.courses.clone().is_some() {
                                html! { 
                                <div class="grid">
                                <MyCourseCard id={self.courses.clone().unwrap().id as i128} description={self.courses.clone().unwrap().short_desc.unwrap().clone()} difficulty=1 title={self.courses.clone().unwrap().name.clone()} image_url={self.courses.clone().unwrap().image.unwrap().clone()}/> 
                            </div>}
                                   
                            }
                            else {
                                html!{}
                            }
                        }
                        // <MyCourseCard id = 0 title="C++ с нуля" description="Курс для базового ознакомления с одним из главных языков в программировании" difficulty=1 image_url="https://ip-calculator.ru/blog/wp-content/uploads/2021/02/6038586442907648.png"/>
                        // <MyCourseCard id = 1 title="Язык программирования Rust" description="Курс подойдет программистам любого уровня, которые хотят изучить новые возможности в программировании" difficulty=1 image_url="https://raw.githubusercontent.com/rust-lang/www.rust-lang.org/master/static/images/rust-social-wide-light.svg"/>
                        // <MyCourseCard id = 0 title="Лидерство в коллективе" description="Короткий курс с практическим обучением. Без воды и долгих речей. Только основные навыки." difficulty=1 image_url="https://static.tildacdn.com/tild3039-3134-4032-a637-663335343332/-7-.jpg"/>
                        // <MyCourseCard id = 1 title="Программирование на C#: от новичка до специалиста" description="Изучите C# и платформу .NET, включая .NET Core и начните практиковать объектно-ориентированное программирование (ООП)." difficulty=1 image_url="https://248006.selcdn.ru/main/iblock/9dd/9dd3c730431af9dbab7befd41814fcea/13eaa548ad580af37b96dfbd474da753.png"/>
                     
                    </div>
                 </div>
                }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchCourses => {
                ctx.link().send_future(async {
                    let resp = api::api_get_course_info(2.into()).await;
                    return Msg::SetCourses(resp);
                });

            }
            Msg::SetCourses(data) => {
                self.courses = Some(data.clone())
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
