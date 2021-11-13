#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AddFileResult(pub i32);
impl AddFileResult {
    pub const Added: Self = Self(0i32);
    pub const AlreadyAdded: Self = Self(1i32);
    pub const NotAllowed: Self = Self(2i32);
    pub const Unavailable: Self = Self(3i32);
}
impl ::core::marker::Copy for AddFileResult {}
impl ::core::clone::Clone for AddFileResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FileOpenPickerUI(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FileOpenPickerUI {}
impl ::core::clone::Clone for FileOpenPickerUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FileRemovedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FileRemovedEventArgs {}
impl ::core::clone::Clone for FileRemovedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FileSavePickerUI(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FileSavePickerUI {}
impl ::core::clone::Clone for FileSavePickerUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FileSelectionMode(pub i32);
impl FileSelectionMode {
    pub const Single: Self = Self(0i32);
    pub const Multiple: Self = Self(1i32);
}
impl ::core::marker::Copy for FileSelectionMode {}
impl ::core::clone::Clone for FileSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileOpenPickerUI(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileOpenPickerUI {}
impl ::core::clone::Clone for IFileOpenPickerUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileRemovedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileRemovedEventArgs {}
impl ::core::clone::Clone for IFileRemovedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileSavePickerUI(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileSavePickerUI {}
impl ::core::clone::Clone for IFileSavePickerUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPickerClosingDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPickerClosingDeferral {}
impl ::core::clone::Clone for IPickerClosingDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPickerClosingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPickerClosingEventArgs {}
impl ::core::clone::Clone for IPickerClosingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPickerClosingOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPickerClosingOperation {}
impl ::core::clone::Clone for IPickerClosingOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITargetFileRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITargetFileRequest {}
impl ::core::clone::Clone for ITargetFileRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITargetFileRequestDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITargetFileRequestDeferral {}
impl ::core::clone::Clone for ITargetFileRequestDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITargetFileRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITargetFileRequestedEventArgs {}
impl ::core::clone::Clone for ITargetFileRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PickerClosingDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PickerClosingDeferral {}
impl ::core::clone::Clone for PickerClosingDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PickerClosingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PickerClosingEventArgs {}
impl ::core::clone::Clone for PickerClosingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PickerClosingOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PickerClosingOperation {}
impl ::core::clone::Clone for PickerClosingOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SetFileNameResult(pub i32);
impl SetFileNameResult {
    pub const Succeeded: Self = Self(0i32);
    pub const NotAllowed: Self = Self(1i32);
    pub const Unavailable: Self = Self(2i32);
}
impl ::core::marker::Copy for SetFileNameResult {}
impl ::core::clone::Clone for SetFileNameResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TargetFileRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TargetFileRequest {}
impl ::core::clone::Clone for TargetFileRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TargetFileRequestDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TargetFileRequestDeferral {}
impl ::core::clone::Clone for TargetFileRequestDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TargetFileRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TargetFileRequestedEventArgs {}
impl ::core::clone::Clone for TargetFileRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}