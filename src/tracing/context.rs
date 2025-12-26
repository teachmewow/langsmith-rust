use uuid::Uuid;

/// Context for trace propagation
#[derive(Debug, Clone)]
pub struct TraceContext {
    pub trace_id: Uuid,
    pub parent_run_id: Option<Uuid>,
    pub dotted_order: Option<String>,
    pub thread_id: Option<String>,
    pub session_name: Option<String>,
}

impl TraceContext {
    pub fn new(trace_id: Uuid) -> Self {
        Self {
            trace_id,
            parent_run_id: None,
            dotted_order: None,
            thread_id: None,
            session_name: None,
        }
    }

    pub fn with_parent(mut self, parent_run_id: Uuid) -> Self {
        self.parent_run_id = Some(parent_run_id);
        self
    }

    pub fn with_dotted_order(mut self, dotted_order: String) -> Self {
        self.dotted_order = Some(dotted_order);
        self
    }

    pub fn with_thread_id(mut self, thread_id: String) -> Self {
        self.thread_id = Some(thread_id);
        self
    }

    pub fn with_session_name(mut self, session_name: String) -> Self {
        self.session_name = Some(session_name);
        self
    }
}

