#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Int,
    Float,
    Bool,
    Color,
    String,
    Line,
    LineFill,
    Label,
    Box,
    Table,
    Array(Box<DataType>),
    Matrix(Box<DataType>),
    UDF,


    Null,
    // 其他类型
}