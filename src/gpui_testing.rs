use gpui::*;
//use docx-rust::*
struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld{
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement { //self vs Self???
        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}", &self.text))
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| { // I think cx refers to the name of the app???
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| HelloWorld { //HelloWorld struct passed into new_view
                //research
                text: "World".into(),
            })
        })
        .unwrap(); // i missed rust sm, fuck cpp //no supported device found error, will figure out
        // later
    })
}
