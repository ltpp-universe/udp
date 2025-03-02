use crate::*;

impl ControllerData {
    #[inline]
    pub fn new() -> Self {
        ControllerData {
            socket: None,
            request: Request::new(),
            response: Response::default(),
            log: Log::default(),
            addr: None,
        }
    }
}

impl ArcRwLockControllerData {
    #[inline]
    pub(crate) fn from_controller_data(controller_data: ControllerData) -> Self {
        Self(Arc::new(RwLock::new(controller_data)))
    }

    #[inline]
    pub async fn get_read_lock(&self) -> RwLockReadControllerData {
        let controller_data: RwLockReadControllerData = self.0.read().await;
        controller_data
    }

    #[inline]
    pub async fn get_write_lock(&self) -> RwLockWriteControllerData {
        let controller_data: RwLockWriteControllerData = self.0.write().await;
        controller_data
    }

    #[inline]
    pub async fn get_controller_data(&self) -> ControllerData {
        let controller_data: ControllerData = self.get_read_lock().await.clone();
        controller_data
    }

    #[inline]
    pub async fn get_request(&self) -> Request {
        let controller_data: ControllerData = self.get_controller_data().await;
        controller_data.get_request().clone()
    }

    #[inline]
    pub async fn get_response(&self) -> Response {
        let controller_data: ControllerData = self.get_controller_data().await;
        controller_data.get_response().clone()
    }

    #[inline]
    pub async fn get_log(&self) -> Log {
        let controller_data: ControllerData = self.get_controller_data().await;
        controller_data.get_log().clone()
    }

    #[inline]
    pub async fn get_socket(&self) -> OptionArcRwLockUdpSocket {
        let controller_data: ControllerData = self.get_controller_data().await;
        controller_data.get_socket().clone()
    }

    #[inline]
    pub async fn get_addr(&self) -> OptionSocketAddr {
        let controller_data: ControllerData = self.get_controller_data().await;
        controller_data.get_addr().clone()
    }

    #[inline]
    pub(super) async fn set_response<T: Into<ResponseData>>(&self, data: T) -> &Self {
        let mut controller_data: RwLockWriteControllerData = self.get_write_lock().await;
        controller_data.set_response(server::response::r#type::Response::from(data));
        self
    }

    #[inline]
    pub async fn send<T: Into<ResponseData>>(&self, data: T) -> ResponseResult {
        let response_result: ResponseResult = self
            .set_response(data)
            .await
            .get_response()
            .await
            .send(&self.get_socket().await, &self.get_addr().await)
            .await;
        return response_result;
    }
}
