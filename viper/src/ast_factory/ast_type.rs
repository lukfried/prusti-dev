// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ast_factory::structs::Type;
use ast_factory::AstFactory;
use jni::objects::JObject;
use jni_utils::JniUtils;
use viper_sys::wrappers::viper::silver::ast;

impl<'a> AstFactory<'a> {
    pub fn int_type(&self) -> Type<'a> {
        get_ast_object!(self, Type, ast::Int_object)
    }

    pub fn bool_type(&self) -> Type<'a> {
        get_ast_object!(self, Type, ast::Bool_object)
    }

    pub fn perm_type(&self) -> Type<'a> {
        get_ast_object!(self, Type, ast::Perm_object)
    }

    pub fn ref_type(&self) -> Type<'a> {
        get_ast_object!(self, Type, ast::Ref_object)
    }

    pub fn wand_type(&self) -> Type<'a> {
        get_ast_object!(self, Type, ast::Wand_object)
    }

    pub fn type_var(&self, name: &str) -> Type<'a> {
        let obj = self
            .jni
            .unwrap_result(ast::TypeVar::with(self.env).new(self.jni.new_string(name)));
        Type::new(obj)
    }

    pub fn domain_type(
        &self,
        domain_name: &str,
        partial_typ_vars_map: &[(Type,Type)],
        type_parameters: &[Type],
    ) -> Type<'a> {
        let obj = self.jni.unwrap_result(ast::DomainType::with(self.env).new(
            self.jni.new_string(domain_name),
            self.jni.new_map(&map_to_jobject_pairs!(partial_typ_vars_map)),
            self.jni.new_seq(&map_to_jobjects!(type_parameters)),
        ));
        Type::new(obj)
    }

    pub fn set_type(&self, element_type: Type) -> Type<'a> {
        let obj = self
            .jni
            .unwrap_result(ast::SetType::with(self.env).new(element_type.to_jobject()));
        Type::new(obj)
    }

    pub fn multiset_type(&self, element_type: Type) -> Type<'a> {
        let obj = self
            .jni
            .unwrap_result(ast::MultisetType::with(self.env).new(element_type.to_jobject()));
        Type::new(obj)
    }

    pub fn seq_type(&self, element_type: Type) -> Type<'a> {
        let obj = self
            .jni
            .unwrap_result(ast::SeqType::with(self.env).new(element_type.to_jobject()));
        Type::new(obj)
    }

    // This function is probably wrong. I was just trying things out.
    pub fn backend_type(&self) -> Type<'a> {
        let utils = JniUtils::new(self.env);
        let boogiename = JniUtils::new_string(&utils, "boogiename"); 
        let smtname = JniUtils::new_string(&utils, "smtname"); 
        let obj = self
            .jni
            .unwrap_result(ast::BackendType::with(self.env).new(boogiename,smtname));
        Type::new(obj)
    }

}
