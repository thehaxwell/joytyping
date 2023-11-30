#[cfg(test)]
mod tests;

#[derive(Debug,Clone,PartialEq)]
pub enum ErrMessageBuilderNode {
    Single{field: String},
    OneOfMany{field: String, specific_id: String},
}

#[derive(Debug,Clone,PartialEq)]
pub struct ErrMessageBuilder {
    nodes: Vec<ErrMessageBuilderNode>,
}

impl ErrMessageBuilder {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }

    pub fn branch(&self, new_node: ErrMessageBuilderNode) -> ErrMessageBuilder{
        let mut nodes = self.nodes.to_vec();
        nodes.push(new_node);
        ErrMessageBuilder {
            nodes,
        }
    }

    pub fn build_message(&self, src: String) -> String {
        let mut message = "Error in\n".to_string();
        for (index,node) in self.nodes.iter().enumerate() {
            if index > 0 {
                message.push_str("   ".repeat(index).as_str());
            }
            message.push_str("> ");
            match node {
                ErrMessageBuilderNode::Single { field } => message.push_str(field),
                ErrMessageBuilderNode::OneOfMany { field, specific_id }
                => message.push_str(&format!("{}: \"{}\"",field,specific_id)),
            };
            message.push('\n');
        }
        message.push_str(&src);
        message
    }
}
