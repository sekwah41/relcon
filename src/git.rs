use std::collections::HashMap;
use std::error::Error;
use gix::{ObjectId, Repository};
use regex::Regex;

pub struct LatestTagInfo {
    tag: String,
    id: ObjectId,
}

/// The most recent tag reachable from HEAD which matches vx.x.x
pub fn latest_tag(repo: &Repository) -> Result<Option<LatestTagInfo>, Box<dyn Error + Send + Sync>> {
    log::debug!("Getting latest tag");
    let refs = repo.references().map_err(|e| {
        log::error!("Failed to get repo references {}", e);
        e
    })?;
    let tags = refs.tags().map_err(|e| {
        log::error!("Failed to get repo tags {}", e);
        e
    })?;

    let version_tag = Regex::new(r"^v\d+\.\d+\.\d+$")?;

    let mut tagged_commits: HashMap<ObjectId, String> = HashMap::new();
    for tag in tags {
        let mut tag = tag.map_err(|e| {
            log::error!("Failed to get tag {}", e);
            e
        })?;

        let name = tag.name().shorten().to_string();

        if !version_tag.is_match(&name) {
            log::trace!("Skipping tag {} as it does not match vx.x.x", name);
            continue;
        }

        let id = tag.peel_to_id()?.detach();
        log::trace!("Tag: {} -> {}", name, id);
        tagged_commits.insert(id, name);
    }

    if tagged_commits.is_empty() {
        return Ok(None);
    }

    let head = repo.head_commit()?;
    for info in head.ancestors().all()? {
        let info = info?;
        if let Some(name) = tagged_commits.remove(&info.id) {
            log::debug!("Latest tag {} at commit {}", name, info.id);
            return Ok(Some(LatestTagInfo {
                tag: name,
                id: info.id
            }));
        }
    }

    Ok(None)
}

/// TODO make it so Release-As: 0.0.0 is picked up from the body of release messages
/// TODO add and document a Release-Since: (commit hash) triggers generating changelogs since the specified hash.
pub fn commit_history(repo: &Repository) -> Result<(), Box<dyn Error + Send + Sync>> {
    let head = repo.head_commit()?;

    for info in head.ancestors().all()? {
        let info = info?;

        let commit = info.object()?;
        let message = commit.message()?;
        log::trace!("Checking commit {:?}", message.summary());
    }

    Ok(())
}