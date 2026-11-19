//! Atomic directory operations within one server-configured workspace directory.
use super::{Credential, CredentialDraft};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File, OpenOptions},
    io::ErrorKind,
    path::Path,
};

#[derive(Default, Serialize, Deserialize)]
struct State {
    last_id: u64,
    entries: Vec<Credential>,
}
/// Safe, transport-independent failure categories.
#[derive(Debug, PartialEq, Eq)]
pub enum DirectoryError {
    Unavailable,
    Invalid(&'static str),
    Conflict(&'static str),
}
use DirectoryError::{Conflict, Invalid, Unavailable};
/// A read or revision-checked write to an authorized directory.
pub enum Operation {
    List,
    Create(CredentialDraft),
    Update(u64, u64, CredentialDraft),
    Delete(u64, u64),
}
/// Apply a transaction and return the committed workspace listing.
pub fn execute(root: &Path, op: Operation) -> Result<Vec<Credential>, DirectoryError> {
    fs::create_dir_all(root).map_err(|_| Unavailable)?;
    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(root.join("credentials.lock"))
        .map_err(|_| Unavailable)?;
    match op {
        Operation::List => FileExt::lock_shared(&lock),
        _ => FileExt::lock_exclusive(&lock),
    }
    .map_err(|_| Unavailable)?;
    let path = root.join("credentials.json");
    let mut state: State = match File::open(&path) {
        Ok(f) => serde_json::from_reader(f).map_err(|_| Unavailable)?,
        Err(e) if e.kind() == ErrorKind::NotFound => State::default(),
        Err(_) => return Err(Unavailable),
    };
    match op {
        Operation::List => return Ok(state.entries),
        Operation::Create(mut draft) => {
            validate(&mut draft, &state, None)?;
            state.last_id = state.last_id.checked_add(1).ok_or(Unavailable)?;
            state.entries.push(Credential {
                id: state.last_id,
                revision: 1,
                metadata: draft,
            });
        }
        Operation::Update(id, revision, mut draft) => {
            validate(&mut draft, &state, Some(id))?;
            let index = locate(&state, id, revision)?;
            let row = &mut state.entries[index];
            row.metadata = draft;
            row.revision = row.revision.checked_add(1).ok_or(Unavailable)?;
        }
        Operation::Delete(id, revision) => {
            let index = locate(&state, id, revision)?;
            state.entries.remove(index);
        }
    }
    let mut tmp = tempfile::NamedTempFile::new_in(root).map_err(|_| Unavailable)?;
    serde_json::to_writer(&mut tmp, &state).map_err(|_| Unavailable)?;
    tmp.as_file().sync_all().map_err(|_| Unavailable)?;
    tmp.persist(path).map_err(|_| Unavailable)?;
    Ok(state.entries)
}
fn locate(state: &State, id: u64, revision: u64) -> Result<usize, DirectoryError> {
    state
        .entries
        .iter()
        .position(|r| r.id == id && r.revision == revision)
        .ok_or(Conflict(
            "Record changed or removed. Refresh the directory.",
        ))
}
fn validate(d: &mut CredentialDraft, s: &State, id: Option<u64>) -> Result<(), DirectoryError> {
    d.name = d.name.trim().into();
    d.alias = d.alias.trim().into();
    if d.name.is_empty() || d.name.len() > 100 || d.name.chars().any(char::is_control) {
        return Err(Invalid("Enter a name of up to 100 bytes."));
    }
    if d.alias.is_empty()
        || d.alias.len() > 128
        || !d
            .alias
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"_-.".contains(&c))
    {
        return Err(Invalid(
            "Use letters, digits, periods, underscores or hyphens for the alias.",
        ));
    }
    if s.entries.iter().any(|r| {
        Some(r.id) != id
            && (r.metadata.name.eq_ignore_ascii_case(&d.name) || r.metadata.alias == d.alias)
    }) {
        return Err(Conflict("A reference already uses that name or alias."));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn references_are_persistent_isolated_and_revision_checked() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        let run = |op| execute(first.path(), op);
        let mut draft = CredentialDraft {
            name: "Partner".into(),
            alias: "partner-key".into(),
            ..Default::default()
        };
        let created = run(Operation::Create(draft.clone())).unwrap().remove(0);
        assert_eq!(
            run(Operation::List).unwrap(),
            std::slice::from_ref(&created)
        );
        assert!(execute(second.path(), Operation::List).unwrap().is_empty());
        assert!(run(Operation::Create(draft.clone())).is_err());
        draft.alias = "rotated-key".into();
        let updated = run(Operation::Update(created.id, 1, draft))
            .unwrap()
            .remove(0);
        assert_eq!(updated.revision, 2);
        assert!(run(Operation::Delete(created.id, 1)).is_err());
        assert!(run(Operation::Delete(created.id, 2)).unwrap().is_empty());
    }
}
