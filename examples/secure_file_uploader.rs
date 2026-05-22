/// This example demonstrate how to use struct with
/// generic type parameters to enforce
/// a state machine pattern for a file uploader.

pub struct Pending;
pub struct Validated;

pub struct FileUploader<State> {
    pub filename: String,
    state: std::marker::PhantomData<State>,
}

impl FileUploader<Pending> {
    pub fn new(name: &str) -> Self {
        println!("Created FileUploader for file: {} with status: Pending", name);

        Self {
            filename: name.to_string(),
            state: std::marker::PhantomData,
        }
    }

    pub fn validate(self) -> FileUploader<Validated> {
        println!("Validating file: {}", self.filename);

        FileUploader {
            filename: self.filename,
            state: std::marker::PhantomData,
        }
    }
}

impl FileUploader<Validated> {
    pub fn upload(&self) {
        println!("Successfully uploaded file: {}", self.filename);
    }
}

fn main() {
    let uploader = FileUploader::<Pending>::new("secret.txt");
    let validated_uploader = uploader.validate();
    validated_uploader.upload();
}
