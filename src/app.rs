use std::str::{FromStr};

use color_art::Color;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use verdigris_ui::components::{
    display::{alert::*, badge::*, card::*},
    input::{button::*, icon_button::*},
    layout::{grid::*, flex::*, space::*, stack::*, group::*, group::*, Direction, Justify, Align, Wrap, Position},
    misc::pattern::*,
    misc::shell::*,
    typography::*,
    Size,
    Variant,
};
use leptos_icons::Icon;
use leptos_icons::CgIcon;
use leptos_icons::LeptosIcon;
use leptos_icons::LeptosIconProps;
use verdigris_ui::theme::HighlightColor;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="components" view=|cx| view! { cx, <ComponentsPage/> }/>
                    <Route path="start" view=|cx| view! { cx, <StartPage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Verdigris UI"</h1>
        <h2>"Simple Leptos component library"</h2>
        <Pattern>
            <Flex>
                <a href="components">
                    <Button variant=Variant::Gradient(Color::from_str("#3eafa8").unwrap(), Color::from_str("#4bd5cc").unwrap(), 45)
                        size=Size::ExtraLarge
                    >
                    "Learn more"</Button>
                </a>
            </Flex>
        </Pattern>
    }
}

#[component]
fn ComponentsPage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <HeaderBar>
            <Group position=Position::Left>"Verdigris UI"</Group>
            <Group position=Position::Left gap=Size::Medium>
                <Button variant=Variant::Outline>"See examples"</Button>
                <IconButton><LeptosIcon icon=Icon::Cg(CgIcon::CgSoftwareDownload) width="18" height="18"/></IconButton>
            </Group>
        </HeaderBar>
        <h1>"Verdigris UI"</h1>
        <h2>"Grid"</h2>
        <Grid>
            <div class="preview-background">
                <div style="padding:1rem;">
                    <Stack>
                        <h2>"Card"</h2>
                        <Card padding=Size::Medium>
                            <p>"This is a card"</p>
                        </Card>
                        <Card padding=Size::Medium hover_tilt=true>
                            <p>"This is a card has "<Code>"hover-tilt"</Code>" attribute"</p>
                        </Card>
                        <Card padding=Size::Medium border=true>
                            <p>"This is a card with border"</p>
                        </Card>
                    </Stack>
                </div>
            </div>
            <div>
                <h2>"Alert"</h2>
                <Alert variant=Variant::Light title="Bummer!">
                    <p>"This is an alert!"</p>
                </Alert>
                <div>
                    "Test"
                </div>
            </div>
            <div>
                <h2>"Button"</h2>
                <Button variant=Variant::Filled>"Filled Button"</Button><Space width=Size::Medium/>
                <Button variant=Variant::Outline>"Outline Button"</Button><Space width=Size::Medium/>
                <Button variant=Variant::Light>"Light Button"</Button><Space width=Size::Medium/>
                <Button variant=Variant::Subtle>"Subtle Button"</Button><Space width=Size::Medium/>
                <Button variant=Variant::Gradient(Color::from_str("#3eafa8").unwrap(), Color::from_str("#4bd5cc").unwrap(), 60)>"Gradient Button"</Button><Space width=Size::Medium/>
                <br/><Space height=Size::Medium/><br/>
                <Button variant=Variant::Filled compact=true>"Filled Button"</Button><Space width=Size::Medium/>
                <Button variant=Variant::Outline compact=true>"Outline Button"</Button><Space width=Size::Medium/>
                <Button variant=Variant::Light compact=true>"Light Button"</Button><Space width=Size::Medium/>
                <Button variant=Variant::Subtle compact=true>"Subtle Button"</Button><Space width=Size::Medium/>
                <Button variant=Variant::Gradient(Color::from_str("#3eafa8").unwrap(), Color::from_str("#4bd5cc").unwrap(), 60) compact=true>"Gradient Button"</Button><Space width=Size::Medium/>
                <br/><Space height=Size::Medium/><br/>
                <Group>
                    <Button variant=Variant::Filled>
                        <ButtonLeft>
                            <LeptosIcon icon=Icon::Cg(CgIcon::CgSoftwareDownload) width="18" height="18"/>
                        </ButtonLeft>
                        "With Icon (Left)"
                    </Button>
                    <Button variant=Variant::Outline>
                        <ButtonRight>
                            <LeptosIcon icon=Icon::Cg(CgIcon::CgSoftwareDownload) width="18" height="18"/>
                        </ButtonRight>
                        "With Icon (Right)"
                    </Button>
                    <Button variant=Variant::Light>
                        <ButtonLeft>
                            <LeptosIcon icon=Icon::Cg(CgIcon::CgSoftwareDownload) width="18" height="18"/>
                        </ButtonLeft>
                        <ButtonRight>
                            <LeptosIcon icon=Icon::Cg(CgIcon::CgSoftwareDownload) width="18" height="18"/>
                        </ButtonRight>
                        "With Icon (Both)"
                    </Button>
                </Group>
            </div>
            <div>
                <h2>"Badge"</h2>
                <Badge variant=BadgeVariant::Filled>"Filled"</Badge><Space width=Size::Medium/>
                <Badge variant=BadgeVariant::Outline>"Outline"</Badge><Space width=Size::Medium/>
                <Badge variant=BadgeVariant::Light>"Light"</Badge><Space width=Size::Medium/>
                <Badge variant=BadgeVariant::Dot>"Dot"</Badge><Space width=Size::Medium/>
            </div>
            <Pattern>
                <Flex>
                    <h2>"Flex"</h2>
                    <Space width=Size::Medium/>
                    "and Pattern"
                </Flex>
            </Pattern>

            <div>
                <h2>"Typography"</h2>
                <h1>"Heading 1"</h1>
                <h2>"Heading 2"</h2>
                <h3>"Heading 3"</h3>
                <h4>"Heading 4"</h4>
                <h5>"Heading 5"</h5>
                <h6>"Heading 6"</h6>
                <p>"Paragraph"</p>
                <Mark>"Mark"</Mark>
                <Space height=Size::Medium/>
                <Code>"Code"</Code>
                <Space height=Size::Medium/>
                "This "<Mark color=HighlightColor::Purple>"is example"</Mark>" "<Mark>"text"</Mark>" with "<Mark color=HighlightColor::Blue>"markings"</Mark>" and custom "<Code color=HighlightColor::Orange>"inline code block"</Code>""
            </div>
        </Grid>
        <Button on_click=on_click variant=Variant::Filled>"Click Me: " {count}</Button>
    }
}

#[component]
fn StartPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <HeaderBar>
            <Group position=Position::Left>"Verdigris UI"</Group>
            <Group position=Position::Left gap=Size::Medium>
                <Button variant=Variant::Outline>"See examples"</Button>
                <IconButton><LeptosIcon icon=Icon::Cg(CgIcon::CgSoftwareDownload) width="18" height="18"/></IconButton>
            </Group>
        </HeaderBar>
        <div style="text-align: left;">
            <h1>"Verdigris UI"</h1>
            <h2>"Get started with cargo"</h2>
            <p>"Add to your project:"</p>
            <CodeBlock>
                <CodeBlockTab title="cli">
r##"verdigris-ui = { git = "https://github.com/DcZipPL/verdigris-ui.git" }"##
                </CodeBlockTab>
                <CodeBlockTab title="cargo.toml">
"cargo install verdigris-ui"
                </CodeBlockTab>
            </CodeBlock>
            <p>"Set default styles:"</p>
            <CodeBlock>
r##"use verdigris_ui::theme::Theme;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // [...]

    view! {
        cx,
        // [...]

        &ltVerdigrisThemeProvider theme=Theme::default()/&gt

        // [...]
    }
}
"##
            </CodeBlock>
        </div>
    }
}