pub struct Pending;
pub struct Validated;
pub struct Failed;

pub struct FileUploader<State> {
    pub filename: String,
    state: std::marker::PhantomData<State>,
}

pub enum ValidationResult {
    Success(FileUploader<Validated>),
    Blocked(FileUploader<Failed>),
}

impl FileUploader<Pending> {
    pub fn new(name: &str) -> Self {
        println!(
            "Created FileUploader for file: {} with status: Pending",
            name
        );
        Self {
            filename: name.to_string(),
            state: std::marker::PhantomData,
        }
    }

    pub fn validate(self) -> Result<FileUploader<Validated>, FileUploader<Failed>> {
        if self.filename.contains("virus") {
            println!("Validation failed for file: {}", self.filename);
            Err(FileUploader {
                filename: self.filename,
                state: std::marker::PhantomData,
            })
        } else {
            println!("Validation succeeded for file: {}", self.filename);
            Ok(FileUploader {
                filename: self.filename,
                state: std::marker::PhantomData,
            })
        }
    }
}

impl FileUploader<Validated> {
    pub fn execute_upload(&self) {
        println!("Successfully uploaded file: {}", self.filename);
    }
}

impl FileUploader<Failed> {
    pub fn handle_rejection(&self) {
        println!("Upload not allowed: {}", self.filename);
    }
}

impl ValidationResult {
    pub fn upload(&self) {
        match self {
            ValidationResult::Success(uploader) => uploader.execute_upload(),
            ValidationResult::Blocked(uploader) => uploader.handle_rejection(),
        }
    }
}

impl FileUploader<Pending> {
    pub fn validate_to_custom(self) -> ValidationResult {
        if self.filename.contains("virus") {
            println!("Validation failed for file: {}", self.filename);
            ValidationResult::Blocked(FileUploader {
                filename: self.filename,
                state: std::marker::PhantomData,
            })
        } else {
            println!("Validation succeeded for file: {}", self.filename);
            ValidationResult::Success(FileUploader {
                filename: self.filename,
                state: std::marker::PhantomData,
            })
        }
    }
}

fn main() {
    let uploader = FileUploader::new("secret.txt");
    let validated_uploader = uploader.validate_to_custom();

    let virus_uploader = FileUploader::new("secret-virus1.exe");
    let virus_validated_uploader = virus_uploader.validate_to_custom();

    validated_uploader.upload();
    virus_validated_uploader.upload();
}
