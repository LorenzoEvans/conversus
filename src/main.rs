
use iced::{
    button, scrollable, text_input, Align, Application, Button, Checkbox,
    Column, Command, Container, Element, Font, HorizontalAlignment, Length,
    Row, Scrollable, Settings, Text, TextInput, Sandbox
};
use uuid::Uuid;

// Tasks:
    // Add new fn's to each structure implementation
        // Comment - done
        // Panelist/Wayfarer - don't need
        // User - done
        // Conversation - done
        // Agora - done
    // Add updater functions to each structure implementation
        // Comment - done
        // User - done
        // Conversation - done
        // Agora - done
    // Develop basic view for Conversus


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
    UserAdded(User),
    UserExited(Uuid)
}

#[derive(Debug, Clone)]
pub struct Conversation {
    pub assembly: Vec<User>,
    pub presenter: User,
    pub comments: Vec<Comment>,
    pub convo_id: Uuid
}

impl Conversation {
    fn new(assembly: Vec<User>, presenter: User, comments: Vec<Comment>, convo_id: Uuid) -> Conversation {
        Conversation {
            assembly,
            presenter,
            comments,
            convo_id
        }
    }

    fn update(&mut self, message: ConversationMessage) {
        match message {
            ConversationMessage::CommentAdded(comment) => {
                let mut new_comments = self.comments.clone();
                let new_comment = Comment {content: comment, id: Uuid::new_v4()};
                new_comments.push(new_comment);
                self.comments = new_comments;
            }
            ConversationMessage::CommentDeleted(uuid) => {

                let length = self.comments.len();
                let comments = self.comments.clone();

                for mut i in comments {

                    if  i.id == uuid {

                        i.content = String::from("This comment was deleted.");
                    }
                }
            }
            ConversationMessage::UserAdded(user) => {

                let mut new_assembly = self.assembly.clone();
                new_assembly.push(user)
                
            }
            ConversationMessage::UserExited(u_uuid) => {

                let mut new_assembly = self.assembly.clone();
                new_assembly.retain(|user| user.user_id != u_uuid);

            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct User {
    pub kind: String,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub conversations: Vec<Conversation>,
    pub comments: Vec<String>,
    pub user_id: Uuid

}
#[derive(Debug, Clone)]
pub enum UserMessage {
    KindChange(String),
    UserNameChange(String),
    EmailChange(String),
    PasswordChange(String),
}

impl User {
    fn new(
        kind: String, 
        user_name: String,
        password: String, 
        email: String, 
        conversations: Vec<Conversation>, 
        comments: Vec<String>, 
        user_id: Uuid) -> User
        {
            User {
                kind,
                user_name,
                password,
                email,
                conversations,
                comments,
                user_id
            }
        }
    fn update(&mut self, message: UserMessage) {
        match message {
            UserMessage::KindChange(new_kind) => {
                self.kind = new_kind
            }
            UserMessage::UserNameChange(username) => {
                self.user_name = username
            }
            UserMessage::EmailChange(e_mail) => {
                self.email = e_mail
            }
            UserMessage::PasswordChange(new_pw) => {
                // we should use the uuid hashes here, no plaintext!s
                self.password = new_pw
            }
        }
    }
}


#[derive(Debug, Clone)]
pub enum Message {
    AgoraMessage,
    UserMessage,
    ConversationMessage
}

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
