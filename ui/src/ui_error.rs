use strum_macros::IntoStaticStr;

#[derive(Debug, IntoStaticStr)]
pub enum UiErrorMsgId {
    NotInitialized,
}

#[derive(Debug, IntoStaticStr)]
pub enum UiErrorTopicId {
    SdlRender,
}