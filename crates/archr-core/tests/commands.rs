//! Seam tests for the mutation interface: stable `RefId`s, `can_connect`,
//! and `Command` replay through `apply`.

use archr_core::validate::validate_model;
use archr_core::{
    apply, can_connect, model_to_yaml, parse_yaml, ApplyError, Command, ConnError, ElementKind,
    Model, RefId, RelationKind,
};

/// e1=BusinessActor(0), e2=ApplicationComponent(1), e3=Node(2), r1: e2→e1 Serving.
fn base_model() -> Model {
    let mut m = Model::new("T");
    m.add_element_with_id("e1", "A", ElementKind::BusinessActor);
    m.add_element_with_id("e2", "B", ElementKind::ApplicationComponent);
    m.add_element_with_id("e3", "C", ElementKind::Node);
    m.link_with_id("r1", m.resolve_element(&RefId::new("e2")).unwrap(),
        m.resolve_element(&RefId::new("e1")).unwrap(), RelationKind::Serving);
    m
}

#[test]
fn ref_survives_delete_and_cascades_attached_relations() {
    let mut m = base_model();
    m.link_with_id("r2", m.resolve_element(&RefId::new("e3")).unwrap(),
        m.resolve_element(&RefId::new("e1")).unwrap(), RelationKind::Association);

    assert!(m.remove_element(&RefId::new("e2")));
    // Deleted ref is gone; survivor refs still resolve to the right elements.
    assert!(m.resolve_element(&RefId::new("e2")).is_none());
    let c = m.resolve_element(&RefId::new("e3")).unwrap();
    assert_eq!(m[c].name, "C");
    assert_eq!(m.element_count(), 2);

    // Cascade dropped the relation attached to e2; the untouched one survived
    // and its endpoints were remapped to the correct survivors.
    assert!(m.resolve_relation(&RefId::new("r1")).is_none());
    let r2 = m.resolve_relation(&RefId::new("r2")).unwrap();
    assert_eq!(m[m[r2].source].original_id, "e3");
    assert_eq!(m[m[r2].target].original_id, "e1");

    assert!(!m.remove_element(&RefId::new("nope")));

    // The model is still valid and round-trips through YAML.
    assert!(validate_model(&m).success);
    let reparsed = parse_yaml(&model_to_yaml(&m)).unwrap();
    assert_eq!(reparsed.element_count(), 2);
}

#[test]
fn can_connect_mirrors_the_metamodel_matrix() {
    let m = base_model();
    let (e1, e2, e3) = (RefId::new("e1"), RefId::new("e2"), RefId::new("e3"));

    // Known-valid: ApplicationComponent -Serving→ BusinessActor.
    assert_eq!(can_connect(&m, &e2, &e1, RelationKind::Serving), Ok(()));
    // Known-invalid (same triple the validator's own test uses).
    assert_eq!(
        can_connect(&m, &e1, &e3, RelationKind::Realization),
        Err(ConnError::Disallowed {
            source: ElementKind::BusinessActor,
            kind: RelationKind::Realization,
            target: ElementKind::Node,
        })
    );
    // Self-loop is caught before the matrix; unknown refs report the id.
    assert_eq!(
        can_connect(&m, &e1, &e1, RelationKind::Association),
        Err(ConnError::SelfLoop(e1.clone()))
    );
    assert_eq!(
        can_connect(&m, &RefId::new("ghost"), &e1, RelationKind::Association),
        Err(ConnError::UnknownElement(RefId::new("ghost")))
    );
}

fn editor_commands() -> Vec<Command> {
    vec![
        Command::AddElement {
            id: RefId::new("a1"),
            kind: ElementKind::BusinessActor,
            name: "A".into(),
        },
        Command::AddElement {
            id: RefId::new("a2"),
            kind: ElementKind::ApplicationComponent,
            name: "B".into(),
        },
        Command::AddRelation {
            id: RefId::new("r1"),
            kind: RelationKind::Serving,
            source: RefId::new("a2"),
            target: RefId::new("a1"),
        },
        Command::RenameElement {
            id: RefId::new("a1"),
            name: "A2".into(),
        },
    ]
}

#[test]
fn apply_supports_snapshot_undo() {
    let mut stack = vec![Model::new("undo")];
    for cmd in editor_commands() {
        stack.push(apply(stack.last().unwrap(), &cmd).unwrap());
    }
    let top = stack.last().unwrap();
    assert_eq!(top.element_count(), 2);
    assert_eq!(top.relation_count(), 1);
    let a1 = top.resolve_element(&RefId::new("a1")).unwrap();
    assert_eq!(top[a1].name, "A2");
    assert!(validate_model(top).success);

    // Undo = pop snapshots; redo = re-apply the same command data.
    while stack.len() > 1 {
        stack.pop();
    }
    assert_eq!(stack.last().unwrap().element_count(), 0);
}

#[test]
fn replaying_commands_is_deterministic() {
    fn run(cmds: &[Command]) -> String {
        let mut m = Model::new("replay");
        for c in cmds {
            m = apply(&m, c).unwrap();
        }
        model_to_yaml(&m)
    }
    let cmds = editor_commands();
    assert_eq!(run(&cmds), run(&cmds));
}

#[test]
fn commands_round_trip_as_data() {
    let cmd = Command::AddRelation {
        id: RefId::new("r9"),
        kind: RelationKind::Serving,
        source: RefId::new("e2"),
        target: RefId::new("e1"),
    };
    let s = serde_yaml::to_string(&cmd).unwrap();
    assert!(s.contains("op: add_relation"));
    assert_eq!(serde_yaml::from_str::<Command>(&s).unwrap(), cmd);
}

#[test]
fn apply_reports_precise_errors() {
    let m = base_model();
    assert_eq!(
        apply(&m, &Command::AddElement {
            id: RefId::new("e1"),
            kind: ElementKind::BusinessActor,
            name: "dup".into(),
        }),
        Err(ApplyError::DuplicateRef(RefId::new("e1")))
    );
    assert_eq!(
        apply(&m, &Command::DeleteElement { id: RefId::new("ghost") }),
        Err(ApplyError::UnknownElement(RefId::new("ghost")))
    );
    assert_eq!(
        apply(&m, &Command::DeleteRelation { id: RefId::new("ghost") }),
        Err(ApplyError::UnknownRelation(RefId::new("ghost")))
    );
    assert_eq!(
        apply(&m, &Command::AddRelation {
            id: RefId::new("r9"),
            kind: RelationKind::Realization,
            source: RefId::new("e1"),
            target: RefId::new("e3"),
        }),
        Err(ApplyError::InvalidConnection(ConnError::Disallowed {
            source: ElementKind::BusinessActor,
            kind: RelationKind::Realization,
            target: ElementKind::Node,
        }))
    );
    // Nothing entered the model despite the failed commands.
    assert_eq!(m.element_count(), 3);
    assert_eq!(m.relation_count(), 1);
}
