use super::{ErrMessageBuilder, ErrMessageBuilderNode};

#[test]
fn new_initializes_correctly() {
    let err_message_builder = ErrMessageBuilder::new();
    assert_eq!(err_message_builder.nodes.len(),0);
}

#[test]
fn build_message_works() {
    let err_message_builder = ErrMessageBuilder {
        nodes: vec![
            ErrMessageBuilderNode::OneOfMany { 
                field: "profile".to_string(), specific_id: "My PS3 Controller".to_string()},
            ErrMessageBuilderNode::OneOfMany { 
                field: "layer".to_string(), specific_id: "first-layer-step-1".to_string()},
            ErrMessageBuilderNode::Single {
                field: "left_trigger".to_string() },
            ErrMessageBuilderNode::Single {
                field: "on_double_click".to_string() },
            ErrMessageBuilderNode::Single {
                field: "move_to_or_visit_layer".to_string() },
        ],
    };
    assert_eq!(
        err_message_builder.build_message(
            "No layer found having the id \"seconda-layer-step-1\"".to_string()),
        format!("{}\n{}\n{}\n{}\n{}\n{}\n{}",
        "Error in",
        "> profile: \"My PS3 Controller\"",
        "   > layer: \"first-layer-step-1\"",
        "      > left_trigger",
        "         > on_double_click",
        "            > move_to_or_visit_layer",
        "No layer found having the id \"seconda-layer-step-1\"",));
}

#[test]
fn branch_works() {
    let err_message_builder = ErrMessageBuilder {
        nodes: vec![
            ErrMessageBuilderNode::OneOfMany { 
                field: "profile".to_string(), specific_id: "My PS3 Controller".to_string()},
            ErrMessageBuilderNode::OneOfMany { 
                field: "layer".to_string(), specific_id: "first-layer-step-1".to_string()},
            ErrMessageBuilderNode::Single {
                field: "left_trigger".to_string() },
            ErrMessageBuilderNode::Single {
                field: "on_double_click".to_string() },
            ErrMessageBuilderNode::Single {
                field: "move_to_or_visit_layer".to_string() },
        ],
    };

    assert_eq!(err_message_builder.nodes.len(),5);

    let new_err_message_builder = err_message_builder.branch(
            ErrMessageBuilderNode::OneOfMany { 
                field: "hello".to_string(), specific_id: "new-item".to_string()});

    assert_eq!(err_message_builder.nodes.len(),5);

    assert_eq!(new_err_message_builder.nodes.len(),6);
    assert_eq!(new_err_message_builder.nodes[5],
            ErrMessageBuilderNode::OneOfMany { 
                field: "hello".to_string(), specific_id: "new-item".to_string()});
}
