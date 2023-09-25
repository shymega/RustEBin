// SPDX-FileCopyrightText: Dom Rodriguez <shymega+rustebin@shymega.org.uk>
// SPDX-License-Identifier: AGPL-3.0-only

//! DB models module.

use uuid::Uuid;

pub(crate) struct PasteMetadata {
    pub id: Uuid,
    pub file_name: String,
}
