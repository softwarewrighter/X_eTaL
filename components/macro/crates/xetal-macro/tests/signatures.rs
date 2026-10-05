//! Signature lines (T4, MC21): `s:u_se< :: Char -> Char -> Unit` in
//! System.xtlm declares a built-in system macro with no definition; its
//! type is read as the catalog's are; anywhere else `::` is refused.

use xetal_macro::{Found, Libraries, expand_library, system_signatures};

struct None_;

impl Libraries for None_ {
    fn find(&self, _spec: &str, _from: &str) -> Option<Found> {
        None
    }
}

#[test]
fn system_xtlm_declares_u_se_with_its_type_and_doc() {
    let sigs = system_signatures().unwrap();
    let u_se = sigs.iter().find(|s| s.name == "u_se<").unwrap();
    assert_eq!(u_se.ty, "Char -> Char -> Unit");
    assert!(
        u_se.doc.iter().any(|l| l.starts_with("Import")),
        "{:?}",
        u_se.doc
    );
    assert!(
        u_se.doc.iter().any(|l| l.starts_with(">> ")),
        "{:?}",
        u_se.doc
    );
}

#[test]
fn a_signature_line_in_system_xtlm_is_not_code() {
    let text = "## A built-in.\ns:u_se< :: Char -> Char -> Unit\ns:i_d< := { a b -> b }\n";
    assert!(expand_library("lib/System.xtlm", text, &None_).is_ok());
}

#[test]
fn a_signature_must_read_as_a_type() {
    let text = "s:u_se< :: Char -> Bogus\n";
    let err = expand_library("lib/System.xtlm", text, &None_).unwrap_err();
    assert_eq!(err.diagnostic.code, "bad-signature");
    let text = "u:f_ :: Char -> Char\n";
    let err = expand_library("lib/System.xtlm", text, &None_).unwrap_err();
    assert_eq!(err.diagnostic.code, "bad-signature");
}
