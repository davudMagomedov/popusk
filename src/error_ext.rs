use std::error::Error as ErrorTrait;

pub trait IntoBoxExt {
    fn into_box(self) -> Box<dyn ErrorTrait + Send + Sync>;
}

pub trait CommonizeResultExt<O> {
    fn commonize(self) -> Result<O, Box<dyn ErrorTrait + Sync + Send>>;
}

impl<T: ErrorTrait + Send + Sync + 'static> IntoBoxExt for T {
    fn into_box(self) -> ComError {
        Box::<dyn ErrorTrait + Send + Sync>::from(self)
    }
}

impl<O, E: ErrorTrait + Send + Sync + 'static> CommonizeResultExt<O> for Result<O, E> {
    fn commonize(self) -> Result<O, ComError> {
        self.map_err(|e| e.into_box())
    }
}

pub type ComError = Box<dyn ErrorTrait + Sync + Send>;
pub type ComResult<O> = Result<O, ComError>;
