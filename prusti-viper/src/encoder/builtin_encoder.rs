// © 2019, ETH Zurich
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prusti_common::vir;
use prusti_common::vir::{Expr, PermAmount};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum BuiltinMethodKind {
    HavocBool,
    HavocInt,
    HavocRef,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum BuiltinFunctionKind {
    /// type
    Unreachable(vir::Type),
    /// type
    Undefined(vir::Type),
    // todo builtin lookup function
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum BuiltinPredicateKind {
    BuiltinInt,
    BuiltinTerminated,
}

pub struct BuiltinEncoder {}

impl BuiltinEncoder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn encode_builtin_method_name(&self, method: BuiltinMethodKind) -> String {
        match method {
            BuiltinMethodKind::HavocBool => "builtin$havoc_bool".to_string(),
            BuiltinMethodKind::HavocInt => "builtin$havoc_int".to_string(),
            BuiltinMethodKind::HavocRef => "builtin$havoc_ref".to_string(),
        }
    }

    pub fn encode_builtin_method_def(&self, method: BuiltinMethodKind) -> vir::BodylessMethod {
        let return_type = match method {
            BuiltinMethodKind::HavocBool => vir::Type::Bool,
            BuiltinMethodKind::HavocInt => vir::Type::Int,
            BuiltinMethodKind::HavocRef => vir::Type::TypedRef("".to_string()),
        };
        vir::BodylessMethod {
            name: self.encode_builtin_method_name(method),
            formal_args: vec![],
            formal_returns: vec![vir::LocalVar::new("ret", return_type)],
        }
    }

    pub fn encode_builtin_function_name(&self, function: &BuiltinFunctionKind) -> String {
        match function {
            BuiltinFunctionKind::Unreachable(vir::Type::Int) => format!("builtin$unreach_int"),
            BuiltinFunctionKind::Unreachable(vir::Type::Bool) => format!("builtin$unreach_bool"),
            BuiltinFunctionKind::Unreachable(vir::Type::TypedRef(_)) => {
                format!("builtin$unreach_ref")
            }
            BuiltinFunctionKind::Unreachable(vir::Type::Domain(_)) => {
                format!("builtin$unreach_domain")
            }
            BuiltinFunctionKind::Undefined(vir::Type::Int) => format!("builtin$undef_int"),
            BuiltinFunctionKind::Undefined(vir::Type::Bool) => format!("builtin$undef_bool"),
            BuiltinFunctionKind::Undefined(vir::Type::TypedRef(_)) => format!("builtin$undef_ref"),
            BuiltinFunctionKind::Undefined(vir::Type::Domain(_)) => format!("builtin$undef_doman"),
        }
    }

    pub fn encode_builtin_function_def(&self, function: BuiltinFunctionKind) -> vir::Function {
        let fn_name = self.encode_builtin_function_name(&function);
        match function {
            BuiltinFunctionKind::Unreachable(typ) => vir::Function {
                name: fn_name,
                formal_args: vec![],
                return_type: typ,
                // Precondition is false, because we want to be sure that this function is never used
                pres: vec![false.into()],
                posts: vec![],
                body: None,
            },
            BuiltinFunctionKind::Undefined(typ) => vir::Function {
                name: fn_name,
                formal_args: vec![],
                return_type: typ,
                pres: vec![],
                posts: vec![],
                body: None,
            },
        }
    }

    pub fn encode_builtin_predicate_name(&self, predicate: BuiltinPredicateKind) -> String {
        match predicate {
            BuiltinPredicateKind::BuiltinInt => "builtin$builtin_int".to_string(),
            BuiltinPredicateKind::BuiltinTerminated => {"builtin$terminated"}.to_string(),
        }
    }

    pub fn encode_builtin_predicate_def(&self, predicate: BuiltinPredicateKind) -> vir::Predicate {
        let predicate_name = self.encode_builtin_predicate_name(predicate);
        match predicate {
            BuiltinPredicateKind::BuiltinInt => {
                let field = vir::Field::new("val_int", vir::Type::Int);
                vir::Predicate::new_builtin_value(
                    predicate_name,
                    vir::Type::Int,
                    vir::Field::new("val_int", vir::Type::Int),
                )
            }
            BuiltinPredicateKind::BuiltinTerminated => {
                vir::Predicate::new_builtin_value(
                    predicate_name,
                    vir::Type::Bool,
                    vir::Field::new("val_bool", vir::Type::Bool),
                )
            }
        }
    }
}
