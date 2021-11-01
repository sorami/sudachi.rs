/*
 *  Copyright (c) 2021 Works Applications Co., Ltd.
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *   Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 */

use crate::error::SudachiError;
use crate::prelude::SudachiResult;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("{file}:{line}\t{cause}")]
pub struct DicWriteError {
    pub(super) file: String,
    pub(super) line: usize,
    pub(super) cause: DicWriteReason,
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum DicWriteReason {
    #[error("The actual size {actual} was larger than expected {expected}")]
    InvalidSize { actual: usize, expected: usize },

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Field {0} did not exist in CSV lexicon")]
    NoRawField(&'static str),

    #[error(transparent)]
    CsvError(csv::Error),

    #[error("Invalid character literal {0}")]
    InvalidCharLiteral(String),

    #[error("Invalid i16 literal {0}")]
    InvalidI16Literal(String),

    #[error("Invalid u32 literal {0}")]
    InvalidU32Literal(String),

    #[error("Invalid word id: {0}")]
    InvalidWordId(String),

    #[error("Invalid word split {0}")]
    InvalidSplit(String),

    #[error("Invalid word split format - field {field} did not exist in {original}")]
    SplitFormatError {
        field: &'static str,
        original: String,
    },

    #[error("Surface can't be empty")]
    EmptySurface,

    #[error("Maximum number of POS (2^15-1) exceeded with {0}")]
    PosLimitExceeded(String),

    #[error("Split reference {0} was incorrect")]
    InvalidSplitWordReference(String),

    #[error("Lexicon contains unresolved splits, call resolve() function")]
    UnresolvedSplits,
}

pub struct DicCompilationCtx {
    name: String,
    line: usize,
}

impl Default for DicCompilationCtx {
    fn default() -> Self {
        DicCompilationCtx {
            name: Default::default(),
            line: Default::default(),
        }
    }
}

impl DicCompilationCtx {
    pub fn memory() -> Self {
        DicCompilationCtx {
            name: "<memory>".to_owned(),
            line: 0,
        }
    }

    #[inline]
    pub fn err<T, E: Into<DicWriteReason>>(&self, reason: E) -> SudachiResult<T> {
        Err(self.to_sudachi_err(reason))
    }

    #[inline(always)]
    pub fn to_sudachi_err<E: Into<DicWriteReason>>(&self, reason: E) -> SudachiError {
        match reason.into() {
            DicWriteReason::Io(e) => e.into(),
            reason => {
                let err = DicWriteError {
                    file: self.name.clone(),
                    line: self.line,
                    cause: reason,
                };
                err.into()
            }
        }
    }

    #[inline(never)]
    #[cold]
    pub fn to_sudachi_err_cold<E: Into<DicWriteReason>>(&self, reason: E) -> SudachiError {
        self.to_sudachi_err(reason)
    }

    #[inline(always)]
    pub fn transform<T>(&self, result: DicWriteResult<T>) -> SudachiResult<T> {
        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(self.to_sudachi_err_cold(e)),
        }
    }

    #[inline(always)]
    pub fn apply<T, F: FnOnce() -> DicWriteResult<T>>(&self, f: F) -> SudachiResult<T> {
        match f() {
            Ok(v) => Ok(v),
            Err(e) => Err(self.to_sudachi_err_cold(e)),
        }
    }

    pub fn set_filename(&mut self, new_name: String) -> String {
        std::mem::replace(&mut self.name, new_name)
    }

    pub fn add_line(&mut self, offset: usize) {
        self.line += offset;
    }

    pub fn set_line(&mut self, line: usize) -> usize {
        std::mem::replace(&mut self.line, line)
    }
}

pub type DicWriteResult<T> = std::result::Result<T, DicWriteReason>;
