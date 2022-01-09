use crate::model::rpn_calc::EditMode::AppendWhole;

#[derive(Copy, Clone, Debug)]
enum EditMode {
    AppendWhole,
    AppendFractional(f64),
    PushNext,
}

pub(crate) struct RpnCalc {
    stack: Vec<f64>,
    current: f64,
    edit_mode: EditMode,
}

impl RpnCalc {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            current: 0.0,
            edit_mode: AppendWhole,
        }
    }

    pub fn enter_current(&mut self) -> bool {
        let current = self.current;
        log::debug!("Pushing {} onto stack.", current);
        self.stack.push(current);
        self.current = 0.0;
        self.edit_mode = EditMode::AppendWhole;
        true
    }

    pub fn update_current(&mut self, digit: i8) -> bool {
        log::debug!("NumPressed: {}", digit);
        match self.edit_mode {
            EditMode::AppendFractional(shift) => {
                self.current += digit as f64 * shift;
                self.edit_mode = EditMode::AppendFractional(shift * 0.1);
            }
            EditMode::AppendWhole => {
                self.current = self.current * 10.0 + digit as f64;
            }
            EditMode::PushNext => {
                self.enter_current();
                self.current = digit as f64;
                self.edit_mode = EditMode::AppendWhole;
            }
        }
        true
    }

    pub fn handle_decimal(&mut self) -> bool {
        match self.edit_mode {
            EditMode::AppendFractional(_) => {
                log::warn!("Pressed decimal; decimal portion already activated; ignoring.");
                false
            }
            EditMode::AppendWhole => {
                self.edit_mode = EditMode::AppendFractional(0.1);
                true
            }
            EditMode::PushNext => {
                self.enter_current();
                self.edit_mode = EditMode::AppendFractional(0.1);
                true
            }
        }
    }

    pub fn do_add(&mut self) -> bool {
        if self.stack.is_empty() {
            log::warn!("Nothing on the stack; ignoring.");
            false
        } else {
            let l = self.stack.pop().unwrap();
            let r = self.current;
            self.current = l + r;
            self.edit_mode = EditMode::PushNext;
            true
        }
    }

    pub fn do_subtract(&mut self) -> bool {
        if self.stack.is_empty() {
            log::warn!("Nothing on the stack; ignoring.");
            false
        } else {
            let l = self.stack.pop().unwrap();
            let r = self.current;
            self.current = l - r;
            self.edit_mode = EditMode::PushNext;
            true
        }
    }

    pub fn do_multiply(&mut self) -> bool {
        if self.stack.is_empty() {
            log::warn!("Nothing on the stack; ignoring.");
            false
        } else {
            let l = self.stack.pop().unwrap();
            let r = self.current;
            self.current = l * r;
            self.edit_mode = EditMode::PushNext;
            true
        }
    }

    pub fn do_divide(&mut self) -> bool {
        if self.stack.is_empty() {
            log::warn!("Nothing on the stack; ignoring.");
            false
        } else {
            if f64::abs(*self.stack.last().unwrap()) < f64::MIN_POSITIVE {
                log::warn!("Avoiding a divide-by-zero error.");
                false
            } else {
                let denom = self.current;
                let num = self.stack.pop().unwrap();
                self.current = num / denom;
                self.edit_mode = EditMode::PushNext;
                true
            }
        }
    }

    pub fn backspace(&mut self) -> bool {
        log::warn!("Backspace is hard; currently not implemented.");
        false
    }

    pub fn clear_current(&mut self) -> bool {
        self.current = 0.0;
        true
    }

    pub fn clear_stack(&mut self) -> bool {
        self.current = 0.0;
        self.stack.clear();
        true
    }

    pub fn to_string(&self) -> String {
        let mut stack = self
            .stack
            .iter()
            .map(f64::to_string)
            .collect::<Vec<String>>();
        stack.push(self.current.to_string());

        stack.join("\n")
    }
}
