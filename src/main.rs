use eval::{eval, to_value};

slint::slint! {
    import { VerticalBox } from "std-widgets.slint";

    export global CalcLogic {
        callback button-pressed(string);
    }

    component Button {
        in property <string> text;
        min-height: 50px;
        min-width: 50px;
        in property <brush> background: @linear-gradient(-20deg, #a0a3e4, #3c58e3);
        Rectangle {
            background: ta.pressed ? red : ta.has-hover ? background.darker(10%) : background;
            animate background { duration: 100ms; }
            border-radius: 4px;
            border-width: 2px;
            border-color: self.background.darker(20%);
            ta := TouchArea {
                clicked => { CalcLogic.button-pressed(root.text) }
            }
        }
        Text { text: root.text; }
    }

    export component App inherits Window {
        in-out property <string> expr: " ";
        GridLayout {
            padding: 10px;
            spacing: 5px;
            Text { text: expr; colspan: 3; font-size: 26px; }
            Row {
                Button { text: "C"; background: pink; }
            }
            Row {
                Button { text: "1"; }
                Button { text: "2"; }
                Button { text: "3"; }
                Button { text: "+"; background: yellow; }
            }
            Row {
                Button { text: "4"; }
                Button { text: "5"; }
                Button { text: "6"; }
                Button { text: "-"; background: yellow; }
            }
            Row {
                Button { text: "7"; }
                Button { text: "8"; }
                Button { text: "9"; }
                Button { text: "*"; background: yellow; }
            }
            Row {
                Button { text: "."; }
                Button { text: "0"; }
                Button { text: "="; background: green; }
                Button { text: "/"; background: yellow; }
            }
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = App::new()?;
    let ui_handle = ui.as_weak();

    ui.global::<CalcLogic>().on_button_pressed(move |value| {
        let ui = ui_handle.unwrap();
        let expr = ui.get_expr();
        ui.set_expr(if value == "C" {
            " ".into()
        } else if value == "=" {
            eval(&expr).unwrap_or(to_value("Error")).to_string().into()
        } else {
            format!("{}{}", expr, value).into()
        });
    });

    ui.run()
}
