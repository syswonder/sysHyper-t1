/* Copyright (c) [2023] [Syswonder Community]
 *   [sysHyper-t1] is licensed under Mulan PSL v2.
 *   You can use this software according to the terms and conditions of the Mulan PSL v2.
 *   You may obtain a copy of Mulan PSL v2 at:
 *               http://license.coscl.org.cn/MulanPSL2
 *   THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
 *   See the Mulan PSL v2 for more details.
 */

/// The error type for RVM operations.
#[derive(Debug)]
pub enum RvmError {
    AlreadyExists,
    BadState,
    InvalidParam,
    OutOfMemory,
    ResourceBusy,
    Unsupported,
}

/// A [`Result`] type with [`RvmError`] as the error type.
pub type RvmResult<T = ()> = Result<T, RvmError>;

macro_rules! rvm_err_type {
    ($err: ident) => {{
        use $crate::error::RvmError::*;
        warn!("[RvmError::{:?}]", $err);
        $err
    }};
    ($err: ident, $msg: expr) => {{
        use $crate::error::RvmError::*;
        warn!("[RvmError::{:?}] {}", $err, $msg);
        $err
    }};
}

macro_rules! rvm_err {
    ($err: ident) => {
        Err(rvm_err_type!($err))
    };
    ($err: ident, $msg: expr) => {
        Err(rvm_err_type!($err, $msg))
    };
}
