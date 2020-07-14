
use iced::{
    button, scrollable, text_input, Align, Button, Checkbox,
    Column, Command, Container, Element, Font, HorizontalAlignment, Length,
    Row, Scrollable, Settings, Text, TextInput, Sandbox, Application
};
use uuid::Uuid;



// Tasks:
    // Write updater function for entire app-
        // Probably going to be some nested match arms, since we nested message types.
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
    // Develop basic view for Conversus - done (we've got a Gui?)
    // We need to pull stateful fields out into a state struct
    // so that we can update state centrally from the Agora update function
    // then we can remove the update functions for sub structures
    // and just let them read the state they represent
        // We'll leave the scalar state to the objects that need them,
        // an give the complex state to the agora perhaps.
        // however this, then requires the agora to distribute state as instantiations
        // of objects require it- how is a user object going to keep track of conversations it's
        // connected to, without any information about conversation state?
        // We could use a vector of conversation id's (Uuids)
            // Alright so a User object wants to add a comment to some conversation, we'll say
            // identifiable by string u_c_convo_title_hash, so the user says to Agora,
            // hey add this comment to convo u_c_convo_title_hash, and the Agora says, where?
            // and we say to the end of the list of comments. boom.
    // We need to do data normalization and define the arity(-ies?) of these relationships


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



#[derive(Debug, Clone, PartialEq)]
pub struct Agora {
    pub name: String,
    pub topic: String,
    pub desc: String,
    pub conversations: Vec<Conversation>, // Stateful
    pub founder: User,

}

// How will we keep comments in order?
    // incrementing each comment per conversation works, but only if
    // we don't allow threading (or we can treat nested threads like new conversations!)

#[derive(Debug, Clone, PartialEq)]
pub enum AgoraMessage {
    ConversationAdded(Conversation),
    NameChanged(String, Uuid),
    DescChanged(String, Uuid),
    UserMessage(UserMessage),
    ConversationMessage(ConversationMessage),
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
            AgoraMessage::NameChanged(name, uuid) => {
                    // use uuid to target specific agoras
                self.name = name
            }
            AgoraMessage::DescChanged(desc, uuid) => {
                self.desc = desc
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConversationMessage {
    CommentAdded(String),
    CommentDeleted(Uuid),
    UserAdded(User),
    UserExited(Uuid)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Conversation {
    pub assembly: Vec<User>, // Stateful
    pub presenter: User,
    pub comments: Vec<Comment>, // Stateful
    pub agora_id: Uuid
}

impl Conversation {
    fn new(assembly: Vec<User>, presenter: User, comments: Vec<Comment>, agora_id: Uuid) -> Conversation {
        Conversation {
            assembly,
            presenter,
            comments,
            agora_id
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

#[derive(Debug, Clone, Default, PartialEq)]
pub struct User {
    pub kind: String,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub conversations: Vec<Conversation>, // Stateful
    pub user_id: Uuid

}
#[derive(Debug, Clone, PartialEq)]
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
        conversations: Vec<Conversation>, // Stateful
        comments: Vec<String>, // Stateful
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
                // we should use the uuid hashes or hash fn here, no plaintext!
                self.password = new_pw
            }
        }
    }
}


// #[derive(Debug, Clone, PartialEq)]
// pub enum Message {
//     AgoraMessage{ kind: AgoraMessage},
//     UserMessage(UserMessage),
//     ConversationMessage(ConversationMessage)
// }

struct State {
    pub agoras: Vec<Agora>,
    pub users: Vec<User>
    pub conversations: Vec<Conversation>, // Stateful
    pub comments: Vec<String>, // Stateful
}
#[derive(Debug, Default)]
pub struct Conversus {
    agoras: Vec<Agora>, // Stateful
    users: Vec<User>, // Stateful
    title: String,
}
impl Application for Conversus {
    type Executor = iced::executor::Default;
    type Flags =();
    type Message = AgoraMessage;

    fn new(flags: ()) -> (Self, Command<AgoraMessage>) {
        (Self::default(), Command::none())
    }
    fn title(&self) -> String {
        String::from("Conversus")
    }

     fn update(&mut self, message: AgoraMessage) -> Command<AgoraMessage> {
        match message {
            AgoraMessage::ConversationAdded(Conversation { assembly, presenter, comments, agora_id}) => {
            Command::none()
            }
            AgoraMessage::DescChanged(desc, uuid) => {
                Command::none()
            }
            AgoraMessage::NameChanged(name, uuid) => {
                Command::none()
            }
            AgoraMessage::UserMessage(UserMessage::EmailChange(email)) => {
                Command::none()
            }
            AgoraMessage::UserMessage(UserMessage::KindChange(kind)) => {
                Command::none()
            }
            AgoraMessage::UserMessage(UserMessage::PasswordChange(pw)) => {
                Command::none()
            }
            AgoraMessage::UserMessage(UserMessage::UserNameChange(username)) => {
                Command::none()
            }
            // Message::UserMessage => {
            //     // do user things here
            //     Command::none()
            // }
            // Message::ConversationMessage => {
            //     // do convo things here
            //     Command::none()
            // }
            // Message::AgoraMessage::NameChanged(name) => {
            //     // do agora things here
            //     Command::none()
            // }
            // Message::AgoraMessage::DescChanged(desc) => {
            //     // do agora things here
            //     Command::none()
            // }
        }
     }

     fn view(&mut self) -> Element<AgoraMessage> {
                Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Text::new(&self.title.to_string()).size(50)
            )
            // .push(TextInput::new())
            .into()
     }
}

pub fn main() {
    Conversus::run(Settings::default())
}
