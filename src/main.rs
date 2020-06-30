
use iced::{
    button, scrollable, text_input, Align, Application, Button, Checkbox,
    Column, Command, Container, Element, Font, HorizontalAlignment, Length,
    Row, Scrollable, Settings, Text, TextInput,
};
#[derive(Default)]
pub struct Agora {
    pub name: String,
    pub topic: String,
    pub desc: String,
    pub conversations: Option<Vec<Conversation>>,
    pub founder: User,

}
#[derive(Debug, Clone, Default)]
pub struct User {
    pub kind: UserType,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub conversations: Vec<Conversation>,
    pub comments: Vec<String>

}
#[derive(Debug, Clone)]
pub enum UserType {
    Panelist,
    Wayfarer
}
#[derive(Debug, Clone)]
struct Comment(String);
// How will we keep comments in order?
    // incrementing each comment per conversation works, but only if
    // we don't allow threading (or we can treat nested threads like new conversations!)
#[derive(Debug, Clone)]
pub struct Conversation {
    pub assembly: Vec<User>,
    pub presenter: User,
    // reactions to the conversation?
    pub posts: Vec<Comment>

}

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
    KindChange(UserType),
    UserNameChange(String),
    EmailChange(String),
    PasswordChange(String),
    EnteredConversation(User),
    ExitedConversation(String),
    AddedComment(Comment),
    RemovedComment(String)
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
                conversations: Some(conversations),
                founder: founder
            }
        }
    pub fn update(&mut self, message: AgoraMessage) {
        match message {
            AgoraMessage::ConversationAdded(conversation) => {
                self.conversations.unwrap().push(conversation)
            }
            AgoraMessage::NameChanged(name) => {
                self.name = name
            }
            AgoraMessage::DescChanged(desc) => {
                self.desc = desc
            }
        }
    }
}

// impl Default for Agora {
//     fn default(
//         name: String, 
//         topic: String, 
//         desc: String, 
//         conversations: Vec<Conversation>, 
//         founder: User) -> Agora {
//             let name = String::from("Convo Royale!");
//             let topic = String::from("Whatever dominates the conversation");
//             let desc = String::from("May the best topic win.");
//             let conversation = 
//             let conversations = Vec::new();
//             // conversations.push(conversation);

//             Agora {
//                 name: name, 
//                 topic: topic,
//                 desc: desc,
//             }
//         }
// }


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
    fn new(kind: UserType, user_name: String, password: String, email: String, conversations: Vec<Conversation>, comments: Vec<String>) -> User
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

fn main() {
    println!("Hello, world!");
}
