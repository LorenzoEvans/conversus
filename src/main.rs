
use iced::{
    button, scrollable, text_input, Align, Application, Button, Checkbox,
    Column, Command, Container, Element, Font, HorizontalAlignment, Length,
    Row, Scrollable, Settings, Text, TextInput, Sandbox
};
pub struct Conversus {
    agoras: Vec<Agora>,
    users: Vec<User>
}
#[derive(Debug, Clone)]
pub struct Comment(String);

#[derive(Debug, Clone)]
pub struct Conversation {
    pub assembly: Vec<User>,
    pub presenter: User,
    // reactions to the conversation?
    pub posts: Vec<Comment>
}

#[derive(Debug, Clone)]
struct Panelist(String);
impl Default for Panelist {
    fn default() -> Panelist {
        let panelist = Panelist(String::from("Panelist"));
        panelist
    }
}
#[derive(Debug, Clone)]
struct Wayfarer(String);
impl Default for Wayfarer {
    fn default() -> Wayfarer {
        let wayfarer = Wayfarer(String::from("Wayfarer"));
        wayfarer
    }
}


#[derive(Debug, Clone, Default)]
pub struct User {
    pub kind: String,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub conversations: Vec<Conversation>,
    pub comments: Vec<String>

}

pub struct Agora {
    pub name: String,
    pub topic: String,
    pub desc: String,
    pub conversations: Vec<Conversation>,
    pub founder: User,

}

// How will we keep comments in order?
    // incrementing each comment per conversation works, but only if
    // we don't allow threading (or we can treat nested threads like new conversations!)

#[derive(Debug, Clone)]
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
    KindChange(String),
    UserNameChange(String),
    EmailChange(String),
    PasswordChange(String),
    EnteredConversation(User),
    ExitedConversation(String),
    AddedComment(Comment),
    RemovedComment(String)
}
#[derive(Debug, Clone)]
pub enum Message {
    AgoraMessage,
    UserMessage,
    ConversationMessage
}
impl Agora {
    pub fn new (
        name: String, 
        topic: String, 
        desc: String, 
        conversations: Vec<Conversation>, 
        founder: User) -> Agora {
            Agora {
                name,
                topic,
                desc,
                conversations: conversations,
                founder: founder
            }
        }
    pub fn update(&mut self, message: AgoraMessage) {
        match message {
            AgoraMessage::ConversationAdded(conversation) => {

                // *self = Conversations::
                let old_conversations = self.conversations.clone();
                let old_conversations = old_conversations.push(conversation);
                let new_conversation = old_conversations;
                self.conversations = Vec{..new_conversation};
            }
            AgoraMessage::NameChanged(name) => {
                self.name = name
            }
            AgoraMessage::DescChanged(desc) => {
                self.desc = desc
            }
        }
    // }
}

impl Conversation {
    fn new(assembly: Vec<User>, presenter: User, posts: Vec<Comment>) -> Conversation {
        Conversation {
            assembly,
            presenter,
            posts,
        }
    }
}


impl User {
    fn new(kind: String, user_name: String, password: String, email: String, conversations: Vec<Conversation>, comments: Vec<String>) -> User
        {
            User {
                kind,
                user_name,
                password,
                email,
                conversations,
                comments
            }
        }

}


// impl Sandbox for Conversus {
//     type Message = Message;

//     fn new() -> {
//         Self::default()
//     }
//     fn title(&self) -> String {
//         String::from("Conversus")
//     }

//     fn update(&mut self, message: Message) {
//         match message {
//             Message:: AgoraMessage => match agora_message {
//                 if let Some(Agoram)
//             }
//         }
//     }
// }

pub fn main() {
    Conversus::run(Settings::default())
}
