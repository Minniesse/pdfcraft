//! `pdfcraft-js` stand-in for a build without the `js` feature: the same data types, read from the
//! script engine's own model files (so there is one definition of each), and entry points that run
//! nothing. No JavaScript or FormCalc interpreter is compiled in: an explicit run answers
//! [`NOT_AVAILABLE`](crate::js::NOT_AVAILABLE), and the scripts a document carries never run.

// The model files also hold helpers only the script engine itself calls.
#[allow(dead_code, reason = "shared with pdfcraft-js, whose interpreter uses the rest")]
#[path = "../../../js/src/model.rs"]
mod model;
pub use model::{
    DISPLAY_HIDDEN, DISPLAY_NO_PRINT, DISPLAY_NO_VIEW, DISPLAY_VISIBLE, DocInfo, Event, FieldState, FieldType, Layer, Limits, Message,
    MessageButtons, MessageIcon, Outcome, Panel, Request, Timer,
};

#[path = "../../../js/src/geometry.rs"]
mod geometry;
pub use geometry::{rotate_rect, unrotate_rect};

use crate::js::NOT_AVAILABLE;

/// Maximum reset calls admitted by one document invocation (as in pdfcraft-js).
pub const MAX_RESET_CALLS: usize = 64;

/// A run that did nothing: the event keeps its value and the error says why.
fn not_run(event: &Event) -> Outcome {
    Outcome {
        execution_refused: true,
        realm_untouched: true,
        rc: true,
        value: event.value.clone(),
        change: event.change.clone(),
        error: Some(NOT_AVAILABLE.into()),
        ..Default::default()
    }
}

/// No script engine, so no date names to set.
pub fn set_date_names(_: [&'static str; 12], _: [&'static str; 12], _: [&'static str; 7], _: [&'static str; 7]) {}

/// No script engine, so no date language: `run` runs as it is (the web build's calculate path).
#[cfg(target_arch = "wasm32")]
pub fn with_english_printd<R>(_english: bool, run: impl FnOnce() -> R) -> R {
    run()
}

/// The web build's calculate path: runs nothing.
#[cfg(target_arch = "wasm32")]
pub fn run_shared_calculations(
    _script: &str,
    event: &Event,
    _doc: &DocInfo,
    _fields: std::sync::Arc<Vec<FieldState>>,
    _doc_scripts: &[String],
    _calculations: &[(usize, String)],
    _limits: Limits,
) -> Outcome {
    not_run(event)
}

pub mod interaction {
    pub use super::model::{InteractiveEvent, MenuItem, Prompt};
}

pub mod session {
    pub use super::model::Job;
    use super::{FieldState, Outcome, not_run};
    use std::sync::Arc;

    /// A document's script realm, with no engine behind it: every run answers
    /// [`crate::js::NOT_AVAILABLE`] and nothing is ever pending.
    #[derive(Default)]
    pub struct ScriptSession;

    impl ScriptSession {
        pub fn stamp_fields(&mut self, _fields: &mut Arc<Vec<FieldState>>) {}

        pub fn sync_field_ids(&mut self, _ids: impl IntoIterator<Item = u64>) {}

        pub fn retire_icon_handles(&mut self) {}

        pub fn retire_changed_icons(&mut self, _changed: impl FnMut(u64) -> bool) {}

        #[cfg(not(target_arch = "wasm32"))]
        pub fn start(&mut self, _job: Job) -> Result<std::sync::mpsc::Receiver<super::interaction::InteractiveEvent>, String> {
            Err(crate::js::NOT_AVAILABLE.into())
        }

        pub fn is_executing(&self) -> bool {
            false
        }

        pub fn is_pending(&self) -> bool {
            false
        }

        pub fn acknowledge_completion(&mut self) {}

        pub fn retire_timed_out_worker(&mut self) {}

        pub fn fresh_realm(&mut self) -> Self {
            Self
        }

        pub fn reset_after_refusal(&mut self) {}

        pub fn run(&mut self, job: Job, _answers: &[Option<String>]) -> Outcome {
            not_run(&job.event)
        }

        pub fn run_with_limit(&mut self, job: Job, _answers: &[Option<String>], _limit: std::time::Duration) -> Outcome {
            not_run(&job.event)
        }

        pub fn run_english_retry(&mut self, job: Job, _limit: std::time::Duration) -> Outcome {
            not_run(&job.event)
        }
    }
}

// The XFA model file names `HashSet`, `Message` and `Panel` from its parent module.
use std::collections::HashSet;

#[allow(dead_code, reason = "shared with pdfcraft-js, whose interpreter uses the rest")]
#[path = "../../../js/src/xfa/model.rs"]
mod xfa_model;

pub mod xfa {
    pub use super::xfa_model::{Script, XfaDoc, XfaEffect, XfaEvent, XfaKind, XfaNode, XfaOutcome, XfaProp, XfaProps};
}

pub mod xfa_session {
    pub use super::xfa::Script;
    use super::xfa::{XfaDoc, XfaOutcome};

    /// The scripts of one event, with no engine behind them: each run answers
    /// [`crate::js::NOT_AVAILABLE`] and changes nothing.
    pub struct XfaSession;

    impl XfaSession {
        pub fn new(_doc: XfaDoc) -> XfaSession {
            XfaSession
        }

        pub fn with_time_scale(self, _factor: u32) -> XfaSession {
            self
        }

        pub fn run(&mut self, _script: Script) -> XfaOutcome {
            XfaOutcome { error: Some(crate::js::NOT_AVAILABLE.into()), ..Default::default() }
        }
    }
}
