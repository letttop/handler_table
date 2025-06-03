use core::sync::atomic::{AtomicBool, Ordering};
use handler_table::HandlerTable;

static CALLED: AtomicBool = AtomicBool::new(false);

fn handler() {
    CALLED.store(true, Ordering::SeqCst);
}

#[test]
fn test_default() {
    let table = HandlerTable::<3>::default();
    assert!(!table.handle(0)); // Should be empty
}

#[test]
fn test_register_and_handle() {
    CALLED.store(false, Ordering::SeqCst);
    let table = HandlerTable::<4>::new();

    // Register and call handler
    assert!(table.register_handler(1, handler));
    assert!(table.handle(1));
    assert!(CALLED.load(Ordering::SeqCst));

    // Duplicate registration should fail
    assert!(!table.register_handler(1, handler));
}

#[test]
fn test_unregister_and_handle() {
    CALLED.store(false, Ordering::SeqCst);
    let table = HandlerTable::<4>::new();

    // Register handler
    assert!(table.register_handler(2, handler));

    // Unregister and get the original handler, calling it should set CALLED
    let h = table.unregister_handler(2).expect("should have handler");
    h();
    assert!(CALLED.load(Ordering::SeqCst));

    // After unregistering, handle should return false
    CALLED.store(false, Ordering::SeqCst);
    assert!(!table.handle(2));
    assert!(!CALLED.load(Ordering::SeqCst));
}

#[test]
fn test_out_of_bounds() {
    let table = HandlerTable::<2>::new();

    // Out-of-bounds operations should fail or return None
    assert!(!table.register_handler(2, handler));
    assert!(table.unregister_handler(2).is_none());
    assert!(!table.handle(2));
}

#[test]
fn test_unregister_empty_slot() {
    let table = HandlerTable::<4>::new();

    // Unregistering from empty slot should return None
    assert!(table.unregister_handler(1).is_none());
}
