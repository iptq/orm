use super::QueryClause;
use Backend;

pub struct Select {
    from_clause: Option<SelectFromClause>,
    where_clause: SelectWhereClause,
    inner_columns: Vec<SelectColumn>,
    limit_clause: Option<SelectLimitClause>,
}

pub struct SelectColumn {}

pub struct SelectFromClause {}

pub struct SelectLimitClause {}

pub enum SelectWhereClause {}
