
use iced::{
    button, scrollable, text_input, Align, Button, Checkbox,
    Column, Command, Container, Element, Font, HorizontalAlignment, Length,
    Row, Scrollable, Settings, Text, TextInput, Sandbox, Application
};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
pub fn main() {
    Conversus::run(Settings::default())
}

#[derive(Debug)]
enum Conversus {
    Loading,
    Loaded(State)
}

#[derive(Debug)]
struct State {
    pub agoras: Vec<Agora>,
    scroll: scrollable::State,
    input: text_input::State,
    input_value: String,
    saving: bool,
    pub users: Vec<User>,
    pub conversations: Vec<Conversation>, // Stateful
    pub comments: Vec<String>, // Stateful
}
// What state from above is going to be saved?
    // Agoras,
    // users,
    // conversations
    // comments
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersistentState {
    agoras: Vec<Agora>,
    users: Vec<User>,
    conversations: Vec<Conversation>,
    comments: Vec<Comment>,
    input_value: String
}
#[derive(Debug, Clone, Serialize, Deserialize)]
enum LoadErr {
    FileErr,
    FormatErr,
}
#[derive(Debug)]
enum SaveErr {
    DirErr,
    FileErr,
    WriteErr,
    FormatErr,
}
#[cfg(not(target_arch = "wasm32"))]
impl PersistentState {
    fn path() -> std::path::PathBuf {
        let mut path = if let Some(dirs) = directories::ProjectDirs::from("rs", "Conversus", "Convos")
            {
                dirs.data_dir().into()
            } else {
                std::env::current_dir().unwrap_or(std::path::PathBuf::new())
            };

        path.push("convos.json");
        path
    }

    async fn load() -> Result<PersistentState, LoadErr> {
        use async_std::prelude::*;

        let mut contents = String::new();

        let mut file = async_std::fs::File::open(Self::path())
            .await
            .map_err(|_| LoadErr::FileErr)?;
        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadErr::FileErr)?;
        
            serde_json::from_str(&contents).map_err(|_|, LoadErr::FormatErr)
    }

    async fn save(self) -> Result<(), SaveErr> {
        use async_std::prelude::*;

        let json = serde_json::to_string_pretty((&self))
            .map_err(|_| SaveErr::FormatErr)?;
        let path = Self::path();

        if let Some(dir) = path.parent() {
            async_std::fs::create_dir_all(dir)
                .await
                .map_err(|_| SaveErr::DirErr)?;
        }

        {
            let mut file = async_std::fs::File::create(path)
                .await
                .map_err(|_| SaveErr::FileErr)?;

            file.write_all(json.as_bytes())
                .await
                .map_err(|_| SaveErr::WriteErr)?;
        }

        async_std::task::sleep(std::time::Duration::from_secs(3)).await;

        Ok(())
    }

}
#[derive(Debug, Clone, Serialize, Deserialize)]
enum AgoraMessage {
    Loaded(Result<PersistentState, LoadErr>),
    ConversationAdded(Conversation),
    NameChanged(String, Uuid),
    DescChanged(String, Uuid),
    UserMessage(UserMessage),
    InputChanged(String),
    ConversationMessage(ConversationMessage),
}

impl Application for Conversus {
    type Executor = iced::executor::Default;
    type Flags =();
    type Message = AgoraMessage;

    fn new(flags: ()) -> (Conversus, Command<AgoraMessage>) {
        (Conversus::Loading, Command::none())
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
            AgoraMessage::ConversationMessage(ConversationMessage::CommentAdded(comment)) => {
                Command::none()
            }
            AgoraMessage::ConversationMessage(ConversationMessage::CommentDeleted(comment)) => {
                Command::none()
            }
            AgoraMessage::ConversationMessage(ConversationMessage::UserAdded(user)) => {
                Command::none()
            }
            AgoraMessage::ConversationMessage(ConversationMessage::UserExited(uuid)) => {
                Command::none()
            }
            _ => {
                Command::none()
            }
        }
     }

     fn view(&mut self) -> Element<AgoraMessage> {
                Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Text::new(&self.title().to_string()).size(50)
            )
            // .push(TextInput::new())
            .into()
     }
}



// Tasks:
    // Write updater function for entire app - In Progress
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
    // We need to pull stateful fields out into a state struct - In Progress
    // so that we can update state centrally from the Agora update function
    // then we can remove the update functions for sub structures
    // and just let them read the state they represent
        // We'll leave the scalar state to the objects that need them,
        // an give the complex state to the agora perhaps.
        // however this, then requires the agora to distribute state as instantiations
        // of objects require it- how is a user object going to keep track of conversations it's
        // connected to, without any information about conversation state?
        // We could use a vector of conversation id's (Uuids) - impl'd
            // Alright so a User object wants to add a comment to some conversation, we'll say
            // identifiable by string u_c_convo_title_hash, so the user says to Agora,
            // hey add this comment to convo u_c_convo_title_hash, and the Agora says, where?
            // and we say to the end of the list of comments. boom.
    // We need to do data normalization and define the arity(-ies?) of these relationships


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    content: String,
    id: Uuid,
    convo_id: Uuid
}

impl Comment {
    fn new(content: String, id: Uuid, convo_id: Uuid) -> Comment {
        Comment {
            content: content,
            id: id,
            convo_id
        }
    }
    fn make_comment() -> Comment {
        Comment {
        content: "Awesome".to_string(), 
        id: Uuid::new_v4(),
        convo_id: Uuid::new_v4()
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



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    // pub fn update(&mut self, message: AgoraMessage) {
    //     match message {
    //         AgoraMessage::ConversationAdded(conversation) => {

    //             let mut new_conversations = self.conversations.clone();
    //             new_conversations.push(conversation);
    //             self.conversations = new_conversations;
    //         }
    //         AgoraMessage::NameChanged(name, uuid) => {
    //                 // use uuid to target specific agoras
    //             self.name = name
    //         }
    //         AgoraMessage::DescChanged(desc, uuid) => {
    //             self.desc = desc
    //         }
    //     }
    // }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConversationMessage {
    CommentAdded(String),
    CommentDeleted(Uuid),
    UserAdded(User),
    UserExited(Uuid)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Conversation {
    pub assembly: Vec<User>, // Stateful
    pub presenter: User,
    pub comments: Vec<Comment>, // Stateful
    pub agora_id: Uuid,
}

impl Conversation {
    fn new(assembly: Vec<User>, presenter: User, comments: Vec<Comment>, agora_id: Uuid) -> Conversation {
        Conversation {
            assembly,
            presenter,
            comments,
            agora_id,
        }
    }

    fn update(&mut self, message: ConversationMessage) {
        match message {
            ConversationMessage::CommentAdded(comment) => {
                let mut new_comments = self.comments.clone();
                let new_comment = Comment {content: comment, id: Uuid::new_v4(), convo_id: Uuid::new_v4()};
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
                new_assembly.push(user);
                self.assembly = new_assembly;
                
            }
            ConversationMessage::UserExited(u_uuid) => {

                let mut new_assembly = self.assembly.clone();
                new_assembly.retain(|user| user.user_id != u_uuid);

            }
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub kind: String,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub conversations: Vec<Uuid>, // Stateful
    pub user_id: Uuid,
    pub comments: Vec<String>

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
        conversations: Vec<Uuid>, // Stateful
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



