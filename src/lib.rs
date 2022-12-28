use std::sync::Arc;

type Callback = Arc<dyn Fn() + Sync + Send + 'static>;

#[derive(Clone)]
pub struct RBXScriptSignal {
    connection: Callback,
    call_once: bool,
}

impl RBXScriptSignal {
    pub fn new() -> Self {
        Self {
            connection: Arc::new(Box::new(|| ())),
            call_once: false,
        }
    }

    pub fn connect(&mut self, callback: Callback) {
        self.call_once = false;
        self.connection = callback
    }

    pub fn fire(&mut self) {
        (self.connection)();
        if self.call_once {
            self.disconnect();
        }
    }

    pub fn disconnect(&mut self) {
        self.connection = Arc::new(Box::new(|| ()));
    }

    pub fn connect_once(&mut self, callback: Callback) {
        self.call_once = true;
        self.connection = callback
    }
}
