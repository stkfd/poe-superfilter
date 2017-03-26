
use ast::{TransformedNode, CompileErr, Node};
use ast::transform::{Transform, TransformResult};
use scope::{ScopeData, ScopeValue};
use arena::TypedArena;
use std::cell::RefCell;
use std::rc::Rc;
use std::io::Write;
use std::cmp::PartialEq;

/// AST structure for a value set or other instruction statement
#[derive(Debug, Clone)]
pub struct SetValueStatement<'a> {
    pub name : String,
    pub values : Vec<&'a Node<'a>>
}

#[derive(Debug,Clone)]
pub struct PlainSetValueStatement {
    pub name: String,
    pub values: Vec<ScopeValue>
}

impl PartialEq for PlainSetValueStatement {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl <'a> Transform<'a> for SetValueStatement<'a> {
    fn transform(&'a self, parent_scope: Rc<RefCell<ScopeData<'a>>>, transformed_arena: &'a TypedArena<TransformedNode<'a>>)
        -> Result<Option<&'a TransformedNode<'a>>, CompileErr> {
        let mut transformed_values: Vec<ScopeValue> = vec![];
        for value in &self.values {
            if let Some(t_value) = try!(value.transform(parent_scope.clone(), transformed_arena)) {
                transformed_values.push(t_value.return_value());
            }
        }

        Ok(Some(transformed_arena.alloc(TransformedNode::SetValueStmt(
            PlainSetValueStatement {
                name: self.name.clone(),
                values: transformed_values
            }
        ))))
    }
}

impl TransformResult for PlainSetValueStatement {
    fn render(&self, buf: &mut Write) -> Result<(), CompileErr> {
        buf.write(self.name.as_ref())?;
        for val in &self.values {
            buf.write(" ".as_ref())?;
            val.render(buf)?;
        }
        buf.write("\n".as_ref())?;
        Ok(())
    }
}

/// AST structure for a condition statement
#[derive(Debug, Clone)]
pub struct ConditionStatement<'a> {
    pub name : String,
    pub condition : Condition<'a>
}

#[derive(Debug, Clone)]
pub struct PlainConditionStatement {
    pub name : String,
    pub condition: PlainCondition
}

impl PartialEq for PlainConditionStatement {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl <'a> Transform<'a> for ConditionStatement<'a> {
    fn transform(&'a self, parent_scope: Rc<RefCell<ScopeData<'a>>>, transformed_arena: &'a TypedArena<TransformedNode<'a>>)
        -> Result<Option<&'a TransformedNode<'a>>, CompileErr> {
        if let Some(t_value) = self.condition.value.transform(parent_scope.clone(), transformed_arena)? {
            return Ok(Some(transformed_arena.alloc(TransformedNode::ConditionStmt(
                PlainConditionStatement {
                    name: self.name.clone(),
                    condition: PlainCondition { value: t_value.return_value(), operator: self.condition.operator },
                }
            ))));
        }
        return Err(CompileErr::Unknown);
    }
}

impl TransformResult for PlainConditionStatement {
    fn render(&self, buf: &mut Write) -> Result<(), CompileErr> {
        buf.write(self.name.as_ref())?;
        buf.write(" ".as_ref())?;
        self.condition.operator.render(buf)?;
        buf.write(" ".as_ref())?;
        self.condition.value.render(buf)?;
        buf.write("\n".as_ref())?;
        Ok(())
    }
}

/// AST structure for a condition. This node is always embedded, so there is no
/// corresponding variant for it in the ast::Node enum
#[derive(Debug, Clone)]
pub struct Condition<'a> {
    pub value: &'a Node<'a>,
    pub operator: ComparisonOperator
}

#[derive(Debug, Clone)]
pub struct PlainCondition {
    pub value: ScopeValue,
    pub operator: ComparisonOperator
}

/// Comparison operator AST structure
#[derive(Debug, Clone, Copy)]
pub enum ComparisonOperator {
    Eql,
    Lt,
    Lte,
    Gt,
    Gte
}

impl TransformResult for ComparisonOperator {
    fn render(&self, buf: &mut Write) -> Result<(), CompileErr> {
        buf.write(match *self {
            ComparisonOperator::Eql => "=",
            ComparisonOperator::Gt => ">",
            ComparisonOperator::Gte => ">=",
            ComparisonOperator::Lt => "<",
            ComparisonOperator::Lte => "<=",
        }.as_ref())?;
        Ok(())
    }
}
