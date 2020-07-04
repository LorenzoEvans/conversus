
use iced::{
    button, scrollable, text_input, Align, Application, Button, Checkbox,
    Column, Command, Container, Element, Font, HorizontalAlignment, Length,
    Row, Scrollable, Settings, Text, TextInput, Sandbox
};
use uuid::Uuid;


// impl Default for Conversus {}
#[derive(Debug, Clone, PartialEq)]
pub struct Comment {
    content: String,
    id: Uuid
}

impl Comment {
    fn new(content: String, id: Uuid) -> Comment {
        Comment {
            content: content,
            id: id
        }
    }
    fn make_comment() -> Comment {
        Comment {
        content: "Awesome".to_string(), 
        id: Uuid::new_v4()
        }
    }
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
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

                let mut new_conversations = self.conversations.clone();
                new_conversations.push(conversation);
                self.conversations = new_conversations;
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

#[derive(Debug, Clone)]
pub enum ConversationMessage {
    CommentAdded(String),
    CommentDeleted(Uuid),
}

#[derive(Debug, Clone)]
pub struct Conversation {
    pub assembly: Vec<User>,
    pub presenter: User,
    pub comments: Vec<Comment>
}

impl Conversation {
    fn new(assembly: Vec<User>, presenter: User, comments: Vec<Comment>) -> Conversation {
        Conversation {
            assembly,
            presenter,
            comments,
        }
    }

    fn update(&mut self, message: ConversationMessage) {
        match message {
            ConversationMessage::CommentAdded(comment) => {
                let mut new_comments = self.comments.clone();
                let new_comment = Comment {content: comment, id: Uuid::new_v4()};
                new_comments.push(new_comment);
                self.comments = new_comments
            }
            ConversationMessage::CommentDeleted(uuid) => {
                let length = self.comments.len();
                let comments = self.comments.clone();
                for mut i in comments {
                    if  i.id == uuid {
                        i.content = String::from("This comment was deleted.")
                    }
                }
            }
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

#[derive(Debug, Clone)]
pub struct Conversus {
    agoras: Vec<Agora>,
    users: Vec<User>
}
// impl Sandbox for Conversus {
//     type Message = Message;

//     fn new() -> Self {
//         Self::default()
//     }
//     fn title(&self) -> String {
//         String::from("Conversus")
//     }

//      fn update(&mut self, message: Message) {
//          match message {
//              Message:: AgoraMessage => match agora_message {
//                  if let Some(Agoram)
//              }
//          }
//      }
// }

pub fn main() {
    print!("Hey!");
}
