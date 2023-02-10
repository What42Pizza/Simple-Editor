use crate::prelude::*;
use sdl2::render::TextureValueError;



/*
pub trait ToProgramErrorResult<T> {
    type ErrorType;
    fn to_program_error (self) -> Result<T, ProgramError>;
}

impl<T> ToProgramErrorResult<T> for Result<T, String> {
    type ErrorType = String;
    fn to_program_error (self) -> Result<T, ProgramError> {
        self.map_err(|e: String| -> ProgramError {e.into()})
    }
}

impl<T> ToProgramErrorResult<T> for Result<T, SerdeError> {
    type ErrorType = SerdeError;
    fn to_program_error (self) -> Result<T, ProgramError> {
        self.map_err(|e: SerdeError| -> ProgramError {e.into()})
    }
}

impl<T> ToProgramErrorResult<T> for Result<T, TextureValueError> {
    type ErrorType = TextureValueError;
    fn to_program_error (self) -> Result<T, ProgramError> {
        self.map_err(|e: TextureValueError| -> ProgramError {e.into()})
    }
}
*/
