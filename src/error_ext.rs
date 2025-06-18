use std::error::Error as ErrorTrait;
use std::process::exit;

pub trait ErrorExt {
    fn into_box(self) -> Box<dyn ErrorTrait + Send + Sync>;
    fn detonate(self) -> !;
    fn detonate_with(self, action: String) -> !;
}

pub trait ResultExt<T> {
    fn commonize(self) -> Result<T, Box<dyn ErrorTrait + Sync + Send>>;
    fn unwrap_or_explosion(self) -> T;
    fn unwrap_or_explosion_with<F: Fn() -> String>(self, action: F) -> T;
}

impl<T: ErrorTrait + Send + Sync + 'static> ErrorExt for T {
    fn into_box(self) -> ComError {
        Box::<dyn ErrorTrait + Send + Sync>::from(self)
    }

    fn detonate(self) -> ! {
        println!("{}", self);
        exit(1);
    }

    fn detonate_with(self, action: String) -> ! {
        println!("{}: {}", action, self);
        exit(1);
    }
}

impl<T, E: ErrorTrait + Send + Sync + 'static> ResultExt<T> for Result<T, E> {
    fn commonize(self) -> Result<T, ComError> {
        self.map_err(|e| e.into_box())
    }

    fn unwrap_or_explosion(self) -> T {
        self.unwrap_or_else(|err| err.detonate())
    }

    fn unwrap_or_explosion_with<F: Fn() -> String>(self, action: F) -> T {
        self.unwrap_or_else(|err| err.detonate_with(action()))
    }
}

pub type ComError = Box<dyn ErrorTrait + Sync + Send>;
pub type ComResult<O> = Result<O, ComError>;
