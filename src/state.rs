// use messages::{AgoraMessage, ConversationMessage, UserMessage};
// pub struct Agora {
//     pub name: String,
//     pub topic: String,
//     pub desc: String,
//     pub conversations: Vec<Conversation>,
//     pub founder: User,

// }

// impl Agora {
//     pub fn new (
//         name: String, 
//         topic: String, 
//         desc: String, 
//         conversations: Vec<Conversation>, 
//         founder: User) -> Agora {
//             Agora {
//                 name,
//                 topic,
//                 desc,
//                 conversations,
//                 founder
//             }
//         }
//     pub fn update(&mut self, message: AgoraMessage) {
//         match message {
//             AgoraMessage::ConversationAdded(conversation) => {
//                 self.conversations.push(conversation)
//             }
//             AgoraMessage::NameChanged(name) => {
//                 self.state = Agora::NameChanged(name)
//             }
//         }
//     }
// }

// struct Comment(String);
// // How will we keep comments in order?
//     // incrementing each comment per conversation works, but only if
//     // we don't allow threading (or we can treat nested threads like new conversations!)
// pub struct Conversation {
//     pub assembly: Vec<User>,
//     pub presenter: User,
//     // reactions to the conversation?
//     pub posts: Vec<Comment>

// }

// impl Conversation {
//     fn new(assembly: Vec<User>, presenter: User, posts: Vec<Comment>) -> Conversation {
//         Conversation {
//             assembly,
//             presenter,
//             posts,
//         }
//     }
// }

// pub enum UserType {
//     Panelist,
//     Wayfarer
// }

// struct User {
//     pub kind: UserType,
//     pub user_name: String,
//     pub email: String,
//     pub password: String,
//     pub conversations: Vec<Conversation>,
//     pub comments: Vec<String>

// }

// impl User {
//     fn new(kind: UserType, user_name: String, password: String, conversations: Vec<Conversation>, comments: Vec<String>) -> User
//         {
//             User {
//                 kind,
//                 user_name,
//                 password,
//                 conversations,
//                 comments
//             }
//         }

// }

