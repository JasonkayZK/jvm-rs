use std::fmt::Display;

#[derive(Debug)]
pub enum AttributeTypeNameEnum {
    Code,
    ConstantValue,
    Deprecated,
    Exceptions,
    LineNumberTable,
    LocalVariableTable,
    SourceFile,
    Synthetic,
    Signature,
    EnclosingMethod,
    BootstrapMethods,
    InnerClasses,
    LocalVariableTypeTable,
    Unparsed,
}

impl Display for AttributeTypeNameEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let info = match self {
            AttributeTypeNameEnum::Code => "Code",
            AttributeTypeNameEnum::ConstantValue => "ConstantValue",
            AttributeTypeNameEnum::Deprecated => "Deprecated",
            AttributeTypeNameEnum::Exceptions => "Exceptions",
            AttributeTypeNameEnum::LineNumberTable => "LineNumberTable",
            AttributeTypeNameEnum::LocalVariableTable => "LocalVariableTable",
            AttributeTypeNameEnum::SourceFile => "SourceFile",
            AttributeTypeNameEnum::Synthetic => "Synthetic",
            AttributeTypeNameEnum::Signature => "Signature",
            AttributeTypeNameEnum::EnclosingMethod => "EnclosingMethod",
            AttributeTypeNameEnum::BootstrapMethods => "BootstrapMethods",
            AttributeTypeNameEnum::InnerClasses => "InnerClasses",
            AttributeTypeNameEnum::LocalVariableTypeTable => "LocalVariableTypeTable",
            AttributeTypeNameEnum::Unparsed => "Unparsed",
        };

        write!(f, "{}", info)
    }
}

impl From<AttributeTypeNameEnum> for &str {
    fn from(val: AttributeTypeNameEnum) -> Self {
        match val {
            AttributeTypeNameEnum::Code => "Code",
            AttributeTypeNameEnum::ConstantValue => "ConstantValue",
            AttributeTypeNameEnum::Deprecated => "Deprecated",
            AttributeTypeNameEnum::Exceptions => "Exceptions",
            AttributeTypeNameEnum::LineNumberTable => "LineNumberTable",
            AttributeTypeNameEnum::LocalVariableTable => "LocalVariableTable",
            AttributeTypeNameEnum::SourceFile => "SourceFile",
            AttributeTypeNameEnum::Synthetic => "Synthetic",
            AttributeTypeNameEnum::Signature => "Signature",
            AttributeTypeNameEnum::EnclosingMethod => "EnclosingMethod",
            AttributeTypeNameEnum::BootstrapMethods => "BootstrapMethods",
            AttributeTypeNameEnum::InnerClasses => "InnerClasses",
            AttributeTypeNameEnum::LocalVariableTypeTable => "LocalVariableTypeTable",
            AttributeTypeNameEnum::Unparsed => "Unparsed",
        }
    }
}

impl From<&str> for AttributeTypeNameEnum {
    fn from(val: &str) -> Self {
        match val {
            "Code" => AttributeTypeNameEnum::Code,
            "ConstantValue" => AttributeTypeNameEnum::ConstantValue,
            "Deprecated" => AttributeTypeNameEnum::Deprecated,
            "Exceptions" => AttributeTypeNameEnum::Exceptions,
            "LineNumberTable" => AttributeTypeNameEnum::LineNumberTable,
            "LocalVariableTable" => AttributeTypeNameEnum::LocalVariableTable,
            "SourceFile" => AttributeTypeNameEnum::SourceFile,
            "Synthetic" => AttributeTypeNameEnum::Synthetic,
            "Signature" => AttributeTypeNameEnum::Signature,
            "EnclosingMethod" => AttributeTypeNameEnum::EnclosingMethod,
            "BootstrapMethods" => AttributeTypeNameEnum::BootstrapMethods,
            "InnerClasses" => AttributeTypeNameEnum::InnerClasses,
            "LocalVariableTypeTable" => AttributeTypeNameEnum::LocalVariableTypeTable,
            _ => AttributeTypeNameEnum::Unparsed,
        }
    }
}
