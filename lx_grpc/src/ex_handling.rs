use std::fmt::Display;

use tonic::{Code, Status};

#[track_caller]
#[inline]
pub(crate) fn location() -> (&'static str, u32, u32) {
    let loc = std::panic::Location::caller();
    (loc.file(), loc.line(), loc.column())
}

pub trait GrpcCatch<T> {
    fn gcatch_cm(self, code: Code, msg: &str) -> Result<T, Status>;
    fn gcatch_c(self, code: Code) -> Result<T, Status>;
    fn gcatch_m(self, msg: &str) -> Result<T, Status>;
    fn gcatch_(self) -> Result<T, Status>;
}

pub trait GrpcIfNone<T> {
    fn gunwrap_cm(self, code: Code, msg: &str) -> Result<T, Status>;
    fn gunwrap_c(self, code: Code) -> Result<T, Status>;
    fn gunwrap_m(self, msg: &str) -> Result<T, Status>;
    fn gunwrap_(self) -> Result<T, Status>;
}

impl<T, E> GrpcCatch<T> for Result<T, E>
where
    E: Display + Send + Sync + 'static,
{
    #[track_caller]
    fn gcatch_cm(self, code: Code, msg: &str) -> Result<T, Status> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                let (file, line, column) = location();
                let msg = format!(
                    "Exception {} occured at {}:{}:{}\nContext: {}\nCaused by: {}",
                    code, file, line, column, msg, e
                );
                Err(Status::new(code, msg))
            }
        }
    }

    #[track_caller]
    fn gcatch_c(self, code: Code) -> Result<T, Status> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                let (file, line, column) = location();
                let msg = format!(
                    "Exception {} occured at {}:{}:{}\nCaused by: {}",
                    code, file, line, column, e
                );
                Err(Status::new(code, msg))
            }
        }
    }

    #[track_caller]
    fn gcatch_m(self, msg: &str) -> Result<T, Status> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                let code = Code::Internal;
                let (file, line, column) = location();
                let msg = format!(
                    "Exception {} occured at {}:{}:{}\nContext: {}\nCaused by: {}",
                    code, file, line, column, msg, e
                );
                Err(Status::new(code, msg))
            }
        }
    }

    #[track_caller]
    fn gcatch_(self) -> Result<T, Status> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                let code = Code::Internal;
                let (file, line, column) = location();
                let msg = format!(
                    "Exception {} occured at {}:{}:{}\nCaused by: {}",
                    code, file, line, column, e
                );
                Err(Status::new(code, msg))
            }
        }
    }
}

impl<T> GrpcIfNone<T> for Option<T> {
    #[track_caller]
    fn gunwrap_cm(self, code: Code, msg: &str) -> Result<T, Status> {
        match self {
            Some(v) => Ok(v),
            None => {
                let (file, line, column) = location();
                let msg = format!(
                    "Exception {} occured at {}:{}:{}\nContext: {}",
                    code, file, line, column, msg
                );
                Err(Status::new(code, msg))
            }
        }
    }

    #[track_caller]
    fn gunwrap_c(self, code: Code) -> Result<T, Status> {
        match self {
            Some(v) => Ok(v),
            None => {
                let msg = "Unexpected `Option::None`";
                let (file, line, column) = location();
                let msg = format!(
                    "Exception {} occured at {}:{}:{}\nContext: {}",
                    code, file, line, column, msg
                );
                Err(Status::new(code, msg))
            }
        }
    }

    #[track_caller]
    fn gunwrap_m(self, msg: &str) -> Result<T, Status> {
        match self {
            Some(v) => Ok(v),
            None => {
                let code = Code::DataLoss;
                let (file, line, column) = location();
                let msg = format!(
                    "Exception {} occured at {}:{}:{}\nContext: {}",
                    code, file, line, column, msg
                );
                Err(Status::new(code, msg))
            }
        }
    }

    #[track_caller]
    fn gunwrap_(self) -> Result<T, Status> {
        match self {
            Some(v) => Ok(v),
            None => {
                let code = Code::DataLoss;
                let msg = "Unexpected `Option::None`";
                let (file, line, column) = location();
                let msg = format!(
                    "Exception {} occured at {}:{}:{}\nContext: {}",
                    code, file, line, column, msg
                );
                Err(Status::new(code, msg))
            }
        }
    }
}

#[macro_export]
macro_rules! grpc_assert {
    ($cond:expr, $explanation:expr) => {
        if !($cond) {
            let loc = std::panic::Location::caller();
            let msg = format!(
                "Assertion {} failed at {}:{}:{}\nExplanation: {}",
                stringify!($cond),
                loc.file(),
                loc.line(),
                loc.column(),
                $explanation
            );
            let err = tonic::Status::new(tonic::Code::FailedPrecondition, &msg);
            return Err(err);
        }
    };
    ($cond:expr) => {
        if !($cond) {
            let loc = std::panic::Location::caller();
            let msg = format!(
                "Assertion {} failed at {}:{}:{}",
                stringify!($cond),
                loc.file(),
                loc.line(),
                loc.column()
            );
            let err = tonic::Status::new(tonic::Code::FailedPrecondition, &msg);
            return Err(err);
        }
    };
}

#[macro_export]
macro_rules! grpc_throw {
    ($ctx:expr) => {
        let loc = std::panic::Location::caller();
        let msg = format!(
            "Exception {} occured at {}:{}:{}\nContext: {}",
            tonic::Code::Internal,
            loc.file(),
            loc.line(),
            loc.column(),
            $ctx,
        );
        let err = tonic::Status::new(tonic::Code::Internal, &msg);
        Err(err)
    };
}
