//! The mutation seam: serializable [`Command`]s + [`apply`], and
//! [`can_connect`] for single-connection validation.
//!
//! `Command` is data, not behaviour: ids are minted by the caller
//! (uuid → [`RefId`]) so a command replays deterministically on any model
//! state that carries the referenced ids. [`apply`] validates before
//! mutating and returns a new [`Model`], leaving the input untouched —
//! clients keep an undo stack of snapshots; stateless adapters (MCP, web)
//! apply one command per request.

use serde::{Deserialize, Serialize};

use crate::model::{ElementKind, Model, RefId, RelationKind};

// ---------------------------------------------------------------------------
// Single-connection validation
// ---------------------------------------------------------------------------

/// Why a connection `source -kind→ target` is invalid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnError {
    /// Source or target id resolves to no element in the model.
    UnknownElement(RefId),
    /// Source and target resolve to the same element.
    SelfLoop(RefId),
    /// The metamodel derivation matrix forbids `source -kind→ target`.
    Disallowed {
        source: ElementKind,
        kind: RelationKind,
        target: ElementKind,
    },
}

impl std::fmt::Display for ConnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnError::UnknownElement(id) => write!(f, "unknown element `{id}`"),
            ConnError::SelfLoop(id) => write!(f, "`{id}` cannot connect to itself"),
            ConnError::Disallowed {
                source,
                kind,
                target,
            } => write!(f, "{source} cannot {kind} {target}"),
        }
    }
}

impl std::error::Error for ConnError {}

/// Validates a single connection against the metamodel derivation matrix
/// (the same rules as whole-model validation, at drag-and-drop granularity).
pub fn can_connect(
    model: &Model,
    source: &RefId,
    target: &RefId,
    kind: RelationKind,
) -> Result<(), ConnError> {
    let Some(src) = model.resolve_element(source) else {
        return Err(ConnError::UnknownElement(source.clone()));
    };
    let Some(tgt) = model.resolve_element(target) else {
        return Err(ConnError::UnknownElement(target.clone()));
    };
    if src == tgt {
        return Err(ConnError::SelfLoop(source.clone()));
    }
    let (s, t) = (model[src].kind, model[tgt].kind);
    crate::validate::validate_relationship(s, t, kind)
        .map_err(|_| ConnError::Disallowed {
            source: s,
            kind,
            target: t,
        })
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

/// A model mutation as data.
///
/// Every variant carries caller-minted [`RefId`]s, so a serialized command is
/// replayable: applying the same command stream to the same starting model
/// yields the same model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum Command {
    AddElement {
        id: RefId,
        kind: ElementKind,
        name: String,
    },
    AddRelation {
        id: RefId,
        kind: RelationKind,
        source: RefId,
        target: RefId,
    },
    /// Removes the element and cascades to every relationship attached to it
    /// (dangling endpoints are unrepresentable in a valid model).
    DeleteElement {
        id: RefId,
    },
    DeleteRelation {
        id: RefId,
    },
    RenameElement {
        id: RefId,
        name: String,
    },
}

/// Why [`apply`] rejected a [`Command`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplyError {
    /// The command mints a `RefId` the model already carries.
    DuplicateRef(RefId),
    UnknownElement(RefId),
    UnknownRelation(RefId),
    /// `AddRelation` rejected by [`can_connect`].
    InvalidConnection(ConnError),
}

impl From<ConnError> for ApplyError {
    fn from(e: ConnError) -> Self {
        ApplyError::InvalidConnection(e)
    }
}

impl std::fmt::Display for ApplyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplyError::DuplicateRef(id) => write!(f, "id `{id}` already exists in the model"),
            ApplyError::UnknownElement(id) => write!(f, "unknown element `{id}`"),
            ApplyError::UnknownRelation(id) => write!(f, "unknown relationship `{id}`"),
            ApplyError::InvalidConnection(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for ApplyError {}

/// Applies a command and returns the resulting model.
///
/// Input is never mutated; on error the caller's model is unchanged and the
/// error names the offending `RefId`. `AddRelation` routes through
/// [`can_connect`], so an invalid connection can never enter the model.
pub fn apply(model: &Model, command: &Command) -> Result<Model, ApplyError> {
    let mut m = model.clone();
    match command {
        Command::AddElement { id, kind, name } => {
            if m.resolve_element(id).is_some() {
                return Err(ApplyError::DuplicateRef(id.clone()));
            }
            m.add_element_with_id(&id.0, name, *kind);
        }
        Command::AddRelation {
            id,
            kind,
            source,
            target,
        } => {
            can_connect(&m, source, target, *kind)?;
            if m.resolve_relation(id).is_some() {
                return Err(ApplyError::DuplicateRef(id.clone()));
            }
            let (Some(src), Some(tgt)) = (m.resolve_element(source), m.resolve_element(target))
            else {
                return Err(ApplyError::UnknownElement(source.clone()));
            };
            m.link_with_id(&id.0, src, tgt, *kind);
        }
        Command::DeleteElement { id } => {
            if !m.remove_element(id) {
                return Err(ApplyError::UnknownElement(id.clone()));
            }
        }
        Command::DeleteRelation { id } => {
            if !m.remove_relation(id) {
                return Err(ApplyError::UnknownRelation(id.clone()));
            }
        }
        Command::RenameElement { id, name } => {
            let idx = m
                .resolve_element(id)
                .ok_or_else(|| ApplyError::UnknownElement(id.clone()))?;
            m.rename_element(idx, name.clone());
        }
    }
    Ok(m)
}
