use crate::lexer::tokens::{Token, TokenType};

#[derive(Debug, PartialEq)]
struct AST {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

#[derive(Debug, PartialEq, Eq)]
enum NodeType {
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

#[derive(Debug, PartialEq, Eq)]
struct Node {
    row: (usize, usize),
    col: (usize, usize),

    // index: usize,
    node_type: NodeType,
}

#[derive(Debug,PartialEq, Eq)]
struct Edge {
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

    pub fn add_base_node(self, node: Node) -> AST {
        let mut nodes = self.nodes;
        nodes.push(node);
        AST {
            nodes,
            ..self
        }
    }

    pub fn add_child_index(self, parent_index: usize, child: Node) -> Result<AST, String> {
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
            None => Err("Parent not found!".to_string()),
        }
    }

    pub fn add_child(self, parent: &Node, child: Node) -> Result<AST, String> {
        if self.nodes.contains(parent) {
            let index = self.nodes.iter().position(|n| &n == &parent).unwrap();
            self.add_child_index(index, child)
        } else {
            Err("Parent not in AST!".to_string())
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
            TokenType::EOF => None, //TODO: Maybe change this?
        }?;

        Some(Node {
            row,
            col,

            node_type: node_type
        })
    }

    fn new(node_type: NodeType) -> Node{
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
        fn add_child() {
            let ast = AST::new()
                .add_base_node(make_node())
                .add_child(&make_node(), make_node()).unwrap();
            assert_eq!(ast.nodes, vec![make_node(), make_node()]);
            assert_eq!(ast.edges, vec![Edge { parent: 0, child: 1 } ]);
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
