pub struct DeleteRestoreOptions {
    pub owner_id: Option<usize>,
}

impl Default for DeleteRestoreOptions {
    fn default() -> Self {
        Self { owner_id: None }
    }
}

pub struct EditOptions {
    pub options: Options,
}

pub enum Options {
    NewOptions(Privacy, Vec<NewOptionsList>, Privacy, Vec<NewOptionsList>),
    OldOptions(Privacy, Privacy),
}

pub enum NewOptionsList {
    List(i64),
    User(i64),
    ListRestricted(i64),
    UserRestricted(i64),
}

impl Default for EditOptions {
    fn default() -> Self {
        Self {
            options: Options::NewOptions(Privacy::All, vec![], Privacy::All, vec![]),
        }
    }
}

pub enum Privacy {
    All,
    Friends,
    FriendsOfFriends,
    OnlyMe,
}
