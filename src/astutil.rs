use ast::*;
use span::{Node, Span};

#[cfg_attr(test, derive(Debug, PartialEq, Clone))]
pub enum Operation {
    Member(Node<MemberOperator>, Node<Identifier>),
    Unary(Node<UnaryOperator>),
    Binary(Node<BinaryOperator>, Node<Expression>),
    Call(Vec<Node<Expression>>),
}

fn apply_op(a: Node<Expression>, op: Node<Operation>) -> Node<Expression> {
    let span = Span::span(a.span.start, op.span.end);
    let expr = match op.node {
        Operation::Member(op, id) => Expression::Member(Node::new(
            MemberExpression {
                operator: op,
                expression: Box::new(a),
                identifier: id,
            },
            span,
        )),
        Operation::Unary(op) => Expression::UnaryOperator(Node::new(
            UnaryOperatorExpression {
                operator: op,
                operand: Box::new(a),
            },
            span,
        )),
        Operation::Binary(op, b) => Expression::BinaryOperator(Node::new(
            BinaryOperatorExpression {
                operator: op,
                lhs: Box::new(a),
                rhs: Box::new(b),
            },
            span,
        )),
        Operation::Call(args) => Expression::Call(Node::new(
            CallExpression {
                callee: Box::new(a),
                arguments: args,
            },
            span,
        )),
    };

    Node::new(expr, span)
}

pub fn apply_ops(ops: Vec<Node<Operation>>, expr: Node<Expression>) -> Node<Expression> {
    ops.into_iter().fold(expr, apply_op)
}

pub fn concat<T>(mut a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.extend(b);
    a
}

pub fn infix(node: Node<()>, op: BinaryOperator, lhs: Node<Expression>, rhs: Node<Expression>) -> Node<Expression> {
    let span = Span::span(lhs.span.start, rhs.span.end);
    Node::new(
        Expression::BinaryOperator(Node::new(
            BinaryOperatorExpression {
                operator: Node::new(op, node.span),
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
            span,
        )),
        span,
    )
}

pub fn with_ext(mut d: Node<Declarator>, e: Option<Vec<Node<Extension>>>) -> Node<Declarator> {
    if let Some(e) = e {
        d.node.extensions.extend(e);
    }
    d
}
