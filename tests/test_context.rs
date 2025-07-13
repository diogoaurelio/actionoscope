use log::error;

pub struct TearDownTestContext<F>
where
    F: FnOnce() + Send + 'static,
{
    tear_down_fn: Option<F>,
}

impl<F> TearDownTestContext<F>
where
    F: FnOnce() + Send + 'static,
{
    pub fn new(tear_down_fn: F) -> Self {
        TearDownTestContext {
            tear_down_fn: Some(tear_down_fn),
        }
    }
}

impl<F> Drop for TearDownTestContext<F>
where
    F: FnOnce() + Send + 'static,
{
    fn drop(&mut self) {
        if let Some(tear_down_fn) = self.tear_down_fn.take() {
            tear_down_fn();
        } else {
            error!("Tear down function was not set or already executed.");
        }
    }
}
