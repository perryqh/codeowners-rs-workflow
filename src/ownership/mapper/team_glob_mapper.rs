use std::sync::Arc;

use super::Entry;
use super::{Mapper, OwnerMatcher};
use crate::project::Project;

pub struct TeamGlobMapper {
    project: Arc<Project>,
}

impl TeamGlobMapper {
    pub fn build(project: Arc<Project>) -> Self {
        Self { project }
    }
}

impl Mapper for TeamGlobMapper {
    fn entries(&self) -> Vec<Entry> {
        let mut entries: Vec<Entry> = Vec::new();

        for team in &self.project.teams {
            for owned_glob in &team.owned_globs {
                entries.push(Entry {
                    path: owned_glob.to_owned(),
                    github_team: team.github_team.to_owned(),
                    team_name: team.name.to_owned(),
                    disabled: team.avoid_ownership,
                });
            }
        }

        entries
    }

    fn owner_matchers(&self) -> Vec<OwnerMatcher> {
        let mut owner_matchers: Vec<OwnerMatcher> = Vec::new();

        for team in &self.project.teams {
            for owned_glob in &team.owned_globs {
                owner_matchers.push(OwnerMatcher::Glob {
                    glob: owned_glob.clone(),
                    team_name: team.github_team.clone(),
                    source: "team_glob_mapper".to_owned(),
                })
            }
        }

        owner_matchers
    }

    fn name(&self) -> String {
        "Team-specific owned globs".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::common_test::tests::{build_ownership_with_all_mappers, build_ownership_with_team_glob_codeowners};

    use super::*;
    #[test]
    fn test_entries() -> Result<(), Box<dyn Error>> {
        let ownership = build_ownership_with_all_mappers()?;
        let mapper = TeamGlobMapper::build(ownership.project.clone());
        let entries = mapper.entries();
        assert_eq!(
            entries,
            vec![Entry {
                path: "packs/bar/**".to_owned(),
                github_team: "@Baz".to_owned(),
                team_name: "Baz".to_owned(),
                disabled: false
            }]
        );
        Ok(())
    }

    #[test]
    fn test_owner_matchers() -> Result<(), Box<dyn Error>> {
        let ownership = build_ownership_with_team_glob_codeowners()?;
        let mapper = TeamGlobMapper::build(ownership.project.clone());
        let mut owner_matchers = mapper.owner_matchers();
        owner_matchers.sort_by_key(|e| match e {
            OwnerMatcher::Glob { glob, .. } => glob.clone(),
            OwnerMatcher::ExactMatches(_, source) => source.clone(),
        });
        let expected_owner_matchers = vec![OwnerMatcher::Glob {
            glob: "packs/bar/**".to_owned(),
            team_name: "@Baz".to_owned(),
            source: "team_glob_mapper".to_owned(),
        }];
        assert_eq!(owner_matchers, expected_owner_matchers);
        Ok(())
    }
}
