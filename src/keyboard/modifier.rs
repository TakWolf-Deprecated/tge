
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct ModifiersState {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub logo: bool,
}

impl From<winit::event::ModifiersState> for ModifiersState {

    fn from(state: winit::event::ModifiersState) -> Self {
        Self {
            shift: state.shift(),
            ctrl: state.ctrl(),
            alt: state.alt(),
            logo: state.logo(),
        }
    }

}
