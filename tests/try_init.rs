use tracing_subscriber::Layer;
use tracing_subscriber_init::{TestAll, full_filtered, init, try_init};

#[test]
fn init_works_then_try_init_err() {
    let config = TestAll;
    let layer = full_filtered(&config);
    init(vec![layer.boxed()]);
    let layer = full_filtered(&config);
    let res = try_init(vec![layer.boxed()]);
    assert!(res.is_err());
}
