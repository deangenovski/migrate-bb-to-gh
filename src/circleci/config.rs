use std::{collections::HashSet, str::FromStr};

use serde::{Deserialize, Serialize};

use self::raw::Context;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub contexts: HashSet<String>,
}

impl FromStr for Config {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let raw = serde_yaml::from_str::<raw::RawConfig>(s)?;

        let mut contexts = HashSet::<String>::new();

        raw.workflows
            .into_values()
            .filter(|w| matches!(w, raw::WorkflowEntry::Workflow(_)))
            .flat_map(|w| match w {
                raw::WorkflowEntry::Workflow(w) => w.jobs,
                _ => unreachable!(),
            })
            .flat_map(|j| j.into_values())
            .flat_map(|j| j.context)
            .for_each(|c| match c {
                Context::String(ctx) => {
                    contexts.insert(ctx);
                }
                Context::Vec(ctx) => {
                    ctx.into_iter().for_each(|c| {
                        contexts.insert(c);
                    });
                }
            });

        Ok(Config { contexts })
    }
}

mod raw {
    use std::collections::BTreeMap;

    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub(crate) struct RawConfig {
        pub workflows: BTreeMap<String, WorkflowEntry>,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub(crate) enum WorkflowEntry {
        Workflow(Workflow),
        Other(serde_yaml::Value),
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub(crate) struct Workflow {
        pub jobs: Vec<BTreeMap<String, Job>>,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub(crate) struct Job {
        pub context: Option<Context>,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub(crate) enum Context {
        String(String),
        Vec(Vec<String>),
    }
}
