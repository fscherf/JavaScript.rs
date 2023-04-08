use std::fs;

pub struct Input {
    pub file: String,
    pub content: String,
}

impl Input {
    pub fn _from_string(input_string: String) -> Input {
        Input {
            file: String::from("<input>"),
            content: input_string,
        }
    }

    pub fn from_file(path: String) -> Input {
        let content = fs::read_to_string(&path)
            .expect("Should have been able to read the file")
            .to_string();

        Input {
            file: path,
            content: content,
        }
    }
}
