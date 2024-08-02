use gloo_console::log;
use serde::{ Deserialize, Serialize };
use web_sys::HtmlInputElement;
use yew::{ function_component, html::IntoHtmlResult, prelude::* };
use yewdux::{ use_store, Store };
use wasm_bindgen::JsCast;

use crate::api::{ self, ApiCourseInfoData };
#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i128,
}

const INFO: &str =
    r#"
Добро пожаловать в *The Rust Programming Language*, вводную книгу о Rust. Язык программирования Rust помогает создавать быстрые, более надёжные приложения. Хорошая эргономика и низкоуровневый контроль часто являются противоречивыми требованиями для дизайна языков программирования; Rust бросает вызов этому конфликту. Благодаря сбалансированности мощных технических возможностей c большим удобством разработки, Rust предоставляет возможности управления низкоуровневыми элементами (например, использование памяти) без трудностей, традиционно связанных с таким контролем.

## Кому подходит Rust

Rust идеально подходит для многих людей по целому ряду причин. Давайте рассмотрим несколько наиболее важных групп.

### Команды разработчиков

Rust зарекомендовал себя как продуктивный инструмент для совместной работы больших команд разработчиков с разным уровнем знаний в области системного программирования. Низкоуровневый код подвержен различным трудноуловимым ошибкам, которые в большинстве других языков могут быть обнаружены только с помощью тщательного тестирования и проверки кода опытными разработчиками. В Rust компилятор играет роль привратника, отказываясь компилировать код с этими неуловимыми ошибками, включая ошибки параллелизма. Работая вместе с компилятором, команда может сфокусироваться на работе над логикой программы, а не над поиском ошибок.

Rust также привносит современные инструменты разработчика в мир системного программирования:

- Cargo, входящий в комплект менеджер зависимостей и инструмент сборки, делает добавление, компиляцию и управление зависимостями безболезненным и согласованным в рамках всей экосистемы Rust.
- Инструмент форматирования Rustfmt обеспечивает единый стиль кодирования для всех разработчиков.
- Rust Language Server обеспечивает интеграцию с интегрированной средой разработки (IDE) для автодополнения кода и встроенных сообщений об ошибках.

Благодаря применению этих и других инструментов в экосистеме Rust разработчики способны продуктивно работать при написании кода системного уровня.

### Студенты

Rust полезен для студентов и тех, кто заинтересован в изучении  системных концепций. Используя Rust, многие люди узнали о таких темах, как разработка операционных систем. Сообщество радушно и с удовольствием ответит на вопросы начинающих. Благодаря усилиям — таким, как эта книга — команды Rust хотят сделать концепции систем более доступными для большего числа людей, особенно для новичков в программировании.

### Компании

Сотни больших и малых компаний используют Rust в промышленных условиях для решения различных задач, включая инструменты командной строки, веб-сервисы, инструменты DevOps, встраиваемые устройства, анализ и транскодирование аудио и видео, криптовалюты, биоинформатику, поисковые системы, приложения Интернета вещей, машинное обучение и даже основные части веб-браузера Firefox.

### Разработчики Open Source

Rust предназначен для людей, которые хотят развивать язык программирования Rust, сообщество, инструменты для разработчиков и библиотеки. Мы будем рады, если вы внесёте свой вклад в развитие языка Rust.

### Люди, ценящие скорость и стабильность

Rust предназначен для любителей скорости и стабильности в языке. Под скоростью мы подразумеваем как быстродействие программы на Rust, так и быстроту, с которой Rust позволяет писать программы. Проверки компилятора Rust обеспечивают стабильность за счёт функциональных дополнений и рефакторинга. Это выгодно отличается от хрупкого унаследованного кода в языках без таких проверок, который разработчики часто боятся изменять. Благодаря обеспечению абстракций с нулевой стоимостью, высокоуровневых возможностей, компилируемых в низкоуровневый код такой же быстрый, как и написанный вручную, Rust стремится сделать безопасный код ещё и быстрым.

Язык Rust надеется поддержать и многих других пользователей; перечисленные здесь - лишь самые значимые заинтересованные лица. В целом, главная цель Rust - избавиться от компромиссов, на которые программисты шли десятилетиями, обеспечив безопасность *и* производительность, скорость *и* эргономичность. Попробуйте Rust и убедитесь, подойдут ли вам его решения.

## Для кого эта книга

В этой книге предполагается, что вы писали код на другом языке программирования, но не оговаривается, на каком именно. Мы постарались сделать материал доступным для широкого круга людей с разным уровнем подготовки в области программирования. Мы не будем тратить время на обсуждение *сути понятия* программирования или как его понимать. Если вы совсем новичок в программировании, рекомендуем прочитать книгу, посвящённую введению в программирование.

## Как использовать эту книгу

В целом, книга предполагает, что вы будете читать последовательно от начала до конца. Более поздние главы опираются на концепции, изложенные в предыдущих главах, а предыдущие главы могут не углубляться в детали конкретной темы, так как в последующих главах они будут рассматриваться более подробно.

В этой книге вы найдёте два вида глав: главы о концепциях и главы с проектом. В главах о концепциях вы узнаете о каком-либо аспекте Rust. В главах проекта мы будем вместе создавать небольшие программы, применяя то, что вы уже узнали. Главы 2, 12 и 20 - это главы проекта; остальные - главы о концепциях.

Глава 1 объясняет, как установить Rust, как написать программу "Hello, world!" и как использовать Cargo, менеджер пакетов и инструмент сборки Rust. Глава 2 - это практическое введение в написание программы на Rust, в которой вам предлагается создать игру для угадывания чисел. Здесь мы рассмотрим концепции на высоком уровне, а в последующих главах будет предоставлена дополнительная информация. Если вы хотите сразу же приступить к работе, глава 2 - самое подходящее место для этого. В главе 3 рассматриваются возможности Rust, схожие с возможностями других языков программирования, а в главе 4 вы узнаете о системе владения Rust. Если вы особенно дотошный ученик и предпочитаете изучить каждую деталь, прежде чем переходить к следующей, возможно, вы захотите пропустить главу 2 и сразу перейти к главе 3, вернувшись к главе 2, когда захотите поработать над проектом, применяя изученные детали.

Глава 5 описывает структуры и методы, а глава 6 охватывает перечисления, выражения `match` и конструкции управления потоком `if let`. Вы будете использовать структуры и перечисления для создания пользовательских типов в Rust.

В главе 7 вы узнаете о системе модулей Rust, о правилах организации приватности вашего кода и его публичном интерфейсе прикладного программирования (API). В главе 8 обсуждаются некоторые распространённые структуры данных - коллекции, которые предоставляет стандартная библиотека, такие как векторы, строки и HashMaps. В главе 9 рассматриваются философия и методы обработки ошибок в Rust.

В главе 10 рассматриваются шаблонные типы данных, типажи и времена жизни, позволяющие написать код, который может использоваться разными типами. Глава 11 посвящена тестированию, которое даже с гарантиями безопасности в Rust необходимо для обеспечения правильной логики вашей программы. В главе 12 мы создадим собственную реализацию подмножества функциональности инструмента командной строки `grep`, предназначенного для поиска текста в файлах. Для этого мы будем использовать многие концепции, которые обсуждались в предыдущих главах.

В главе 13 рассматриваются замыкания и итераторы: особенности Rust, пришедшие из функциональных языков программирования. В главе 14 мы более подробно рассмотрим Cargo и поговорим о лучших методах распространения ваших библиотек среди других разработчиков. В главе 15 обсуждаются умные указатели, которые предоставляет стандартная библиотека, и типажи, обеспечивающие их функциональность.

В главе 16 мы рассмотрим различные модели параллельного программирования и поговорим о возможности Rust для безбоязненного многопоточно программирования. В главе 17 рассматривается сравнение идиом Rust с принципами объектно-ориентированного программирования, которые наверняка вам знакомы.

Глава 18 - это справочник по шаблонам и сопоставлению с образцами, которые являются мощными способами выражения идей в программах на Rust. Глава 19 содержит множество интересных дополнительных тем, включая небезопасный Rust, макросы и многое другое о времени жизни, типажах, типах, функциях и замыканиях.

В главе 20 мы завершим проект, в котором реализуем низкоуровневый многопоточный веб-сервер!

Наконец, некоторые приложения содержат полезную информацию о языке в более справочном формате. В приложении A рассматриваются ключевые слова Rust, в приложении B — операторы и символы Rust, в приложении C — производные типажи, предоставляемые стандартной библиотекой, в приложении D — некоторые полезные инструменты разработки, а в приложении E — издания Rust. В приложении F вы найдёте переводы книги, а в приложении G мы расскажем о том, как создаётся Rust и что такое nightly Rust.

Нет неправильного способа читать эту книгу: если вы хотите пропустить главу - сделайте это! Возможно, вам придётся вернуться к предыдущим главам, если возникнет недопонимание. Делайте все, как вам удобно.


В большинстве случаев мы приведём вас к правильной версии любого кода, который не компилируется.

## Исходные коды

Файлы с исходным кодом, используемым в этой книге, можно найти на [GitHub].


[Язык программирования Rust]: https://nostarch.com/rust-programming-language-2nd-edition
[No Starch Press]: https://nostarch.com/
[GitHub]: https://github.com/rust-lang/book/tree/main/src
"#;

pub struct CourseInfo {
    image: String,
    content: String,
    short_desc: String,
    id: i128,
}
pub enum Msg {
    FetchContent,
    SetContent(ApiCourseInfoData),
}

impl Component for CourseInfo {
    type Message = Msg;
    type Properties = Props;
    fn create(_ctx: &Context<Self>) -> Self {
        let data = Self {
            image: String::new(),
            content: String::new(),
            short_desc: String::new(),
            id: _ctx.props().id,
        };
        _ctx.link().send_message(Msg::FetchContent);
        data
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let markdown_input = self.content.clone();
        let parser = pulldown_cmark::Parser::new(&markdown_input);
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);
        let val = Html::from_html_unchecked(html_output.into());
        html! { <>
            <section class="section">
               <div class="columns is-vcentered">
                <div class="column is-half">
                    <article class="message is-dark">
          <div class="message-header">
            <p>{"Язык программирования Rust"}</p>
            // <button class="delete" aria-label="delete"></button>
          </div>
          <div class="message-body">
           {self.short_desc.clone()}
          </div>
        </article>
        <button class="button is-dark is-centered">
        {"Записаться"}
        </button>
                
                </div>
                <div class="column is-half">
                <div class="box">
                <figure class="image is-2by1">
                
                <img  src={self.image.clone()}/>
                </figure>
                </div>
               </div>
               </div>
        </section>
            <section class="section course-info-content">
            
            
         
            <div class="columns"> 
                <div class="column is-two-thirds">
                <article class="message">
                    <div class="message-header">
                <p>{"Язык программирования Rust"}</p>
                // <button class="delete" aria-label="delete"></button>
                </div>
                    <div class="message-body">
                        {val}
                    </div>
                    
                </article>
                </div>
                <div class="column">
                    <article class="message">
                    <div class="message-header">
                <p>{"Отзывы"}</p>
                // <button class="delete" aria-label="delete"></button>
                </div>
                    <div class="message-body">
                        <div class="fixed-grid has-1-cols">
                    <div class="grid  is-row-gap-7.5">
                        <div class="cell">
                            <div class="box">{
                                if self.id == 1 {
                                    html! {
                                    <article class="media">
                                        <figure class="media-left">
                                            <p class="image is-64x64">
                                            <img src="https://bulma.io/assets/images/placeholders/128x128.png" />
                                            </p>
                                        </figure>
                                        
                                        <div class="media-content">
                                            <div class="content">
                                            <p><strong>{"Александр Петров"}</strong></p>
                                            <p>
                                                {"Курс очень помог разобраться в основах языка Rust. Стало многое понятно после изучения всех глав и прохождения финального теста!"}
                                            </p>
                                            </div>
                                        </div>
                                        </article>
                                        }
                                }
                                else {
                                    html!{<p>{"Пока что никто не оставил отзыв к этому курсу :("}</p>}
                                }
                            }
                            </div>
                            
                        </div>
                    </div>
                    </div>
                    </div>
                    
                </article>
                    
                    
                </div>
                
                </div>
            </section> 
            </>
                }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchContent => {
                let id = self.id;
                ctx.link().send_future(async move {
                    let resp = api::api_get_course_info(id.into()).await;
                    return Msg::SetContent(resp);
                });
            }
            Msg::SetContent(data) => {
                self.image = data.image.unwrap().clone();
                self.content = data.description.unwrap().clone();
                self.short_desc = data.short_desc.unwrap().clone();
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

    fn destroy(&mut self, ctx: &Context<Self>) {}
}

// #[function_component]
// pub fn CourseInfo(props: &Props) -> Html {
//     let id = props.id;
//     wasm_bindgen_futures::spawn_local(async move {
//         let resp = api::api_get_course_info(
//             id
//         ).await;
//     });

//     html! {

//     }
// }
