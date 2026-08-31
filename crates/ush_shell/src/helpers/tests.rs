mod lambdas;
mod streams;
mod syntax;

use super::{HelperInvocation, ValueStream};

#[test]
fn len_counts_lines() {
    let helper = HelperInvocation::parse("len")
        .expect("helper")
        .expect("parse");
    let (output, _) = helper
        .execute(ValueStream::Text("a\nb\n".to_string()))
        .expect("execute");
    assert_eq!(output.to_text().expect("text"), "2\n");
}

#[test]
fn length_remains_as_compat_alias() {
    let helper = HelperInvocation::parse("length")
        .expect("helper")
        .expect("parse");
    let (output, _) = helper
        .execute(ValueStream::Text("a\nb\n".to_string()))
        .expect("execute");
    assert_eq!(output.to_text().expect("text"), "2\n");
}

#[test]
fn html_and_xml_helpers_are_recognized() {
    let html = HelperInvocation::parse("html")
        .expect("helper")
        .expect("parse");
    let xml = HelperInvocation::parse("xml")
        .expect("helper")
        .expect("parse");
    assert!(matches!(html.kind, super::HelperKind::Html));
    assert!(matches!(xml.kind, super::HelperKind::Xml));
}

#[test]
fn lambda_helpers_support_custom_args_and_constants() {
    let helper = HelperInvocation::parse(r#"map(\line -> { upper(line) })"#)
        .expect("helper")
        .expect("parse");
    let (output, _) = helper
        .execute(ValueStream::Text("ush\n".to_string()))
        .expect("execute");
    assert_eq!(output.to_text().expect("text"), "USH\n");

    let helper = HelperInvocation::parse(r#"map(\-> { "ok" })"#)
        .expect("helper")
        .expect("parse");
    let (output, _) = helper
        .execute(ValueStream::Text("a\nb\n".to_string()))
        .expect("execute");
    assert_eq!(output.to_text().expect("text"), "ok\nok\n");
}

#[test]
fn functional_aliases_are_recognized() {
    let fmap = HelperInvocation::parse(r#"fmap(\it -> upper(it))"#)
        .expect("helper")
        .expect("parse");
    let ffmap = HelperInvocation::parse(r#"ffmap(\head, rest -> [head, rest])"#)
        .expect("helper")
        .expect("parse");
    let ffilter = HelperInvocation::parse(r#"ffilter(\it -> contains(it, "u"))"#)
        .expect("helper")
        .expect("parse");
    let fany = HelperInvocation::parse(r#"fany(\it -> contains(it, "u"))"#)
        .expect("helper")
        .expect("parse");
    let fsome = HelperInvocation::parse(r#"fsome(\it -> contains(it, "u"))"#)
        .expect("helper")
        .expect("parse");
    assert!(matches!(fmap.kind, super::HelperKind::Map(_)));
    assert!(matches!(ffmap.kind, super::HelperKind::Flat(_)));
    assert!(matches!(ffilter.kind, super::HelperKind::Filter(_)));
    assert!(matches!(fany.kind, super::HelperKind::Any(_)));
    assert!(matches!(fsome.kind, super::HelperKind::Some(_)));
}

#[test]
fn enumerate_and_swap_helpers_are_recognized() {
    let enumerate = HelperInvocation::parse("enumerate(1)")
        .expect("helper")
        .expect("parse");
    let swap = HelperInvocation::parse("swap")
        .expect("helper")
        .expect("parse");
    assert!(matches!(
        enumerate.kind,
        super::HelperKind::Sequence(super::sequence::SequenceOp::Enumerate(1))
    ));
    assert!(matches!(
        swap.kind,
        super::HelperKind::Sequence(super::sequence::SequenceOp::Swap)
    ));
}

#[test]
fn collection_helpers_are_recognized() {
    let reverse = HelperInvocation::parse("frev")
        .expect("helper")
        .expect("parse");
    let sort = HelperInvocation::parse("fsort")
        .expect("helper")
        .expect("parse");
    let unique = HelperInvocation::parse("funiq")
        .expect("helper")
        .expect("parse");
    let join = HelperInvocation::parse(r#"fjoin(",")"#)
        .expect("helper")
        .expect("parse");
    assert!(matches!(
        reverse.kind,
        super::HelperKind::Sequence(super::sequence::SequenceOp::Reverse)
    ));
    assert!(matches!(
        sort.kind,
        super::HelperKind::Sequence(super::sequence::SequenceOp::Sort)
    ));
    assert!(matches!(
        unique.kind,
        super::HelperKind::Sequence(super::sequence::SequenceOp::Unique)
    ));
    assert!(matches!(
        join.kind,
        super::HelperKind::Sequence(super::sequence::SequenceOp::Join(_))
    ));
}
