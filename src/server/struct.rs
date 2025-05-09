use crate::*;

#[derive(Clone, Lombok)]
pub struct Server {
    pub(super) cfg: ArcRwLockServerConfig,
    pub(super) func_list: ArcRwLockVecFuncBox,
    pub(super) tmp: ArcRwLock<Tmp>,
}
