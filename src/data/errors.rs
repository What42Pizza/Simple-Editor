use crate::{data::{program_data::*, settings::*, errors::Result::*}, fns};

use std::{fmt, result::Result as stdResult,
    ops::{Try, ControlFlow, FromResidual}
};
use sdl2::{video::WindowBuildError, IntegerOrSdlError, ttf::FontError, render::TextureValueError};





// ---------------- Additng to std::option::Option ----------------

pub trait CustomOptionFns<T> {
    fn none_err (&self, error_name: &str, error_details: &str) -> Result<&T>;
    fn none_err_lazy (&self, error_name: &str, error_details_fn: impl FnOnce() -> String) -> Result<&T>;
}

impl<T> CustomOptionFns<T> for Option<T> {

    fn none_err (&self, error_name: &str, error_details: &str) -> Result<&T> {
        match self {
            Some(v) => Ok(v),
            None => err(error_name, error_details),
        }
    }

    fn none_err_lazy (&self, error_name: &str, error_details_fn: impl FnOnce() -> String) -> Result<&T> {
        match self {
            Some(v) => Ok(v),
            None => err(error_name, &error_details_fn()),
        }
    }

}





// ---------------- Adding to std::result::Result ----------------

pub trait CustomResultFns<T, E> where Error: From<E> {
    fn to_custom_err (self) -> Result<T>;
    fn chain_err (self, name: &str, details: &str) -> Result<T>;
    fn chain_err_lazy (self, name: &str, details: impl FnOnce() -> String) -> Result<T>;
    fn err_details (self, details: &str) -> Result<T>;
    fn err_pre_details (self, details: &str, offset_amount: usize) -> Result<T>;
    fn err_details_lazy (self, details_fn: impl FnOnce() -> String) -> Result<T>;
    fn err_pre_details_lazy (self, details_fn: impl FnOnce() -> String, offset: usize) -> Result<T>;
}

impl<T, E> CustomResultFns<T, E> for stdResult<T, E> where Error: From<E> {

    fn to_custom_err (self) -> Result<T> {
        match self {
            Self::Ok(v) => Result::Ok(v),
            Self::Err(e) => Result::Err(e.into()),
        }
    }

    fn chain_err (self, name: &str, details: &str) -> Result<T> {
        match self {
            Self::Ok(v) => Result::Ok(v),
            Self::Err(e) => Result::Err(Error {
                name: name.to_string(),
                details: vec!(details.to_string()),
                cause: Some(Box::new(e.into())),
            }),
        }
    }

    fn chain_err_lazy (self, name: &str, details_fn: impl FnOnce() -> String) -> Result<T> {
        match self {
            Self::Ok(v) => Result::Ok(v),
            Self::Err(e) => Result::Err(Error {
                name: name.to_string(),
                details: vec!(details_fn()),
                cause: Some(Box::new(e.into())),
            }),
        }
    }

    fn err_details (self, details: &str) -> Result<T> {
        match self {
            Self::Ok(v) => Result::Ok(v),
            Self::Err(e) => {
                let mut new_error: Error = e.into();
                new_error.details.push(details.to_string());
                Result::Err(new_error)
            }
        }
    }

    fn err_details_lazy (self, details_fn: impl FnOnce() -> String) -> Result<T> {
        match self {
            Self::Ok(v) => Result::Ok(v),
            Self::Err(e) => {
                let mut new_error: Error = e.into();
                new_error.details.push(details_fn());
                Result::Err(new_error)
            }
        }
    }

    fn err_pre_details (self, details: &str, offset: usize) -> Result<T> {
        match self {
            Self::Ok(v) => Result::Ok(v),
            Self::Err(e) => {
                let mut new_error: Error = e.into();
                new_error.details.insert(new_error.details.len() - offset, details.to_string());
                Result::Err(new_error)
            }
        }
    }

    fn err_pre_details_lazy (self, details_fn: impl FnOnce() -> String, offset: usize) -> Result<T> {
        match self {
            Self::Ok(v) => Result::Ok(v),
            Self::Err(e) => {
                let mut new_error: Error = e.into();
                new_error.details.insert(new_error.details.len() - offset, details_fn());
                Result::Err(new_error)
            }
        }
    }

}





// ---------------- Custom Result ----------------

#[derive(is_enum_variant)]
pub enum Result<T> {
    Ok (T),
    Err (Error),
}



impl<T> Result<T> {

    pub fn chain_err (self, name: &str, details: &str) -> Self {
        if let Self::Err(error) = self {
            return Self::Err(Error {
                name: name.to_string(),
                details: vec!(details.to_string()),
                cause: Some(Box::new(error)),
            });
        }
        self
    }

    pub fn chain_err_lazy (self, name: &str, details_fn: impl FnOnce() -> String) -> Self {
        if let Self::Err(error) = self {
            return Self::Err(Error {
                name: name.to_string(),
                details: vec!(details_fn()),
                cause: Some(Box::new(error)),
            });
        }
        self
    }

    pub fn err_details (mut self, details: &str) -> Self {
        if let Self::Err(error) = &mut self {
            error.details.push(details.to_string());
        }
        self
    }

    pub fn err_details_lazy (mut self, details_fn: impl FnOnce() -> String) -> Self {
        if let Self::Err(error) = &mut self {
            error.details.push(details_fn());
        }
        self
    }

    pub fn err_pre_details (mut self, details: &str, offset: usize) -> Self {
        if let Self::Err(error) = &mut self {
            error.details.insert(error.details.len() - offset, details.to_string());
        }
        self
    }

    pub fn err_pre_details_lazy (mut self, details_fn: impl FnOnce() -> String, offset: usize) -> Self {
        if let Self::Err(error) = &mut self {
            error.details.insert(error.details.len() - offset, details_fn());
        }
        self
    }

    pub fn or_else (self, other: impl FnOnce(Error) -> Self) -> Self {
        if let Self::Err(error) = self {
            other(error)
        } else {
            self
        }
    }

    pub fn unwrap (self) -> T {
        match self {
            Ok(v) => v,
            Err(error) => panic!("tried to unwrap err variant (error: {error})"),
        }
    }

    pub fn unwrap_or (self, other: impl FnOnce(Error) -> T) -> T {
        match self {
            Ok(v) => v,
            Err(error) => other(error),
        }
    }

}



impl<T> Try for Result<T> {
    type Output = T;
    type Residual = Error;
    fn from_output(value: <Self as Try>::Output) -> Self {
        Self::Ok(value)
    }
    fn branch(self) -> ControlFlow<<Self as Try>::Residual, <Self as Try>::Output> {
        match self {
            Self::Ok (v) => ControlFlow::Continue(v),
            Self::Err (e) => ControlFlow::Break(e),
        }
    }
}

impl<T> FromResidual<Error> for Result<T> {
    fn from_residual(error: Error) -> Self {
        Self::Err (error)
    }
}

impl<T> FromResidual<Error> for stdResult<T, Error> {
    fn from_residual(error: Error) -> Self {
        Self::Err (error)
    }
}



/*
impl<T> Try for &Result<T> {
    type Output = T;
    type Residual = Error;
    fn from_output(value: <Self as Try>::Output) -> Self {
        &Result::Ok(value)
    }
    fn branch(self) -> ControlFlow<<Self as Try>::Residual, <Self as Try>::Output> {
        match self {
            &Result::Ok (v) => ControlFlow::Continue(v),
            &Result::Err (e) => ControlFlow::Break(e),
        }
    }
}

impl<T> FromResidual<Error> for &Result<T> {
    fn from_residual(error: Error) -> Self {
        &Result::Err (error)
    }
}
*/





// ---------------- Custom Error ----------------

#[derive(Debug)]
pub struct Error {
    name: String,
    details: Vec<String>,
    cause: Option<Box<Error>>,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let details_string = self.get_details();
        if let Some(cause) = &self.cause {
            write!(formatter, "{}{}\n\nCaused by: {}", self.name, details_string, *cause)
        } else {
            write!(formatter, "{}{}", self.name, details_string)
        }
    }
}

impl Error {
    pub fn new (name: &str, details: &str) -> Self {
        Self {
            name: name.to_string(),
            details: vec![details.to_string()],
            cause: None,
        }
    }
    fn get_details (&self) -> String {
        let mut details_string = String::from("");
        for current_detail in &self.details {
            if current_detail.is_empty() {continue;}
            details_string += "\n| ";
            details_string += current_detail;
        }
        details_string
    }
}



impl From<std::io::Error> for Error {
    fn from (error: std::io::Error) -> Self {
        Self {
            name: String::from("io::Error"),
            details: vec!(error.to_string()),
            cause: None,
        }
    }
}

impl From<String> for Error {
    fn from (error: String) -> Self {
        Self {
            name: String::from("string::String"),
            details: vec!(error),
            cause: None,
        }
    }
}

impl From<serde_hjson::Error> for Error {
    fn from (error: serde_hjson::Error) -> Self {
        Self {
            name: String::from("serde_hjson::Error"),
            details: vec!(error.to_string()),
            cause: None,
        }
    }
}

impl From<WindowBuildError> for Error {
    fn from (error: WindowBuildError) -> Self {
        Self {
            name: String::from("sdl2::WindowBuildError"),
            details: vec!(error.to_string()),
            cause: None,
        }
    }
}

impl From<IntegerOrSdlError> for Error {
    fn from (error: IntegerOrSdlError) -> Self {
        Self {
            name: String::from("sdl2::IntegerOrSdlError"),
            details: vec!(error.to_string()),
            cause: None,
        }
    }
}

impl From<FontError> for Error {
    fn from (error: FontError) -> Self {
        Self {
            name: String::from("sdl2::FontError"),
            details: vec!(error.to_string()),
            cause: None,
        }
    }
}

impl From<TextureValueError> for Error {
    fn from (error: TextureValueError) -> Self {
        Self {
            name: String::from("sdl2::TextureValueError"),
            details: vec!(error.to_string()),
            cause: None,
        }
    }
}





// ---------------- fns ----------------

pub fn err<T> (error_name: &str, error_details: &str) -> Result<T> {
    Result::Err(Error{
        name: error_name.to_string(),
        details: vec!(error_details.to_string()),
        cause: None,
    })
}

pub fn err_lazy<T> (error_name: &str, error_details_fn: impl FnOnce() -> String) -> Result<T> {
    Result::Err(Error{
        name: error_name.to_string(),
        details: vec!(error_details_fn()),
        cause: None,
    })
}
