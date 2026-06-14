pub enum InjectionMethod {
    Uinput,
    Clipboard,
    TmuxSendKeys,
    KittyRemote,
    Stdout,
}

pub trait Injector: Send {
    fn inject(&self, text: &str) -> anyhow::Result<()>;
    fn method(&self) -> InjectionMethod;
}
