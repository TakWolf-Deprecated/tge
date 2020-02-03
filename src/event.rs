
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    AppSuspend,
    AppResume,
    WindowClose,
}
