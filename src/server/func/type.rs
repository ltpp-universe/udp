use crate::*;

pub type FuncBox = Box<dyn Func + Send + 'static>;
pub type FuncListArcLock = ArcRwLock<Vec<FuncBox>>;
