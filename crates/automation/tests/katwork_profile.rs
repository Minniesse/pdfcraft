//! The tools built without the `js` feature (the KatWork build profile): no script engine is
//! compiled in, so a document's scripts never run and the script tools answer "not available in
//! this build" and store nothing.
//! `cargo test -p pdfcraft-automation --no-default-features --features mcp --test katwork_profile`.
#![cfg(not(feature = "js"))]

use pdfcraft_automation::{Automation, Content};
use serde_json::{Value, json};

const NOT_AVAILABLE: &str = "not available in this build";

fn tool(a: &mut Automation, name: &str, args: Value) -> Value {
    match a.call(name, &args).unwrap_or_else(|e| panic!("{name}: {e}")).remove(0) {
        Content::Json(value) => value,
        other => panic!("expected JSON: {other:?}"),
    }
}

/// The error a tool answers with; panics when it succeeds.
fn refused(a: &mut Automation, name: &str, args: Value) -> String {
    match a.call(name, &args) {
        Ok(out) => panic!("{name} succeeded: {out:?}"),
        Err(e) => e.to_string(),
    }
}

fn pdf_of(objs: &[String]) -> Vec<u8> {
    let mut out = b"%PDF-1.7\n".to_vec();
    let mut offsets = Vec::new();
    for (i, o) in objs.iter().enumerate() {
        offsets.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n{o}\nendobj\n", i + 1).as_bytes());
    }
    let xref = out.len();
    out.extend_from_slice(format!("xref\n0 {}\n0000000000 65535 f \n", objs.len() + 1).as_bytes());
    for o in offsets {
        out.extend_from_slice(format!("{o:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(format!("trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n", objs.len() + 1).as_bytes());
    out
}

/// Field `a`; field `total` (stored value `1`) whose calculate script sets it to `9999`; field
/// `log`, which the open action, a document-level script and `a`'s validate script would write.
fn form() -> Vec<u8> {
    pdf_of(&[
        "<< /Type /Catalog /Pages 2 0 R /OpenAction 7 0 R /Names << /JavaScript << /Names [(init) 8 0 R] >> >> /AcroForm << /Fields [4 0 R 5 0 R 6 0 R] /CO [5 0 R] /DA (/Helv 0 Tf 0 g) /DR << /Font << /Helv 9 0 R >> >> >> >>".into(),
        "<< /Type /Pages /Kids [3 0 R] /Count 1 /MediaBox [0 0 200 300] >>".into(),
        "<< /Type /Page /Parent 2 0 R /Annots [4 0 R 5 0 R 6 0 R] >>".into(),
        "<< /Type /Annot /Subtype /Widget /FT /Tx /T (a) /V () /Rect [20 250 180 270] /P 3 0 R /F 4 /AA << /V << /S /JavaScript /JS (getField\\('log'\\).value = 'validated';) >> >> >>".into(),
        "<< /Type /Annot /Subtype /Widget /FT /Tx /T (total) /V (1) /Rect [20 200 180 220] /P 3 0 R /F 4 /AA << /C << /S /JavaScript /JS (event.value = '9999';) >> >> >>".into(),
        "<< /Type /Annot /Subtype /Widget /FT /Tx /T (log) /V () /Rect [20 150 180 170] /P 3 0 R /F 4 >>".into(),
        "<< /S /JavaScript /JS (getField\\('log'\\).value = 'opened';) >>".into(),
        "<< /S /JavaScript /JS (getField\\('log'\\).value = 'document script';) >>".into(),
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>".into(),
    ])
}

fn open(a: &mut Automation, name: &str) -> u64 {
    let dir = std::env::temp_dir().join(format!("pdfcraft-katwork-profile-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("form.pdf"), form()).unwrap();
    *a = Automation::new().with_root(&dir).unwrap();
    tool(a, "doc_open", json!({ "path": "form.pdf" }))["doc"].as_u64().unwrap()
}

/// The document as `doc_list` describes it (its `dirty` flag among the rest).
fn state(a: &mut Automation, doc: u64) -> Value {
    let list = tool(a, "doc_list", json!({}));
    list["documents"].as_array().unwrap().iter().find(|d| d["doc"] == doc).cloned().unwrap()
}

/// Field `field`'s value as text (an empty field has none).
fn value(a: &mut Automation, doc: u64, field: &str) -> String {
    let f = tool(a, "form_fields", json!({ "doc": doc }));
    let v = f["fields"].as_array().unwrap().iter().find(|f| f["name"] == field).map(|f| f["value"].clone()).unwrap();
    v.as_str().unwrap_or_default().to_string()
}

#[test]
fn document_scripts_never_run() {
    let mut a = Automation::new();
    let doc = open(&mut a, "never-run");
    // Opening ran neither the open action nor the document-level script.
    assert_eq!(value(&mut a, doc, "log"), "");
    tool(&mut a, "form_fill", json!({ "doc": doc, "values": { "a": "5" } }));
    assert_eq!(value(&mut a, doc, "a"), "5", "the value itself is stored");
    assert_eq!(value(&mut a, doc, "total"), "1", "the calculate script did not run");
    assert_eq!(value(&mut a, doc, "log"), "", "the validate script did not run");
    let e = refused(&mut a, "js_run", json!({ "doc": doc, "script": "getField('total').value = 'ran'; 1 + 1" }));
    assert!(e.contains(NOT_AVAILABLE), "{e}");
    assert_eq!(value(&mut a, doc, "total"), "1");
    // JavaScript cannot be turned on in this build.
    assert_eq!(tool(&mut a, "js_enabled", json!({ "enabled": true }))["enabled"], false);
    tool(&mut a, "form_fill", json!({ "doc": doc, "values": { "a": "6" } }));
    assert_eq!(value(&mut a, doc, "total"), "1", "still no calculation after asking for JavaScript");
}

#[test]
fn script_tools_answer_not_available_and_store_nothing() {
    let mut a = Automation::new();
    let doc = open(&mut a, "store-nothing");
    let before = state(&mut a, doc);
    let scripts = tool(&mut a, "js_document_scripts", json!({ "doc": doc }));
    let e = refused(&mut a, "js_set_document_script", json!({ "doc": doc, "name": "more", "script": "app.alert('x');" }));
    assert!(e.contains(NOT_AVAILABLE), "{e}");
    let e = refused(&mut a, "js_set_document_script", json!({ "doc": doc, "name": "init" }));
    assert!(e.contains(NOT_AVAILABLE), "deleting one is refused too: {e}");
    let e = refused(&mut a, "form_set_script", json!({ "doc": doc, "field": "a", "event": "calculate", "script": "event.value = 1;" }));
    assert!(e.contains(NOT_AVAILABLE), "{e}");
    assert_eq!(tool(&mut a, "js_document_scripts", json!({ "doc": doc })), scripts, "the document's scripts are unchanged");
    assert_eq!(state(&mut a, doc), before, "nothing was stored");
    assert_eq!(before["dirty"], false);
}
