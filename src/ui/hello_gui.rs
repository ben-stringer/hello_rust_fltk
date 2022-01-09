use fltk::app::{Receiver, Sender};
use fltk::button::Button;
use fltk::enums::{Align, Key, Shortcut};
use fltk::group::Flex;
use fltk::text::{TextBuffer, TextDisplay};
use fltk::{prelude::*, *};

use crate::model::rpn_calc::RpnCalc;

#[derive(Copy, Clone, Debug)]
enum Op {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Copy, Clone, Debug)]
enum Message {
    NumPressed(i8),
    OpPressed(Op),
    Decimal,
    ClearItem,
    ClearStack,
    Enter,
    Backspace,
}

pub(crate) struct HelloGui {
    app: app::App,
    model: RpnCalc,
    receiver: Receiver<Message>,
    stack_view: TextBuffer,
}

impl HelloGui {
    pub fn new(model: RpnCalc) -> Self {
        log::info!("Constructing app.");

        let app = app::App::default();
        let (sender, receiver) = app::channel::<Message>();

        let stack_view = Self::build_ui(sender);

        Self {
            app,
            model,
            receiver,
            stack_view,
        }
    }

    fn build_ui(sender: Sender<Message>) -> TextBuffer {
        let mut wind = window::Window::default().with_size(400, 400);
        wind.make_resizable(true);

        let flex_col = Flex::default().size_of_parent().column();

        let stack_view = {
            let flex_row = Flex::default().row();
            let mut disp = TextDisplay::default();
            disp.set_buffer(TextBuffer::default());
            flex_row.end();
            disp.buffer().expect("Failed to get the text buffer back")
        };

        {
            let flex_row = Flex::default().row();
            HelloGui::build_btn(
                sender,
                "@<",
                Message::Backspace,
                Shortcut::None | Key::BackSpace,
            );
            HelloGui::build_btn(
                sender,
                "C",
                Message::ClearItem,
                Shortcut::None | Key::Delete,
            );
            HelloGui::build_btn(
                sender,
                "AC",
                Message::ClearStack,
                Shortcut::None | Key::Escape,
            );
            flex_row.end();
        }
        {
            let flex_row = Flex::default().row();
            HelloGui::build_btn(sender, "7", Message::NumPressed(7), Shortcut::None | '7');
            HelloGui::build_btn(sender, "8", Message::NumPressed(8), Shortcut::None | '8');
            HelloGui::build_btn(sender, "9", Message::NumPressed(9), Shortcut::None | '9');
            HelloGui::build_btn(
                sender,
                "/",
                Message::OpPressed(Op::Divide),
                Shortcut::None | '/',
            );
            flex_row.end();
        }
        {
            let flex_row = Flex::default().row();
            HelloGui::build_btn(sender, "4", Message::NumPressed(4), Shortcut::None | '4');
            HelloGui::build_btn(sender, "5", Message::NumPressed(5), Shortcut::None | '5');
            HelloGui::build_btn(sender, "6", Message::NumPressed(6), Shortcut::None | '6');
            HelloGui::build_btn(
                sender,
                "*",
                Message::OpPressed(Op::Multiply),
                Shortcut::None | '*',
            );
            flex_row.end();
        }
        {
            let flex_row = Flex::default().row();
            HelloGui::build_btn(sender, "1", Message::NumPressed(1), Shortcut::None | '1');
            HelloGui::build_btn(sender, "2", Message::NumPressed(2), Shortcut::None | '2');
            HelloGui::build_btn(sender, "3", Message::NumPressed(3), Shortcut::None | '3');
            HelloGui::build_btn(
                sender,
                "-",
                Message::OpPressed(Op::Minus),
                Shortcut::None | '-',
            );
            flex_row.end();
        }
        {
            let flex_row = Flex::default().row();
            HelloGui::build_btn(sender, "0", Message::NumPressed(0), Shortcut::None | '0');
            HelloGui::build_btn(sender, ".", Message::Decimal, Shortcut::None | '.');
            HelloGui::build_btn(sender, "Enter", Message::Enter, Shortcut::None | Key::Enter);
            HelloGui::build_btn(
                sender,
                "+",
                Message::OpPressed(Op::Plus),
                Shortcut::None | '+',
            );
            flex_row.end();
        }

        flex_col.end();

        wind.end();
        wind.show();

        stack_view
    }

    fn build_btn(sender: Sender<Message>, lbl: &str, msg: Message, shortcut: Shortcut) -> Button {
        let mut btn = button::Button::default().with_label(lbl).center_of_parent();
        btn.emit(sender, msg);
        btn.set_shortcut(shortcut);
        btn
    }

    fn view_changed(&mut self) {
        self.stack_view.set_text(&self.model.to_string());
    }

    fn handle_op(&mut self, op: Op) -> bool {
        log::debug!("Op pressed: {:?}", op);
        match op {
            Op::Plus => self.model.do_add(),
            Op::Minus => self.model.do_subtract(),
            Op::Multiply => self.model.do_multiply(),
            Op::Divide => self.model.do_divide(),
        }
    }

    pub fn run(&mut self) {
        log::info!("Running app");
        while self.app.wait() {
            if let Some(msg) = self.receiver.recv() {
                if match msg {
                    Message::OpPressed(op) => self.handle_op(op),
                    Message::NumPressed(i) => self.model.update_current(i),
                    Message::Enter => self.model.enter_current(),
                    Message::Decimal => self.model.handle_decimal(),
                    Message::ClearItem => self.model.clear_current(),
                    Message::ClearStack => self.model.clear_stack(),
                    Message::Backspace => self.model.backspace(),
                } {
                    self.view_changed();
                }
            }
        }
    }

    pub fn get_model(&self) -> &RpnCalc {
        &self.model
    }
}
