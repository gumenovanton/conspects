THISERROR
// for error handling
// used for libs or where i need error type checking
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("Ошибка ввода-вывода: {0}")] // impl Display and places error to 0
    Io(#[from] std::io::Error), // impl From trait to convert error from std io error

    #[error("Ошибка парсинга: {0}")]
    Parse(String),

    #[error("Не найдено")]
    NotFound,

    #[error("Неверный ввод для поля '{field}': {reason}")] // replaces aliases with fields value
    InvalidInput {
        field: String,
        reason: String,
    },

    #[error("HTTP ошибка {status}: {message}")]
    HttpError {
        status: u16,
        message: String,
    },
}
