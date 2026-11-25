//! The engine built without the `js` feature (the KatWork build profile): no script engine is
//! compiled in, so JavaScript stays off whatever the preference says, an opened document's XFA
//! and document scripts never run, and an explicit run answers "not available in this build".
//! `cargo test -p pdfcraft-engine --no-default-features --test katwork_profile`.
#![cfg(not(feature = "js"))]

use std::sync::Arc;

use pdfcraft_engine::{Edit, Session};

#[test]
fn javascript_stays_off_and_xfa_scripts_never_run_on_open() {
    let mut s = Session::new().with_clock(|| 1_700_000_000);
    s.set_javascript(true);
    assert!(!s.javascript(), "there is no script engine to turn on");
    // The template's initialize scripts (one JavaScript, one FormCalc) would set qty to 2.
    for template in [
        pdfcraft_xfa::fixtures::scripted_template(),
        pdfcraft_xfa::fixtures::scripted_template().replace("application/x-javascript", "application/x-formcalc"),
    ] {
        let id = s.open("scripted.pdf", None, Arc::new(pdfcraft_xfa::fixtures::shell(&template)), None).expect("opens");
        let qty = s.get(id).unwrap().form.iter().find(|f| f.name == "qty").unwrap().value.clone();
        assert!(qty.is_empty(), "no initialize script ran: {qty:?}");
        let e = s.run_javascript(id, "1 + 1", None).unwrap_err().to_string();
        assert!(e.contains(pdfcraft_engine::js::NOT_AVAILABLE), "{e}");
        let e = s.run_javascript(id, "", Some("addRow")).unwrap_err().to_string();
        assert!(e.contains(pdfcraft_engine::js::NOT_AVAILABLE), "a button's click script: {e}");
    }
}

#[test]
fn a_script_inside_a_batch_is_refused_and_nothing_is_stored() {
    let mut s = Session::new().with_clock(|| 1_700_000_000);
    let id = s.open("form.pdf", None, Arc::new(pdfcraft_xfa::fixtures::shell(&pdfcraft_xfa::fixtures::scripted_template())), None).expect("opens");
    let dirty = s.get(id).unwrap().dirty;
    let script = Edit::SetDocumentScript { name: "init".into(), script: Some("app.alert(1);".into()) };
    let batch = Edit::Batch { label: "Several".into(), edits: vec![script] };
    let e = s.apply(id, batch).unwrap_err().to_string();
    assert!(e.contains(pdfcraft_engine::js::NOT_AVAILABLE), "{e}");
    assert_eq!(s.get(id).unwrap().dirty, dirty, "nothing was stored");
}
