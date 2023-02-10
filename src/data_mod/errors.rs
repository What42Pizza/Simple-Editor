use crate::prelude::*;
use sdl2::render::TextureValueError;



#[inline]
pub fn err<T> (input: RawProgramError) -> Result<T, ProgramError> {
    Err(input.into())
}





#[derive(Debug)]
pub struct ProgramError {
    pub raw: RawProgramError,
}





impl From<SerdeError> for ProgramError {
    fn from (source: SerdeError) -> ProgramError {
        Self {
            raw: RawProgramError::SerdeError(source),
        }
    }
}

impl From<TextureValueError> for ProgramError {
    fn from (source: TextureValueError) -> ProgramError {
        Self {
            raw: RawProgramError::TextureValueError(source),
        }
    }
}

impl From<String> for ProgramError {
    fn from (source: String) -> ProgramError {
        Self {
            raw: RawProgramError::String(source),
        }
    }
}





impl From<RawProgramError> for ProgramError {
    fn from (input: RawProgramError) -> ProgramError {
        Self {
            raw: input,
        }
    }
}





#[derive(Debug)]
pub enum RawProgramError {

    InvalidFileIndex {
        file_index: usize,
        num_of_files: usize,
    },

    SettingsAreNotAnObject,

    CouldNotLoadFile {
        file_path: String,
        source: IoError,
    },

    SerdeError (SerdeError),
    TextureValueError (TextureValueError),
    String (String),

    WIP {
        details: String,
    },

}
