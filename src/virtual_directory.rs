pub struct DirectoryItemDescription<'a> {
    pub pid: u64,
    pub user: &'a String,
    pub size: u64,
    pub time: String,
    pub name: String,
}

impl std::string::ToString for DirectoryItemDescription<'_> {
    fn to_string(&self) -> String {
        format!("{} {} {} {} {} {}", &self.pid, &self.user, &self.user, &self.size, &self.time, &self.name)
    }
}

pub enum DirectoryItem<'a> {
    File {
        description: DirectoryItemDescription<'a>,
        content: String,
    },
    Directory {
        description: DirectoryItemDescription<'a>,
        content: Vec<DirectoryItem<'a>>
    }
}

impl std::fmt::Display for DirectoryItem<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DirectoryItem::File { description, content } => {
                write!(formatter, "-rw-rw-r-- {}", description.to_string())
            }
            DirectoryItem::Directory { description, content } => {
                write!(formatter, "drwxrwxr-x {}", description.to_string())
            }
        }
    }
}

pub struct VirtualDirectory<'a> {
    root: DirectoryItem<'a>
}

pub fn generate_virtual_directory<'a>(user: &'a String) -> DirectoryItem {
    let files = vec![];
    let description = DirectoryItemDescription {
        pid: 0,
        user: user,
        size: 4096,
        time: String::from("Monday"),
        name: String::from("mikle"),
    };
    let file = DirectoryItem::File {
        description,
        content: String::from("some content"),
    };
    return file;
}
