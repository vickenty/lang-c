extern crate lang_c;
use lang_c::ast::*;
use std::mem::size_of;

macro_rules! ps {
    ($( $i:ident )+) => ({ $(println!("{:3}  {}", size_of::<$i>(), stringify!($i));)+ })
}

fn main() {
    ps! {
        TypeOf
        Identifier
        Constant
        Integer
        Float
        Expression
        MemberOperator
        GenericAssociation
        UnaryOperator
        BinaryOperator
        OffsetDesignator
        OffsetMember
        Declaration
        DeclarationSpecifier
        InitDeclarator
        StorageClassSpecifier
        TypeSpecifier
        StructType
        StructDeclaration
        SpecifierQualifier
        StructDeclarator
        Enumerator
        TypeQualifier
        FunctionSpecifier
        AlignmentSpecifier
        Declarator
        DeclaratorKind
        DerivedDeclarator
        PointerQualifier
        ArraySize
        ParameterDeclaration
        Ellipsis
        TypeName
        Initializer
        InitializerListItem
        Designator
        StaticAssert
        Statement
        Label
        ForInitializer
        BlockItem
        TranslationUnit
        ExternalDeclaration
        FunctionDefinition
        Extension
        AsmStatement
        GnuAsmOperand
        TypeOf
    }
}
