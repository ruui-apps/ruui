
// TODO look if there is a better way to handle or generate this. Would be great if we could allow
// expansion plugins for others to make components easier.
pub enum WidgetKind {
    Window,
    Column,
    Row,
    Text,
    Button,
    ScrollView,
}

pub struct WidgetId(pub u64);
pub struct EventId(pub u64);

pub trait NativeComponent {
    fn create() -> Self;
}

