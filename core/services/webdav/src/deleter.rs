// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::sync::Arc;

use http::StatusCode;

use super::core::parse_error;
use super::core::*;
use opendal_core::raw::oio;
use opendal_core::raw::*;
use opendal_core::*;

pub struct WebdavDeleter {
    core: Arc<WebdavCore>,
    ctx: OperationContext,
}

impl WebdavDeleter {
    pub fn new(core: Arc<WebdavCore>, ctx: OperationContext) -> Self {
        Self { core, ctx }
    }
}

impl oio::OneShotDelete for WebdavDeleter {
    async fn delete_once(&self, path: String, args: OpDelete) -> Result<()> {
        let resp = self.core.webdav_delete(&self.ctx, &path, &args).await?;

        let status = resp.status();
        match status {
            StatusCode::NO_CONTENT => Ok(()),
            // Deleting a missing target is a no-op, unless the caller required a
            // matching ETag: a positive match cannot be satisfied by nothing.
            StatusCode::NOT_FOUND if args.if_match().is_some() => Err(Error::new(
                ErrorKind::ConditionNotMatch,
                format!("delete {path} failed: the target does not exist"),
            )
            .with_operation(Operation::Delete)),
            StatusCode::NOT_FOUND => Ok(()),
            _ => Err(parse_error(
                ErrorContext::new(ServiceOperation("Delete")),
                resp,
            )),
        }
    }
}
