use ast::*;
use span::Span;

pub trait Visit<'ast> {
    fn visit_identifier(&mut self, identifier: &'ast Identifier, _span: &'ast Span) {
        visit_identifier(self, identifier, _span)
    }

    fn visit_constant(&mut self, constant: &'ast Constant, _span: &'ast Span) {
        visit_constant(self, constant, _span)
    }

    fn visit_integer(&mut self, integer: &'ast Integer, _span: &'ast Span) {
        visit_integer(self, integer, _span)
    }

    fn visit_integer_base(&mut self, integer_base: &'ast IntegerBase, _span: &'ast Span) {
        visit_integer_base(self, integer_base, _span)
    }

    fn visit_integer_suffix(&mut self, integer_suffix: &'ast IntegerSuffix, _span: &'ast Span) {
        visit_integer_suffix(self, integer_suffix, _span)
    }

    fn visit_integer_size(&mut self, integer_size: &'ast IntegerSize, _span: &'ast Span) {
        visit_integer_size(self, integer_size, _span)
    }

    fn visit_float(&mut self, float: &'ast Float, _span: &'ast Span) {
        visit_float(self, float, _span)
    }

    fn visit_float_base(&mut self, float_base: &'ast FloatBase, _span: &'ast Span) {
        visit_float_base(self, float_base, _span)
    }

    fn visit_float_suffix(&mut self, float_suffix: &'ast FloatSuffix, _span: &'ast Span) {
        visit_float_suffix(self, float_suffix, _span)
    }

    fn visit_float_format(&mut self, float_format: &'ast FloatFormat, _span: &'ast Span) {
        visit_float_format(self, float_format, _span)
    }

    fn visit_string_literal(&mut self, string_literal: &'ast StringLiteral, _span: &'ast Span) {
        visit_string_literal(self, string_literal, _span)
    }

    fn visit_expression(&mut self, expression: &'ast Expression, _span: &'ast Span) {
        visit_expression(self, expression, _span)
    }

    fn visit_member_operator(&mut self, member_operator: &'ast MemberOperator, _span: &'ast Span) {
        visit_member_operator(self, member_operator, _span)
    }

    fn visit_generic_selection(&mut self, generic_selection: &'ast GenericSelection, _span: &'ast Span) {
        visit_generic_selection(self, generic_selection, _span)
    }

    fn visit_generic_association(&mut self, generic_association: &'ast GenericAssociation, _span: &'ast Span) {
        visit_generic_association(self, generic_association, _span)
    }

    fn visit_generic_association_type(&mut self, generic_association_type: &'ast GenericAssociationType, _span: &'ast Span) {
        visit_generic_association_type(self, generic_association_type, _span)
    }

    fn visit_member_expression(&mut self, member_expression: &'ast MemberExpression, _span: &'ast Span) {
        visit_member_expression(self, member_expression, _span)
    }

    fn visit_call_expression(&mut self, call_expression: &'ast CallExpression, _span: &'ast Span) {
        visit_call_expression(self, call_expression, _span)
    }

    fn visit_compound_literal(&mut self, compound_literal: &'ast CompoundLiteral, _span: &'ast Span) {
        visit_compound_literal(self, compound_literal, _span)
    }

    fn visit_unary_operator(&mut self, unary_operator: &'ast UnaryOperator, _span: &'ast Span) {
        visit_unary_operator(self, unary_operator, _span)
    }

    fn visit_unary_operator_expression(&mut self, unary_operator_expression: &'ast UnaryOperatorExpression, _span: &'ast Span) {
        visit_unary_operator_expression(self, unary_operator_expression, _span)
    }

    fn visit_cast_expression(&mut self, cast_expression: &'ast CastExpression, _span: &'ast Span) {
        visit_cast_expression(self, cast_expression, _span)
    }

    fn visit_binary_operator(&mut self, binary_operator: &'ast BinaryOperator, _span: &'ast Span) {
        visit_binary_operator(self, binary_operator, _span)
    }

    fn visit_binary_operator_expression(&mut self, binary_operator_expression: &'ast BinaryOperatorExpression, _span: &'ast Span) {
        visit_binary_operator_expression(self, binary_operator_expression, _span)
    }

    fn visit_conditional_expression(&mut self, conditional_expression: &'ast ConditionalExpression, _span: &'ast Span) {
        visit_conditional_expression(self, conditional_expression, _span)
    }

    fn visit_va_arg_expression(&mut self, va_arg_expression: &'ast VaArgExpression, _span: &'ast Span) {
        visit_va_arg_expression(self, va_arg_expression, _span)
    }

    fn visit_offset_of_expression(&mut self, offset_of_expression: &'ast OffsetOfExpression, _span: &'ast Span) {
        visit_offset_of_expression(self, offset_of_expression, _span)
    }

    fn visit_offset_designator(&mut self, offset_designator: &'ast OffsetDesignator, _span: &'ast Span) {
        visit_offset_designator(self, offset_designator, _span)
    }

    fn visit_offset_member(&mut self, offset_member: &'ast OffsetMember, _span: &'ast Span) {
        visit_offset_member(self, offset_member, _span)
    }

    fn visit_declaration(&mut self, declaration: &'ast Declaration, _span: &'ast Span) {
        visit_declaration(self, declaration, _span)
    }

    fn visit_declaration_specifier(&mut self, declaration_specifier: &'ast DeclarationSpecifier, _span: &'ast Span) {
        visit_declaration_specifier(self, declaration_specifier, _span)
    }

    fn visit_init_declarator(&mut self, init_declarator: &'ast InitDeclarator, _span: &'ast Span) {
        visit_init_declarator(self, init_declarator, _span)
    }

    fn visit_storage_class_specifier(&mut self, storage_class_specifier: &'ast StorageClassSpecifier, _span: &'ast Span) {
        visit_storage_class_specifier(self, storage_class_specifier, _span)
    }

    fn visit_type_specifier(&mut self, type_specifier: &'ast TypeSpecifier, _span: &'ast Span) {
        visit_type_specifier(self, type_specifier, _span)
    }

    fn visit_ts18661_float_type(&mut self, ts18661_float_type: &'ast TS18661FloatType, _span: &'ast Span) {
        visit_ts18661_float_type(self, ts18661_float_type, _span)
    }

    fn visit_ts18661_float_format(&mut self, ts18661_float_format: &'ast TS18661FloatFormat, _span: &'ast Span) {
        visit_ts18661_float_format(self, ts18661_float_format, _span)
    }

    fn visit_struct_type(&mut self, struct_type: &'ast StructType, _span: &'ast Span) {
        visit_struct_type(self, struct_type, _span)
    }

    fn visit_struct_kind(&mut self, struct_kind: &'ast StructKind, _span: &'ast Span) {
        visit_struct_kind(self, struct_kind, _span)
    }

    fn visit_struct_declaration(&mut self, struct_declaration: &'ast StructDeclaration, _span: &'ast Span) {
        visit_struct_declaration(self, struct_declaration, _span)
    }

    fn visit_struct_field(&mut self, struct_field: &'ast StructField, _span: &'ast Span) {
        visit_struct_field(self, struct_field, _span)
    }

    fn visit_specifier_qualifier(&mut self, specifier_qualifier: &'ast SpecifierQualifier, _span: &'ast Span) {
        visit_specifier_qualifier(self, specifier_qualifier, _span)
    }

    fn visit_struct_declarator(&mut self, struct_declarator: &'ast StructDeclarator, _span: &'ast Span) {
        visit_struct_declarator(self, struct_declarator, _span)
    }

    fn visit_enum_type(&mut self, enum_type: &'ast EnumType, _span: &'ast Span) {
        visit_enum_type(self, enum_type, _span)
    }

    fn visit_enumerator(&mut self, enumerator: &'ast Enumerator, _span: &'ast Span) {
        visit_enumerator(self, enumerator, _span)
    }

    fn visit_type_qualifier(&mut self, type_qualifier: &'ast TypeQualifier, _span: &'ast Span) {
        visit_type_qualifier(self, type_qualifier, _span)
    }

    fn visit_function_specifier(&mut self, function_specifier: &'ast FunctionSpecifier, _span: &'ast Span) {
        visit_function_specifier(self, function_specifier, _span)
    }

    fn visit_alignment_specifier(&mut self, alignment_specifier: &'ast AlignmentSpecifier, _span: &'ast Span) {
        visit_alignment_specifier(self, alignment_specifier, _span)
    }

    fn visit_declarator(&mut self, declarator: &'ast Declarator, _span: &'ast Span) {
        visit_declarator(self, declarator, _span)
    }

    fn visit_declarator_kind(&mut self, declarator_kind: &'ast DeclaratorKind, _span: &'ast Span) {
        visit_declarator_kind(self, declarator_kind, _span)
    }

    fn visit_derived_declarator(&mut self, derived_declarator: &'ast DerivedDeclarator, _span: &'ast Span) {
        visit_derived_declarator(self, derived_declarator, _span)
    }

    fn visit_array_declarator(&mut self, array_declarator: &'ast ArrayDeclarator, _span: &'ast Span) {
        visit_array_declarator(self, array_declarator, _span)
    }

    fn visit_function_declarator(&mut self, function_declarator: &'ast FunctionDeclarator, _span: &'ast Span) {
        visit_function_declarator(self, function_declarator, _span)
    }

    fn visit_pointer_qualifier(&mut self, pointer_qualifier: &'ast PointerQualifier, _span: &'ast Span) {
        visit_pointer_qualifier(self, pointer_qualifier, _span)
    }

    fn visit_array_size(&mut self, array_size: &'ast ArraySize, _span: &'ast Span) {
        visit_array_size(self, array_size, _span)
    }

    fn visit_parameter_declaration(&mut self, parameter_declaration: &'ast ParameterDeclaration, _span: &'ast Span) {
        visit_parameter_declaration(self, parameter_declaration, _span)
    }

    fn visit_ellipsis(&mut self, ellipsis: &'ast Ellipsis, _span: &'ast Span) {
        visit_ellipsis(self, ellipsis, _span)
    }

    fn visit_type_name(&mut self, type_name: &'ast TypeName, _span: &'ast Span) {
        visit_type_name(self, type_name, _span)
    }

    fn visit_initializer(&mut self, initializer: &'ast Initializer, _span: &'ast Span) {
        visit_initializer(self, initializer, _span)
    }

    fn visit_initializer_list_item(&mut self, initializer_list_item: &'ast InitializerListItem, _span: &'ast Span) {
        visit_initializer_list_item(self, initializer_list_item, _span)
    }

    fn visit_designator(&mut self, designator: &'ast Designator, _span: &'ast Span) {
        visit_designator(self, designator, _span)
    }

    fn visit_range_designator(&mut self, range_designator: &'ast RangeDesignator, _span: &'ast Span) {
        visit_range_designator(self, range_designator, _span)
    }

    fn visit_static_assert(&mut self, static_assert: &'ast StaticAssert, _span: &'ast Span) {
        visit_static_assert(self, static_assert, _span)
    }

    fn visit_statement(&mut self, statement: &'ast Statement, _span: &'ast Span) {
        visit_statement(self, statement, _span)
    }

    fn visit_labeled_statement(&mut self, labeled_statement: &'ast LabeledStatement, _span: &'ast Span) {
        visit_labeled_statement(self, labeled_statement, _span)
    }

    fn visit_if_statement(&mut self, if_statement: &'ast IfStatement, _span: &'ast Span) {
        visit_if_statement(self, if_statement, _span)
    }

    fn visit_switch_statement(&mut self, switch_statement: &'ast SwitchStatement, _span: &'ast Span) {
        visit_switch_statement(self, switch_statement, _span)
    }

    fn visit_while_statement(&mut self, while_statement: &'ast WhileStatement, _span: &'ast Span) {
        visit_while_statement(self, while_statement, _span)
    }

    fn visit_do_while_statement(&mut self, do_while_statement: &'ast DoWhileStatement, _span: &'ast Span) {
        visit_do_while_statement(self, do_while_statement, _span)
    }

    fn visit_for_statement(&mut self, for_statement: &'ast ForStatement, _span: &'ast Span) {
        visit_for_statement(self, for_statement, _span)
    }

    fn visit_label(&mut self, label: &'ast Label, _span: &'ast Span) {
        visit_label(self, label, _span)
    }

    fn visit_for_initializer(&mut self, for_initializer: &'ast ForInitializer, _span: &'ast Span) {
        visit_for_initializer(self, for_initializer, _span)
    }

    fn visit_block_item(&mut self, block_item: &'ast BlockItem, _span: &'ast Span) {
        visit_block_item(self, block_item, _span)
    }

    fn visit_translation_unit(&mut self, translation_unit: &'ast TranslationUnit) {
        visit_translation_unit(self, translation_unit)
    }

    fn visit_external_declaration(&mut self, external_declaration: &'ast ExternalDeclaration, _span: &'ast Span) {
        visit_external_declaration(self, external_declaration, _span)
    }

    fn visit_function_definition(&mut self, function_definition: &'ast FunctionDefinition, _span: &'ast Span) {
        visit_function_definition(self, function_definition, _span)
    }

    fn visit_extension(&mut self, extension: &'ast Extension, _span: &'ast Span) {
        visit_extension(self, extension, _span)
    }

    fn visit_attribute(&mut self, attribute: &'ast Attribute, _span: &'ast Span) {
        visit_attribute(self, attribute, _span)
    }

    fn visit_asm_statement(&mut self, asm_statement: &'ast AsmStatement, _span: &'ast Span) {
        visit_asm_statement(self, asm_statement, _span)
    }

    fn visit_gnu_extended_asm_statement(&mut self, gnu_extended_asm_statement: &'ast GnuExtendedAsmStatement, _span: &'ast Span) {
        visit_gnu_extended_asm_statement(self, gnu_extended_asm_statement, _span)
    }

    fn visit_gnu_asm_operand(&mut self, gnu_asm_operand: &'ast GnuAsmOperand, _span: &'ast Span) {
        visit_gnu_asm_operand(self, gnu_asm_operand, _span)
    }

    fn visit_type_of(&mut self, type_of: &'ast TypeOf, _span: &'ast Span) {
        visit_type_of(self, type_of, _span)
    }
}

pub fn visit_identifier<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _identifier: &'ast Identifier, _span: &'ast Span) {
}

pub fn visit_constant<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _constant: &'ast Constant, _span: &'ast Span) {
    match _constant {
        Constant::Integer(i) => _visitor.visit_integer(i, _span),
        Constant::Float(f) => _visitor.visit_float(f, _span),
        Constant::Character(_) => {}
    }
}

pub fn visit_integer<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _integer: &'ast Integer, _span: &'ast Span) {
    _visitor.visit_integer_base(&_integer.base, _span);
    _visitor.visit_integer_suffix(&_integer.suffix, _span);
}

pub fn visit_integer_base<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _integer_base: &'ast IntegerBase, _span: &'ast Span) {
}

pub fn visit_integer_suffix<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _integer_suffix: &'ast IntegerSuffix, _span: &'ast Span) {
    _visitor.visit_integer_size(&_integer_suffix.size, _span);
}

pub fn visit_integer_size<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _integer_size: &'ast IntegerSize, _span: &'ast Span) {
}

pub fn visit_float<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _float: &'ast Float, _span: &'ast Span) {
    _visitor.visit_float_base(&_float.base, _span);
    _visitor.visit_float_suffix(&_float.suffix, _span);
}

pub fn visit_float_base<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _float_base: &'ast FloatBase, _span: &'ast Span) {
}

pub fn visit_float_suffix<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _float_suffix: &'ast FloatSuffix, _span: &'ast Span) {
    _visitor.visit_float_format(&_float_suffix.format, _span);
}

pub fn visit_float_format<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _float_format: &'ast FloatFormat, _span: &'ast Span) {
    match _float_format {
        FloatFormat::TS18661Format(f) => _visitor.visit_ts18661_float_type(f, _span),
        _ => {}
    }
}

pub fn visit_string_literal<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _string_literal: &'ast StringLiteral, _span: &'ast Span) {
}

pub fn visit_expression<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _expression: &'ast Expression, _span: &'ast Span) {
    match _expression {
        Expression::Identifier(i) => _visitor.visit_identifier(&i.node, &i.span),
        Expression::Constant(c) => _visitor.visit_constant(&c.node, &c.span),
        Expression::StringLiteral(s) => _visitor.visit_string_literal(&s.node, &s.span),
        Expression::GenericSelection(g) => _visitor.visit_generic_selection(&g.node, &g.span),
        Expression::Member(m) => _visitor.visit_member_expression(&m.node, &m.span),
        Expression::Call(c) => _visitor.visit_call_expression(&c.node, &c.span),
        Expression::CompoundLiteral(c) => _visitor.visit_compound_literal(&c.node, &c.span),
        Expression::SizeOf(s) => _visitor.visit_type_name(&s.node, &s.span),
        Expression::AlignOf(a) => _visitor.visit_type_name(&a.node, &a.span),
        Expression::UnaryOperator(u) => _visitor.visit_unary_operator_expression(&u.node, &u.span),
        Expression::Cast(c) => _visitor.visit_cast_expression(&c.node, &c.span),
        Expression::BinaryOperator(b) => _visitor.visit_binary_operator_expression(&b.node, &b.span),
        Expression::Conditional(c) => _visitor.visit_conditional_expression(&c.node, &c.span),
        Expression::Comma(comma) => {
            for c in comma.iter() {
                _visitor.visit_expression(&c.node, &c.span);
            }
        },
        Expression::OffsetOf(o) => _visitor.visit_offset_of_expression(&o.node, &o.span),
        Expression::VaArg(v) => _visitor.visit_va_arg_expression(&v.node, &v.span),
        Expression::Statement(s) => _visitor.visit_statement(&s.node, &s.span)
    }
}

pub fn visit_member_operator<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _member_operator: &'ast MemberOperator, _span: &'ast Span) {
}

pub fn visit_generic_selection<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _generic_selection: &'ast GenericSelection, _span: &'ast Span) {
    _visitor.visit_expression(&_generic_selection.expression.node, &_generic_selection.expression.span);
    for association in &_generic_selection.associations {
        _visitor.visit_generic_association(&association.node, &association.span);
    }
}

pub fn visit_generic_association<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _generic_association: &'ast GenericAssociation, _span: &'ast Span) {
    match _generic_association {
        GenericAssociation::Type(t) => _visitor.visit_generic_association_type(&t.node, &t.span),
        GenericAssociation::Default(d) => _visitor.visit_expression(&d.node, &d.span)
    }
}

pub fn visit_generic_association_type<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _generic_association_type: &'ast GenericAssociationType, _span: &'ast Span) {
    _visitor.visit_type_name(&_generic_association_type.type_name.node, &_generic_association_type.type_name.span);
    _visitor.visit_expression(&_generic_association_type.expression.node, &_generic_association_type.expression.span);
}

pub fn visit_member_expression<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _member_expression: &'ast MemberExpression, _span: &'ast Span) {
    _visitor.visit_member_operator(&_member_expression.operator.node, &_member_expression.operator.span);
    _visitor.visit_expression(&_member_expression.expression.node, &_member_expression.expression.span);
    _visitor.visit_identifier(&_member_expression.identifier.node, &_member_expression.identifier.span);
}

pub fn visit_call_expression<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _call_expression: &'ast CallExpression, _span: &'ast Span) {
    _visitor.visit_expression(&_call_expression.callee.node, &_call_expression.callee.span);
    for argument in &_call_expression.arguments {
        _visitor.visit_expression(&argument.node, &argument.span);
    }
}

pub fn visit_compound_literal<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _compound_literal: &'ast CompoundLiteral, _span: &'ast Span) {
    _visitor.visit_type_name(&_compound_literal.type_name.node, &_compound_literal.type_name.span);
    for initializer in &_compound_literal.initializer_list {
        _visitor.visit_initializer(&initializer.node, &initializer.span);
    }
}

pub fn visit_unary_operator<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _unary_operator: &'ast UnaryOperator, _span: &'ast Span) {
}

pub fn visit_unary_operator_expression<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _unary_operator_expression: &'ast UnaryOperatorExpression, _span: &'ast Span) {
    _visitor.visit_unary_operator(&_unary_operator_expression.operator.node, &_unary_operator_expression.operator.span);
    _visitor.visit_expression(&_unary_operator_expression.operand.node, &_unary_operator_expression.operand.span);
}

pub fn visit_cast_expression<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _cast_expression: &'ast CastExpression, _span: &'ast Span) {
    _visitor.visit_type_name(&_cast_expression.type_name.node, &_cast_expression.type_name.span);
    _visitor.visit_expression(&_cast_expression.expression.node, &_cast_expression.expression.span);
}

pub fn visit_binary_operator<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _binary_operator: &'ast BinaryOperator, _span: &'ast Span) {
}

pub fn visit_binary_operator_expression<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _binary_operator_expression: &'ast BinaryOperatorExpression, _span: &'ast Span) {
    _visitor.visit_binary_operator(&_binary_operator_expression.operator.node, &_binary_operator_expression.operator.span);
    _visitor.visit_expression(&_binary_operator_expression.lhs.node, &_binary_operator_expression.lhs.span);
    _visitor.visit_expression(&_binary_operator_expression.rhs.node, &_binary_operator_expression.rhs.span);
}

pub fn visit_conditional_expression<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _conditional_expression: &'ast ConditionalExpression, _span: &'ast Span) {
    _visitor.visit_expression(&_conditional_expression.condition.node, &_conditional_expression.condition.span);
    _visitor.visit_expression(&_conditional_expression.then_expression.node, &_conditional_expression.then_expression.span);
    _visitor.visit_expression(&_conditional_expression.else_expression.node, &_conditional_expression.else_expression.span);
}

pub fn visit_va_arg_expression<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _va_arg_expression: &'ast VaArgExpression, _span: &'ast Span) {
    _visitor.visit_expression(&_va_arg_expression.va_list.node, &_va_arg_expression.va_list.span);
    _visitor.visit_type_name(&_va_arg_expression.type_name.node, &_va_arg_expression.type_name.span);
}

pub fn visit_offset_of_expression<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _offset_of_expression: &'ast OffsetOfExpression, _span: &'ast Span) {
    _visitor.visit_type_name(&_offset_of_expression.type_name.node, &_offset_of_expression.type_name.span);
    _visitor.visit_offset_designator(&_offset_of_expression.designator.node, &_offset_of_expression.designator.span);
}

pub fn visit_offset_designator<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _offset_designator: &'ast OffsetDesignator, _span: &'ast Span) {
    _visitor.visit_identifier(&_offset_designator.base.node, &_offset_designator.base.span);
    for member in &_offset_designator.members {
        _visitor.visit_offset_member(&member.node, &member.span);
    }
}

pub fn visit_offset_member<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _offset_member: &'ast OffsetMember, _span: &'ast Span) {
    match _offset_member {
        OffsetMember::Member(m) => _visitor.visit_identifier(&m.node, &m.span),
        OffsetMember::IndirectMember(m) => _visitor.visit_identifier(&m.node, &m.span),
        OffsetMember::Index(i) => _visitor.visit_expression(&i.node, &i.span)
    }
}

pub fn visit_declaration<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _declaration: &'ast Declaration, _span: &'ast Span) {
    for specifier in &_declaration.specifiers {
        _visitor.visit_declaration_specifier(&specifier.node, &specifier.span);
    }

    for declarator in &_declaration.declarators {
        _visitor.visit_init_declarator(&declarator.node, &declarator.span);
    }
}

pub fn visit_declaration_specifier<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _declaration_specifier: &'ast DeclarationSpecifier, _span: &'ast Span) {
    match _declaration_specifier {
        DeclarationSpecifier::StorageClass(s) => _visitor.visit_storage_class_specifier(&s.node, &s.span),
        DeclarationSpecifier::TypeSpecifier(t) => _visitor.visit_type_specifier(&t.node, &t.span),
        DeclarationSpecifier::TypeQualifier(t) => _visitor.visit_type_qualifier(&t.node, &t.span),
        DeclarationSpecifier::Function(f) => _visitor.visit_function_specifier(&f.node, &f.span),
        DeclarationSpecifier::Alignment(a) => _visitor.visit_alignment_specifier(&a.node, &a.span),
        DeclarationSpecifier::Extension(e) => {
            for extension in e {
                _visitor.visit_extension(&extension.node, &extension.span);
            }
        }
    }
}

pub fn visit_init_declarator<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _init_declarator: &'ast InitDeclarator, _span: &'ast Span) {
    _visitor.visit_declarator(&_init_declarator.declarator.node, &_init_declarator.declarator.span);
    if let Some(ref initializer) = _init_declarator.initializer {
        _visitor.visit_initializer(&initializer.node, &initializer.span);
    }
}

pub fn visit_storage_class_specifier<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _storage_class_specifier: &'ast StorageClassSpecifier, _span: &'ast Span) {
}

pub fn visit_type_specifier<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _type_specifier: &'ast TypeSpecifier, _span: &'ast Span) {
    match _type_specifier {
        TypeSpecifier::Atomic(a) => _visitor.visit_type_name(&a.node, &a.span),
        TypeSpecifier::Struct(s) => _visitor.visit_struct_type(&s.node, &s.span),
        TypeSpecifier::Enum(e) => _visitor.visit_enum_type(&e.node, &e.span),
        TypeSpecifier::TypedefName(t) => _visitor.visit_identifier(&t.node, &t.span),
        TypeSpecifier::TypeOf(t) => _visitor.visit_type_of(&t.node, &t.span),
        TypeSpecifier::TS18661Float(t) => _visitor.visit_ts18661_float_type(t, _span),
        _ => {}
    }
}

pub fn visit_ts18661_float_type<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _ts18661_float_type: &'ast TS18661FloatType, _span: &'ast Span) {
    _visitor.visit_ts18661_float_format(&_ts18661_float_type.format, _span);
}

pub fn visit_ts18661_float_format<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _ts18661_float_format: &'ast TS18661FloatFormat, _span: &'ast Span) {
}

pub fn visit_struct_type<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _struct_type: &'ast StructType, _span: &'ast Span) {
    _visitor.visit_struct_kind(&_struct_type.kind.node, &_struct_type.kind.span);
    if let Some(ref identifier) = &_struct_type.identifier {
        _visitor.visit_identifier(&identifier.node, &identifier.span);
    }
    for declaration in &_struct_type.declarations {
        _visitor.visit_struct_declaration(&declaration.node, &declaration.span);
    }
}

pub fn visit_struct_kind<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _struct_kind: &'ast StructKind, _span: &'ast Span) {
}

pub fn visit_struct_declaration<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _struct_declaration: &'ast StructDeclaration, _span: &'ast Span) {
    match _struct_declaration {
        StructDeclaration::Field(f) => _visitor.visit_struct_field(&f.node, &f.span),
        StructDeclaration::StaticAssert(s) => _visitor.visit_static_assert(&s.node, &s.span)
    }
}

pub fn visit_struct_field<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _struct_field: &'ast StructField, _span: &'ast Span) {
    for specifier in &_struct_field.specifiers {
        _visitor.visit_specifier_qualifier(&specifier.node, &specifier.span);
    }
    for declarator in &_struct_field.declarators {
        _visitor.visit_struct_declarator(&declarator.node, &declarator.span);
    }
}

pub fn visit_specifier_qualifier<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _specifier_qualifier: &'ast SpecifierQualifier, _span: &'ast Span) {
    match _specifier_qualifier {
        SpecifierQualifier::TypeSpecifier(t) => _visitor.visit_type_specifier(&t.node, &t.span),
        SpecifierQualifier::TypeQualifier(t) => _visitor.visit_type_qualifier(&t.node, &t.span)
    }
}

pub fn visit_struct_declarator<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _struct_declarator: &'ast StructDeclarator, _span: &'ast Span) {
    if let Some(ref declarator) = _struct_declarator.declarator {
        _visitor.visit_declarator(&declarator.node, &declarator.span);
    }
    if let Some(ref bit_width) = _struct_declarator.bit_width {
        _visitor.visit_expression(&bit_width.node, &bit_width.span);
    }
}

pub fn visit_enum_type<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _enum_type: &'ast EnumType, _span: &'ast Span) {
    if let Some(ref identifier) = _enum_type.identifier {
        _visitor.visit_identifier(&identifier.node, &identifier.span);
    }
    for enumerator in &_enum_type.enumerators {
        _visitor.visit_enumerator(&enumerator.node, &enumerator.span);
    }
}

pub fn visit_enumerator<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _enumerator: &'ast Enumerator, _span: &'ast Span) {
    _visitor.visit_identifier(&_enumerator.identifier.node, &_enumerator.identifier.span);
    if let Some(ref expression) = _enumerator.expression {
        _visitor.visit_expression(&expression.node, &expression.span);
    }
}

pub fn visit_type_qualifier<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _type_qualifier: &'ast TypeQualifier, _span: &'ast Span) {
}

pub fn visit_function_specifier<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _function_specifier: &'ast FunctionSpecifier, _span: &'ast Span) {
}

pub fn visit_alignment_specifier<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _alignment_specifier: &'ast AlignmentSpecifier, _span: &'ast Span) {
    match _alignment_specifier {
        AlignmentSpecifier::Type(t) => _visitor.visit_type_name(&t.node, &t.span),
        AlignmentSpecifier::Constant(c) => _visitor.visit_expression(&c.node, &c.span)
    }
}

pub fn visit_declarator<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _declarator: &'ast Declarator, _span: &'ast Span) {
    _visitor.visit_declarator_kind(&_declarator.kind.node, &_declarator.kind.span);
    for derived in &_declarator.derived {
        _visitor.visit_derived_declarator(&derived.node, &derived.span);
    }
    for extension in &_declarator.extensions {
        _visitor.visit_extension(&extension.node, &extension.span);
    }
}

pub fn visit_declarator_kind<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _declarator_kind: &'ast DeclaratorKind, _span: &'ast Span) {
    match _declarator_kind {
        DeclaratorKind::Identifier(i) => _visitor.visit_identifier(&i.node, &i.span),
        DeclaratorKind::Declarator(d) => _visitor.visit_declarator(&d.node, &d.span),
        _ => {}
    }
}

pub fn visit_derived_declarator<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _derived_declarator: &'ast DerivedDeclarator, _span: &'ast Span) {
    match _derived_declarator {
        DerivedDeclarator::Pointer(p) => {
            for pointer in p {
                _visitor.visit_pointer_qualifier(&pointer.node, &pointer.span);
            }
        },
        DerivedDeclarator::Array(a) => _visitor.visit_array_declarator(&a.node, &a.span),
        DerivedDeclarator::Function(f) => _visitor.visit_function_declarator(&f.node, &f.span),
        DerivedDeclarator::KRFunction(k) => {
            for identifier in k {
                _visitor.visit_identifier(&identifier.node, &identifier.span);
            }
        }
    }
}

pub fn visit_array_declarator<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _array_declarator: &'ast ArrayDeclarator, _span: &'ast Span) {
    for qualifier in &_array_declarator.qualifiers {
        _visitor.visit_type_qualifier(&qualifier.node, &qualifier.span);
    }
}

pub fn visit_function_declarator<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _function_declarator: &'ast FunctionDeclarator, _span: &'ast Span) {
    for parameter in &_function_declarator.parameters {
        _visitor.visit_parameter_declaration(&parameter.node, &parameter.span);
    }
    _visitor.visit_ellipsis(&_function_declarator.ellipsis, _span);
}

pub fn visit_pointer_qualifier<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _pointer_qualifier: &'ast PointerQualifier, _span: &'ast Span) {
    match _pointer_qualifier {
        PointerQualifier::TypeQualifier(t) => _visitor.visit_type_qualifier(&t.node, &t.span),
        PointerQualifier::Extension(e) => {
            for extension in e {
                _visitor.visit_extension(&extension.node, &extension.span);
            }
        }
    }
}

pub fn visit_array_size<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _array_size: &'ast ArraySize, _span: &'ast Span) {
    match _array_size {
        ArraySize::VariableExpression(e) => _visitor.visit_expression(&e.node, &e.span),
        ArraySize::StaticExpression(s) => _visitor.visit_expression(&s.node, &s.span),
        _ => {}
    }
}

pub fn visit_parameter_declaration<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _parameter_declaration: &'ast ParameterDeclaration, _span: &'ast Span) {
    for specifier in &_parameter_declaration.specifiers {
        _visitor.visit_declaration_specifier(&specifier.node, &specifier.span);
    }
    if let Some(ref declarator) = _parameter_declaration.declarator {
        _visitor.visit_declarator(&declarator.node, &declarator.span);
    }
    for extension in &_parameter_declaration.extensions {
        _visitor.visit_extension(&extension.node, &extension.span);
    }
}

pub fn visit_ellipsis<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _ellipsis: &'ast Ellipsis, _span: &'ast Span) {
}

pub fn visit_type_name<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _type_name: &'ast TypeName, _span: &'ast Span) {
    for specifier in &_type_name.specifiers {
        _visitor.visit_specifier_qualifier(&specifier.node, &specifier.span);
    }
    if let Some(ref declarator) = _type_name.declarator {
        _visitor.visit_declarator(&declarator.node, &declarator.span);
    }
}

pub fn visit_initializer<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _initializer: &'ast Initializer, _span: &'ast Span) {
    match _initializer {
        Initializer::Expression(e) => _visitor.visit_expression(&e.node, &e.span),
        Initializer::List(l) => {
            for item in l {
                _visitor.visit_initializer_list_item(&item.node, &item.span);
            }
        }
    }
}

pub fn visit_initializer_list_item<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _initializer_list_item: &'ast InitializerListItem, _span: &'ast Span) {
    for designation in &_initializer_list_item.designation {
        _visitor.visit_designator(&designation.node, &designation.span);
    }
    _visitor.visit_initializer(&_initializer_list_item.initializer.node, &_initializer_list_item.initializer.span);
}

pub fn visit_designator<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _designator: &'ast Designator, _span: &'ast Span) {
    match _designator {
        Designator::Index(i) => _visitor.visit_expression(&i.node, &i.span),
        Designator::Member(m) => _visitor.visit_identifier(&m.node, &m.span),
        Designator::Range(r) => _visitor.visit_range_designator(&r.node, &r.span)
    }
}

pub fn visit_range_designator<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _range_designator: &'ast RangeDesignator, _span: &'ast Span) {
    _visitor.visit_expression(&_range_designator.from.node, &_range_designator.from.span);
    _visitor.visit_expression(&_range_designator.to.node, &_range_designator.to.span);
}

pub fn visit_static_assert<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _static_assert: &'ast StaticAssert, _span: &'ast Span) {
    _visitor.visit_expression(&_static_assert.expression.node, &_static_assert.expression.span);
    _visitor.visit_string_literal(&_static_assert.message.node, &_static_assert.message.span);
}

pub fn visit_statement<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _statement: &'ast Statement, _span: &'ast Span) {
    match _statement {
        Statement::Labeled(l) => _visitor.visit_labeled_statement(&l.node, &l.span),
        Statement::Compound(c) => {
            for item in c {
                _visitor.visit_block_item(&item.node, &item.span);
            }
        },
        Statement::Expression(e) => if let Some(ref e) = e {
            _visitor.visit_expression(&e.node, &e.span);
        },
        Statement::If(i) => _visitor.visit_if_statement(&i.node, &i.span),
        Statement::Switch(s) => _visitor.visit_switch_statement(&s.node, &s.span),
        Statement::While(w) => _visitor.visit_while_statement(&w.node, &w.span),
        Statement::DoWhile(d) => _visitor.visit_do_while_statement(&d.node, &d.span),
        Statement::For(f) => _visitor.visit_for_statement(&f.node, &f.span),
        Statement::Goto(g) => _visitor.visit_identifier(&g.node, &g.span),
        Statement::Return(r) => if let Some(ref r) = r {
            _visitor.visit_expression(&r.node, &r.span);
        },
        Statement::Asm(a) => _visitor.visit_asm_statement(&a.node, &a.span),
        _ => {}
    }
}

pub fn visit_labeled_statement<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _labeled_statement: &'ast LabeledStatement, _span: &'ast Span) {
    _visitor.visit_label(&_labeled_statement.label.node, &_labeled_statement.label.span);
    _visitor.visit_statement(&_labeled_statement.statement.node, &_labeled_statement.statement.span);
}

pub fn visit_if_statement<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _if_statement: &'ast IfStatement, _span: &'ast Span) {
    _visitor.visit_expression(&_if_statement.condition.node, &_if_statement.condition.span);
    _visitor.visit_statement(&_if_statement.then_statement.node, &_if_statement.then_statement.span);
    if let Some(ref e) = _if_statement.else_statement {
        _visitor.visit_statement(&e.node, &e.span);
    }
}

pub fn visit_switch_statement<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _switch_statement: &'ast SwitchStatement, _span: &'ast Span) {
    _visitor.visit_expression(&_switch_statement.expression.node, &_switch_statement.expression.span);
    _visitor.visit_statement(&_switch_statement.statement.node, &_switch_statement.statement.span);
}

pub fn visit_while_statement<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _while_statement: &'ast WhileStatement, _span: &'ast Span) {
    _visitor.visit_expression(&_while_statement.expression.node, &_while_statement.expression.span);
    _visitor.visit_statement(&_while_statement.statement.node, &_while_statement.statement.span);
}

pub fn visit_do_while_statement<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _do_while_statement: &'ast DoWhileStatement, _span: &'ast Span) {
    _visitor.visit_expression(&_do_while_statement.expression.node, &_do_while_statement.expression.span);
    _visitor.visit_statement(&_do_while_statement.statement.node, &_do_while_statement.statement.span);
}

pub fn visit_for_statement<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _for_statement: &'ast ForStatement, _span: &'ast Span) {
    _visitor.visit_for_initializer(&_for_statement.initializer.node, &_for_statement.initializer.span);
    if let Some(ref c) = _for_statement.condition {
        _visitor.visit_expression(&c.node, &c.span);
    }
    if let Some(ref s) = _for_statement.step {
        _visitor.visit_expression(&s.node, &s.span);
    }
    _visitor.visit_statement(&_for_statement.statement.node, &_for_statement.statement.span);
}

pub fn visit_label<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _label: &'ast Label, _span: &'ast Span) {
    match _label {
        Label::Identifier(i) => _visitor.visit_identifier(&i.node, &i.span),
        Label::Case(c) => _visitor.visit_expression(&c.node, &c.span),
        Label::Default => {}
    }
}

pub fn visit_for_initializer<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _for_initializer: &'ast ForInitializer, _span: &'ast Span) {
    match _for_initializer {
        ForInitializer::Empty => {},
        ForInitializer::Expression(e) => _visitor.visit_expression(&e.node, &e.span),
        ForInitializer::Declaration(d) => _visitor.visit_declaration(&d.node, &d.span),
        ForInitializer::StaticAssert(s) => _visitor.visit_static_assert(&s.node, &s.span)
    }
}

pub fn visit_block_item<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _block_item: &'ast BlockItem, _span: &'ast Span) {
    match _block_item {
        BlockItem::Declaration(d) => _visitor.visit_declaration(&d.node, &d.span),
        BlockItem::StaticAssert(s) => _visitor.visit_static_assert(&s.node, &s.span),
        BlockItem::Statement(s) => _visitor.visit_statement(&s.node, &s.span)
    }
}

pub fn visit_translation_unit<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _translation_unit: &'ast TranslationUnit) {
    for element in &_translation_unit.0 {
        _visitor.visit_external_declaration(&element.node, &element.span);
    }
}

pub fn visit_external_declaration<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _external_declaration: &'ast ExternalDeclaration, _span: &'ast Span) {
    match _external_declaration {
        ExternalDeclaration::Declaration(d) => _visitor.visit_declaration(&d.node, &d.span),
        ExternalDeclaration::StaticAssert(s) => _visitor.visit_static_assert(&s.node, &s.span),
        ExternalDeclaration::FunctionDefinition(f) => _visitor.visit_function_definition(&f.node, &f.span)
    }
}

pub fn visit_function_definition<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _function_definition: &'ast FunctionDefinition, _span: &'ast Span) {
    for specifier in &_function_definition.specifiers {
        _visitor.visit_declaration_specifier(&specifier.node, &specifier.span);
    }
    _visitor.visit_declarator(&_function_definition.declarator.node, &_function_definition.declarator.span);
    for declaration in &_function_definition.declarations {
        _visitor.visit_declaration(&declaration.node, &declaration.span);
    }
    _visitor.visit_statement(&_function_definition.statement.node, &_function_definition.statement.span);
}

pub fn visit_extension<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _extension: &'ast Extension, _span: &'ast Span) {
    match _extension {
        Extension::Attribute(a) => _visitor.visit_attribute(a, _span),
        Extension::AsmLabel(a) => _visitor.visit_string_literal(&a.node, &a.span)
    }
}

pub fn visit_attribute<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _attribute: &'ast Attribute, _span: &'ast Span) {
    for argument in &_attribute.arguments {
        _visitor.visit_expression(&argument.node, &argument.span);
    }
}

pub fn visit_asm_statement<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _asm_statement: &'ast AsmStatement, _span: &'ast Span) {
    match _asm_statement {
        AsmStatement::GnuBasic(g) => _visitor.visit_string_literal(&g.node, &g.span),
        AsmStatement::GnuExtended(g) => _visitor.visit_gnu_extended_asm_statement(g, _span)
    }
}

pub fn visit_gnu_extended_asm_statement<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _gnu_extended_asm_statement: &'ast GnuExtendedAsmStatement, _span: &'ast Span) {
    if let Some(ref qualifier) = _gnu_extended_asm_statement.qualifier {
        _visitor.visit_type_qualifier(&qualifier.node, &qualifier.span);
    }
    _visitor.visit_string_literal(&_gnu_extended_asm_statement.template.node, &_gnu_extended_asm_statement.template.span);
    for output in &_gnu_extended_asm_statement.outputs {
        _visitor.visit_gnu_asm_operand(&output.node, &output.span);
    }
    for input in &_gnu_extended_asm_statement.inputs {
        _visitor.visit_gnu_asm_operand(&input.node, &input.span);
    }
    for clobber in &_gnu_extended_asm_statement.clobbers {
        _visitor.visit_string_literal(&clobber.node, &clobber.span);
    }
}

pub fn visit_gnu_asm_operand<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _gnu_asm_operand: &'ast GnuAsmOperand, _span: &'ast Span) {
    if let Some(ref name) = _gnu_asm_operand.symbolic_name {
        _visitor.visit_identifier(&name.node, &name.span);
    }
    _visitor.visit_string_literal(&_gnu_asm_operand.constraints.node, &_gnu_asm_operand.constraints.span);
    _visitor.visit_expression(&_gnu_asm_operand.variable_name.node, &_gnu_asm_operand.variable_name.span);
}

pub fn visit_type_of<'ast, V: Visit<'ast> + ?Sized>(_visitor: &mut V, _type_of: &'ast TypeOf, _span: &'ast Span) {
    match _type_of {
        TypeOf::Expression(e) => _visitor.visit_expression(&e.node, &e.span),
        TypeOf::Type(t) => _visitor.visit_type_name(&t.node, &t.span)
    }
}
