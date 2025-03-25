use crate::*;

impl InnerControllerData {
    pub fn new() -> Self {
        InnerControllerData {
            socket: None,
            request: Request::new(),
            response: Response::default(),
            log: Log::default(),
            socket_addr: None,
        }
    }
}

impl ControllerData {
    pub(crate) fn from_controller_data(controller_data: InnerControllerData) -> Self {
        Self(Arc::new(RwLock::new(controller_data)))
    }

    pub async fn get_read_lock(&self) -> RwLockReadControllerData {
        self.0.read().await
    }

    pub async fn get_write_lock(&self) -> RwLockWriteControllerData {
        self.0.write().await
    }

    pub async fn get(&self) -> InnerControllerData {
        self.get_read_lock().await.clone()
    }

    pub async fn get_request(&self) -> Request {
        self.get().await.get_request().clone()
    }

    pub async fn get_response(&self) -> Response {
        self.get().await.get_response().clone()
    }

    pub async fn get_log(&self) -> Log {
        self.get().await.get_log().clone()
    }

    pub async fn get_socket(&self) -> OptionArcRwLockUdpSocket {
        self.get().await.get_socket().clone()
    }

    pub async fn get_socket_addr(&self) -> OptionSocketAddr {
        self.get().await.get_socket_addr().clone()
    }

    pub async fn get_socket_addr_or_default(&self) -> SocketAddr {
        let socket_result: OptionArcRwLockUdpSocket = self.get_socket().await;
        if socket_result.is_none() {
            return DEFAULT_SOCKET_ADDR;
        }
        let socket_addr: SocketAddr = socket_result
            .unwrap()
            .get_read_lock()
            .await
            .peer_addr()
            .unwrap_or(DEFAULT_SOCKET_ADDR);
        socket_addr
    }

    pub async fn get_socket_addr_string(&self) -> Option<String> {
        self.get_socket_addr().await.map(|data| data.to_string())
    }

    pub async fn get_socket_addr_or_default_string(&self) -> String {
        self.get_socket_addr_or_default().await.to_string()
    }

    pub async fn get_socket_host(&self) -> OptionSocketHost {
        self.get_socket_addr()
            .await
            .map(|socket_addr: SocketAddr| socket_addr.ip())
    }

    pub async fn get_socket_port(&self) -> OptionSocketPort {
        self.get_socket_addr()
            .await
            .map(|socket_addr: SocketAddr| socket_addr.port())
    }

    pub(super) async fn set_response<T: Into<ResponseData>>(&self, data: T) -> &Self {
        self.get_write_lock()
            .await
            .set_response(Response::from(data));
        self
    }

    pub async fn log_info<L>(&self, data: &str, func: L) -> &Self
    where
        L: LogFuncTrait,
    {
        self.get_read_lock().await.get_log().info(data, func);
        self
    }

    pub async fn log_debug<L>(&self, data: &str, func: L) -> &Self
    where
        L: LogFuncTrait,
    {
        self.get_read_lock().await.get_log().debug(data, func);
        self
    }

    pub async fn log_error<L>(&self, data: &str, func: L) -> &Self
    where
        L: LogFuncTrait,
    {
        self.get_read_lock().await.get_log().error(data, func);
        self
    }

    pub async fn async_log_info<L>(&self, data: &str, func: L) -> &Self
    where
        L: LogFuncTrait,
    {
        self.get_read_lock().await.get_log().async_info(data, func);
        self
    }

    pub async fn async_log_debug<L>(&self, data: &str, func: L) -> &Self
    where
        L: LogFuncTrait,
    {
        self.get_read_lock().await.get_log().async_debug(data, func);
        self
    }

    pub async fn async_log_error<L>(&self, data: &str, func: L) -> &Self
    where
        L: LogFuncTrait,
    {
        self.get_read_lock().await.get_log().async_error(data, func);
        self
    }

    pub async fn send<T: Into<ResponseData>>(&self, data: T) -> ResponseResult {
        let response_result: ResponseResult = self
            .set_response(data)
            .await
            .get_response()
            .await
            .send(&self.get_socket().await, &self.get_socket_addr().await)
            .await;
        return response_result;
    }
}
