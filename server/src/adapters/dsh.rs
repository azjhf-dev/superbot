//! DeepSeek Harness CLI: `dsh --profile headless --json`.
//! One task per invocation; resumes a persisted Session with `--session-id <id>`.
//! dsh assigns new session ids itself and reports them in the opening `session`
//! event, so the first run captures the id instead of choosing one.
//! Events are committed messages, not token streams, so there is no TextDelta.
//! Model and effort follow the dsh profile's route (set them in `dsh web` or the
//! profile config): the headless CLI has no model flag.

use super::{describe, short, str_of, Adapter, AgentEvent, ModelOption, RunInput};
use serde_json::Value;

#[derive(Default)]
pub struct Dsh {
    saw_session: bool,
}

impl Adapter for Dsh {
    fn bin(&self) -> &'static str {
        "dsh"
    }

    fn args(&mut self, input: &RunInput, _out: &mut Vec<AgentEvent>) -> Vec<String> {
        let mut a: Vec<String> = vec![
            "--profile".into(), "headless".into(),
            "--json".into(),
            // Headless has no live approval channel, so Bots run unattended:
            // workspace writes allowed, anything needing approval is rejected.
            "--permission-mode".into(), "workspace-write".into(),
            "--approval-policy".into(), "never".into(),
        ];
        if let Some(id) = &input.session_id {
            self.saw_session = true;
            a.extend(["--session-id".into(), id.clone()]);
        }
        a.extend(["--".into(), input.prompt.clone()]);
        a
    }

    fn on_event(&mut self, e: &Value, out: &mut Vec<AgentEvent>) {
        // Field names follow the headless README's event contract. dsh is in
        // developer preview with breaking changes, so check these on upgrade.
        match str_of(e, "type") {
            "session" => {
                if !self.saw_session {
                    self.saw_session = true;
                    out.push(AgentEvent::Session(str_of(e, "sessionId").into()));
                }
            }
            "text" => {
                let t = str_of(e, "text");
                if !t.trim().is_empty() {
                    out.push(AgentEvent::Text(t.into()));
                }
            }
            "tool_call" => out.push(AgentEvent::Tool {
                name: str_of(e, "name").into(),
                detail: describe(&e["arguments"]),
            }),
            "final" => {
                let kind = str_of(&e["reason"], "kind");
                if !e["reason"].is_null() && kind != "completed" {
                    out.push(AgentEvent::Error(format!("dsh run ended: {}", short(&e["reason"].to_string()))));
                }
            }
            "error" => out.push(AgentEvent::Error(e["message"].as_str().unwrap_or("dsh failed").into())),
            _ => {}
        }
    }

    /// The headless CLI takes no model flag: the route is the dsh profile's
    /// agentDefaultModel, chosen in `dsh web` or the profile config. This entry
    /// is informational, so the picker shows the default DeepSeek route.
    fn models(&self) -> Vec<ModelOption> {
        vec![ModelOption {
            id: "deepseek-v4.1-flash".into(),
            label: "DeepSeek V4.1 Flash".into(),
            efforts: vec![],
            aliases: vec!["deepseek-flash".into()],
        }]
    }
}
