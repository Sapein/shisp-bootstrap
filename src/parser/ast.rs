use crate::lexer::tokens::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum ASTError {
    UnableToFindParent,
}

#[derive(Debug, PartialEq)]
pub struct AST {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NodeType {
    Str(String),
    Atom(String),
    Comment(String),
    Number(u128),
    Boolean(bool),

    Quote, Quasiquote,
    Unquote, UnquoteSplice,

    Expr,
    CloseExpr,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub row: (usize, usize),
    pub col: (usize, usize),

    pub node_type: NodeType,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Edge {
    parent: usize,
    child: usize,
}

impl AST {
    pub fn new() -> AST {
        AST {
            nodes: vec![],
            edges: vec![],
        }
    }

    pub fn get_children(&self, node: &Node) -> Vec<&Node> {
        if self.nodes.contains(node) {
            let index = self.nodes
                .iter()
                .position(|n| &n == &node)
                .unwrap();
            self.get_children_index(index)
                .iter()
                .map(|i| self.nodes.get(*i).unwrap())
                .collect::<Vec<&Node>>()
        } else {
            vec![]
        }
    }

    pub fn get_children_index(&self, node_index: usize) -> Vec<usize> {
        match self.nodes.get(node_index) {
            Some(_) => {
                self.edges
                    .iter()
                    .filter(|e| e.parent == node_index)
                    .map(|e| e.child)
                    .collect()
            }
            None => vec![]
        }
    }

    pub fn get_parent(&self, node: &Node) -> Option<&Node> {
        if self.nodes.contains(node) {
            let index = self.nodes.iter().position(|n| &n == &node).unwrap();
            let index = self.get_parent_index(index);
            self.nodes.get(index?)
        } else {
            None
        }
    }

    pub fn get_parent_index(&self, node_index: usize) -> Option<usize> {
        let node = self.nodes.get(node_index)?;
        let parents: usize = self.edges
            .iter()
            .filter(|e| e.child == node_index)
            .collect::<Vec<&Edge>>()
            .get(0).unwrap()
            .parent;
        Some(parents)
    }

    pub fn deparent_index(self, node_index: usize) -> AST {
        match self.nodes.get(node_index) {
            Some(_) => {
                let edges = self.edges
                    .into_iter()
                    .filter(|e| e.parent != node_index && e.child != node_index)
                    .collect();
                AST {
                    edges,
                    ..self
                }

            }
            None => self
        }
    }

    pub fn deparent(self, node: &Node) -> AST {
        if self.nodes.contains(node) {
            let index = self.nodes.iter().position(|n| &n == &node).unwrap();
            self.deparent_index(index)
        } else {
            self
        }
    }

    pub fn add_base_node(self, node: Node) -> AST {
        let mut nodes = self.nodes;
        nodes.push(node);
        AST {
            nodes,
            ..self
        }
    }

    pub fn add_child_index(self, parent_index: usize, child: Node) -> Result<AST, ASTError> {
        match self.nodes.get(parent_index) {
            Some(_) => {
                let mut nodes = self.nodes;
                let mut edges = self.edges;

                nodes.push(child);
                let child_index = nodes.iter().len() - 1;
                edges.push(Edge {
                    parent: parent_index,
                    child: child_index,
                });

                Ok(AST {
                    nodes: nodes,
                    edges: edges,
                })
            },
            None => Err(ASTError::UnableToFindParent),
        }
    }

    pub fn add_child(self, parent: &Node, child: Node) -> Result<AST, ASTError> {
        if self.nodes.contains(parent) {
            let index = self.nodes.iter().position(|n| &n == &parent).unwrap();
            self.add_child_index(index, child)
        } else {
            Err(ASTError::UnableToFindParent)
        }
    }

    pub fn remove_node_index(self, node_index: usize) -> AST {
        match self.nodes.get(node_index) {
            Some(_) => {
                let mut nodes = self.nodes;
                let edges = self.edges
                    .into_iter()
                    .filter(|e| e.parent != node_index && e.child != node_index)
                    .map(|e| { if e.parent > node_index { Edge { parent: e.parent - 1, child: e.child }} else { e }})
                    .map(|e| { if e.child > node_index { Edge { parent: e.parent, child: e.child - 1 }} else { e }})
                    .collect();
                nodes.remove(node_index);
                AST {
                    edges,
                    nodes: nodes,
                }

            }
            None => self
        }
    }

    pub fn remove_node(self, node: &Node) -> AST {
        if self.nodes.contains(node) {
            let index = self.nodes.iter().position(|n| n == node).unwrap();
            self.remove_node_index(index)
        } else {
            self
        }
    }
}

impl Node {
    pub fn from_token(token: Token) -> Option<Node> {
        let (row, col, token_type) = token.into_raw_parts();

        let node_type = match token_type {
            TokenType::Atom(v) => Some(NodeType::Atom(v)),
            TokenType::Number(v) => Some(NodeType::Number(v)),
            TokenType::Str(v) => Some(NodeType::Str(v)),
            TokenType::Comment(v) => Some(NodeType::Comment(v)),
            TokenType::True(_) => Some(NodeType::Boolean(true)),
            TokenType::False(_) => Some(NodeType::Boolean(false)),
            TokenType::UnquoteSplice => Some(NodeType::UnquoteSplice),
            TokenType::Comma => Some(NodeType::Unquote),
            TokenType::Backquote => Some(NodeType::Quasiquote),
            TokenType::SingleQuote => Some(NodeType::Quote),

            TokenType::At => Some(NodeType::Atom("@".to_string())),
            TokenType::LeftParen => Some(NodeType::Expr),
            TokenType::RightParen => Some(NodeType::CloseExpr),

            TokenType::Whitespace(_) | TokenType::Newline | TokenType::Tab => None,
            TokenType::EOF => None
        }?;

        Some(Node {
            row,
            col,
            node_type
        })
    }

    fn new(node_type: NodeType) -> Node {
        Node {
            row: (0,0),
            col: (0,0),
            node_type
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    mod ast{
        use super::*;
        #[test]
        fn test_new() {
            assert_eq!(AST::new(), AST {
                nodes: Vec::new(),
                edges: Vec::new()
            })
        }

        fn make_node() -> Node {
            Node {
                row: (0,0),
                col: (0,0),
                node_type: NodeType::Expr,
            }
        }

        #[test]
        fn add_child_index() {
            let ast = AST::new()
                .add_base_node(make_node())
                .add_child_index(0, make_node()).unwrap();
            assert_eq!(ast.nodes, vec![make_node(), make_node()]);
            assert_eq!(ast.edges, vec![Edge { parent: 0, child: 1 } ]);
        }

        #[test]
        fn remove_node_index() {
            let ast = AST::new()
                .add_base_node(make_node())
                .remove_node_index(0);

            let ast_parent = AST::new()
                .add_base_node(make_node())
                .add_child_index(0, make_node())
                .unwrap()
                .remove_node_index(0);

            let ast_child = AST::new()
                .add_base_node(make_node())
                .add_child_index(0, make_node())
                .unwrap()
                .remove_node_index(1);

            assert_eq!(ast.nodes, vec![]);
            assert_eq!(ast.edges, vec![]);

            assert_eq!(ast_parent.nodes, vec![make_node()]);
            assert_eq!(ast_parent.edges, vec![]);

            assert_eq!(ast_child.nodes, vec![make_node()]);
            assert_eq!(ast_child.edges, vec![]);
        }

        #[test]
        fn remove_node() {
            let ast = AST::new()
                .add_base_node(make_node())
                .remove_node(&make_node());

            let ast_parent = AST::new()
                .add_base_node(make_node())
                .add_child_index(0, make_node())
                .unwrap()
                .remove_node(&make_node());

            assert_eq!(ast.nodes, vec![]);
            assert_eq!(ast.edges, vec![]);

            assert_eq!(ast_parent.nodes, vec![make_node()]);
            assert_eq!(ast_parent.edges, vec![]);
        }

        #[test]
        fn add_child() {
            let ast = AST::new()
                .add_base_node(make_node())
                .add_child(&make_node(), make_node()).unwrap();
            assert_eq!(ast.nodes, vec![make_node(), make_node()]);
            assert_eq!(ast.edges, vec![Edge { parent: 0, child: 1 } ]);
        }

        #[test]
        fn deparent_index() {
            let ast = AST::new()
                .add_base_node(make_node())
                .add_child_index(0, make_node())
                .unwrap()
                .deparent_index(1);

            assert_eq!(ast.edges, vec![]);
        }

        #[test]
        fn test_add_base_node() {
            let node = Node {
                row: (0,0),
                col: (0,0),
                node_type: NodeType::Atom("a".to_string()),
            };

            let ast = AST::new().add_base_node(Node {
                row: (0,0),
                col: (0,0),
                node_type: NodeType::Atom("a".to_string()),
            });

            assert_eq!(ast.nodes[0],node);
        }
    }

    mod node {
        use super::*;

        #[test]
        pub fn from_token() {
            let raw_tokens = ["\"AAA\"", "123","@", ",@", ",", "`", "'", "(", ")", " ", "\t", "\n", "#t", "#f", ";test"];
            let nodes = raw_tokens.map(|rs| Token::new((0,0), (0,0), rs.to_string())).map(|t| Node::from_token(t));
            let proper_nodes = [
                Some(Node::new(NodeType::Str("\"AAA\"".to_string()))),
                Some(Node::new(NodeType::Number(123))),
                Some(Node::new(NodeType::Atom("@".to_string()))),
                Some(Node::new(NodeType::UnquoteSplice)),
                Some(Node::new(NodeType::Unquote)),
                Some(Node::new(NodeType::Quasiquote)),
                Some(Node::new(NodeType::Quote)),
                Some(Node::new(NodeType::Expr)),
                Some(Node::new(NodeType::CloseExpr)),
                None, None, None,
                Some(Node::new(NodeType::Boolean(true))),
                Some(Node::new(NodeType::Boolean(false))),
                Some(Node::new(NodeType::Comment(";test".to_string()))),
            ];

            let mut i = 0;
            for node in nodes {
                println!("{:?}", node);
                println!("{:?}", proper_nodes[i]);
                assert_eq!(node, proper_nodes[i]);
                i += 1;
            }
        }
    }
}
