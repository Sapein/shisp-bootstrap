pub mod ast;

use crate::lexer::tokens::Token;
use self::ast::{AST, Node, NodeType};

pub fn parse_tokens(tokens: Vec<Token>) -> AST {
    let mut ast = self::AST::new();
    let mut root_stack: Vec<Node> = vec![];
    let nodes: Vec<Node> = tokens
        .into_iter()
        .map(|t| Node::from_token(t))
        .filter(|on| *on != None)
        .map(|n| n.unwrap())
        .collect();

    for node in nodes {
        match &node.node_type {
            NodeType::Expr => {
                match root_stack.pop() {
                    Some(root) => {
                        ast = ast.add_child(&root, node.clone()).unwrap();
                        root_stack.push(root);
                        root_stack.push(node);
                    },
                    None => {
                        root_stack.push(node.clone());
                        ast = ast.add_base_node(node);
                    }
                }
            }
            NodeType::CloseExpr => {
                root_stack.pop();
            }
            _ => match root_stack.pop() {
                Some(root) => {
                    ast = ast.add_child(&root, node).unwrap();
                    root_stack.push(root);
                },
                None => ast = ast.add_base_node(node)

            }
        }
    }
    ast
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tokens() {
        use crate::lexer;
        use std::collections::HashSet;

        let tokens = lexer::scan_string("(for the win)".to_string());
        let tokens_2 = lexer::scan_string("(for the win);a\n(shisp)".to_string());
        let ast = super::parse_tokens(tokens);
        let ast_2 = super::parse_tokens(tokens_2);

        assert_eq!(ast.get_base_node_indexes(), vec![0].into_iter().collect::<HashSet<usize>>());
        assert_eq!(ast.get_children_index(0), vec![1, 2, 3]);

        assert_eq!(ast_2.get_base_node_indexes(), vec![0, 4, 5].into_iter().collect::<HashSet<usize>>());
        assert_eq!(ast_2.get_children_index(0), vec![1, 2, 3]);
        assert_eq!(ast_2.get_children_index(4), vec![]);
        assert_eq!(ast_2.get_children_index(5), vec![6]);
    }
}
