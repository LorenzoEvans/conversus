use state::{Conversation};
#[derive(Debug, Clone, Copy)]
pub enum AgoraMessage {
    ConversationAdded(Conversation),
    NameChanged(String),
    DescChanged(String),
}

pub enum ConversationMessage {
    PostAdded(String),
    PostDeleted,
}

pub enum UserMessage {
    KindChange,
    UserNameChange,
    EmailChange,
    PasswordChange,
    EnteredConversation,
    ExitedConversation,
    AddedComment,
    RemovedComment
}