use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum CompileError {
    #[error("Parse error")]
    #[diagnostic(code = "E0001")]
    ParseError {
        #[source_code]
        src: NamedSource<String>,
        #[label = "here"]
        span: SourceSpan,
        message: String,
    },

    #[error("Type error: {message}")]
    #[diagnostic(code = "E0002")]
    TypeError {
        #[source_code]
        src: NamedSource<String>,
        #[label = "problematic expression"]
        span: SourceSpan,
        message: String,
    },

    #[error("Unsupported feature: {message}")]
    #[diagnostic(code = "E0003")]
    UnsupportedFeature {
        #[source_code]
        src: NamedSource<String>,
        #[label = "here"]
        span: SourceSpan,
        message: String,
        #[help]
        suggestion: Option<String>,
    },

    #[error("Compilation error: {message}")]
    #[diagnostic(code = "E0004")]
    CompilationFailed { message: String },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub struct DiagnosticBuilder;

impl DiagnosticBuilder {
    pub fn unsupported_feature(
        source: &str,
        line: usize,
        col: usize,
        feature: &str,
        suggestion: Option<String>,
    ) -> CompileError {
        let offset: usize = line.saturating_mul(80).saturating_add(col);
        let span = SourceSpan::new(offset.into(), 1usize);
        CompileError::UnsupportedFeature {
            src: NamedSource::new("input.py", source.to_string()),
            span,
            message: feature.to_string(),
            suggestion,
        }
    }

    pub fn type_error(source: &str, message: &str) -> CompileError {
        let span = SourceSpan::new(0usize.into(), 10usize);
        CompileError::TypeError {
            src: NamedSource::new("input.py", source.to_string()),
            span,
            message: message.to_string(),
        }
    }

    pub fn compilation_failed(message: &str) -> CompileError {
        CompileError::CompilationFailed {
            message: message.to_string(),
        }
    }
}

pub fn print_success(message: &str) {
    println!("✓ {}", message);
}

pub fn print_info(message: &str) {
    println!("ℹ {}", message);
}

pub fn print_warning(message: &str) {
    eprintln!("⚠ {}", message);
}

pub fn print_error(message: &str) {
    eprintln!("✗ {}", message);
}
