use thiserror::Error;

#[derive(Error, Debug)]
pub enum StateError {
    #[error("Storage access error")]
    StorageIoError(#[from] rocksdb::Error),
    #[error("Storage data format error, not 256 bits")]
    StorageDataFormatErr,

    #[error("Get ColumnFamily empty")]
    ColumnFamilyEmpty,

    #[error("VM execute error:{0}")]
    VmExecError(String),

    #[error("VM json serde error")]
    JsonSerdeError(#[from] serde_json::Error),

    #[error("Can not load the program: {0}")]
    GetProgramError(String),

    #[error("Convert string error")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error("IO operations error")]
    FileIOError(#[from] std::io::Error),

    #[error("Process context empty")]
    ProcessContextEmpty,

    #[error("ExeEnd step empty")]
    ExeEndStepEmpty,

    #[error("Generate storage table error")]
    GenStorageTableError(#[from] crate::vm::error::ProcessorError),

    #[error("Mutex lock error: {0}")]
    MutexLockError(String),

    #[error("Empty array error: {0}")]
    EmptyArrayError(String),

    #[error("Parse int error: {0}")]
    ParseIntError(String),

    #[error("VM is not alive, batch has been finished!")]
    VMNotAvaliable,
    #[error("Cannot make a call after finish batch!")]
    CalledAfterBatchFinished,
}
