extern crate lang_c;
use lang_c::ast::*;
use std::mem::size_of;

macro_rules! ps {
    ($( $t:ty )+) => ({ $(println!("{:3}  {}", size_of::<$t>(), stringify!($t));)+ })
}

fn print_all<T: Name>() {
    ps! {
        TypeOf<T>
        Identifier<T>
        Constant
        Integer
        Float
        Expression<T>
        MemberOperator
        GenericAssociation<T>
        UnaryOperator
        BinaryOperator
        OffsetDesignator<T>
        OffsetMember<T>
        Declaration<T>
        DeclarationSpecifier<T>
        InitDeclarator<T>
        StorageClassSpecifier
        TypeSpecifier<T>
        StructType<T>
        StructDeclaration<T>
        SpecifierQualifier<T>
        StructDeclarator<T>
        Enumerator<T>
        TypeQualifier
        FunctionSpecifier
        AlignmentSpecifier<T>
        Declarator<T>
        DeclaratorKind<T>
        DerivedDeclarator<T>
        PointerQualifier<T>
        ArraySize<T>
        ParameterDeclaration<T>
        Ellipsis
        TypeName<T>
        Initializer<T>
        InitializerListItem<T>
        Designator<T>
        StaticAssert<T>
        Statement<T>
        Label<T>
        ForInitializer<T>
        BlockItem<T>
        TranslationUnit<T>
        ExternalDeclaration<T>
        FunctionDefinition<T>
        Extension<T>
        AsmStatement<T>
        GnuAsmOperand<T>
        TypeOf<T>
    }
}

fn main() {
    print_all::<String>();
}
