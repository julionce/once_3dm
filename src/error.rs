use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ErrorKind {
    BufferSizeMismatch,
    FromUtf16Error,
    InvalidChunkSize,
    InvalidChunkTypeCode,
    InvalidChunkVersion,
    InvalidCompressionMode,
    InvalidDistanceDisplayMode,
    InvalidHeader,
    InvalidKnotsCount,
    InvalidLengthUnitSystem,
    InvalidVersion,
    InvalidSequenceLength,
    InvalidStringLength,
    IoError,
    UnknownObjectId,
}

impl ErrorKind {
    pub fn as_str(&self) -> &'static str {
        use ErrorKind::*;

        match *self {
            BufferSizeMismatch => "buffer size mismatch",
            FromUtf16Error => "from UTF-16 error",
            InvalidChunkSize => "invalid chunk size",
            InvalidChunkTypeCode => "invalid chunk type code",
            InvalidChunkVersion => "invalid chunk version",
            InvalidCompressionMode => "invalid compression mode",
            InvalidDistanceDisplayMode => "invalid distance display mode",
            InvalidHeader => "invalid header",
            InvalidKnotsCount => "invalid knot count",
            InvalidLengthUnitSystem => "invalid length unit system",
            InvalidVersion => "invalid version",
            InvalidSequenceLength => "invalid sequence length",
            InvalidStringLength => "invalid string length",
            IoError => "IO error",
            UnknownObjectId => "Unknown object Id",
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug)]
pub struct ErrorFrame {
    member: &'static str,
    ty: &'static str,
}

impl ErrorFrame {
    pub fn new(member: &'static str, ty: &'static str) -> Self {
        Self { member, ty }
    }
}

#[derive(Debug, Default)]
pub struct ErrorTrace {
    inner: VecDeque<ErrorFrame>,
}

impl std::fmt::Display for ErrorTrace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.inner
                .iter()
                .map(|frame| format!("\u{21aa} {}: {}", frame.member, frame.ty))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    FromUtf16Error(std::string::FromUtf16Error),
    Simple(ErrorKind),
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        use Error::*;
        match self {
            IoError(_) => ErrorKind::IoError,
            FromUtf16Error(_) => ErrorKind::FromUtf16Error,
            Simple(kind) => *kind,
        }
    }
}

#[derive(Debug)]
pub struct ErrorStack {
    trace: ErrorTrace,
    origin: Error,
}

impl ErrorStack {
    pub fn new(origin: Error) -> Self {
        Self {
            trace: ErrorTrace::default(),
            origin,
        }
    }

    pub fn push_frame(&mut self, member: &'static str, ty: &'static str) {
        self.trace.inner.push_back(ErrorFrame::new(member, ty))
    }

    pub fn pop_frame(&mut self) -> Option<ErrorFrame> {
        self.trace.inner.pop_back()
    }

    pub fn origin(&self) -> &Error {
        &self.origin
    }
}

impl std::fmt::Display for ErrorStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.origin {
            Error::IoError(e) => write!(f, "IoError: {}\n{}", e, self.trace),
            Error::Simple(kind) => write!(f, "Simple {}\n{}", kind.as_str(), self.trace),
            Error::FromUtf16Error(e) => write!(f, "{}\n{}", e, self.trace),
        }
    }
}

impl std::error::Error for ErrorStack {}
