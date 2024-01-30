use insta::assert_debug_snapshot;
use roadiebag2::app::App;
use loco_rs::testing;
use serial_test::serial;
use test_log::test;
use tracing_test::traced_test;
use roadiebag2::models::{items, taken_items};

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
    };
}

#[tokio::test]
#[serial]
#[traced_test]
async fn test_model() {
    configure_insta!();

    let boot = testing::boot_test::<App>().await.unwrap();
    testing::seed::<App>(&boot.app_context.db).await.unwrap();

    let create = interface::CreateUpdateItem {
        name: "Test item".to_string(),
        description: None,
        quantity: 2,
        size: interface::ItemSize::Small,
        infinite: false
    };

    let model = items::Model::create(&boot.app_context.db, create).await;

    let current_taken_none = taken_items::Model::get_current(&boot.app_context.db).await.unwrap();

    let current_random = taken_items::Model::get_random(&boot.app_context.db).await.unwrap();

    taken_items::Model::mark_done(&boot.app_context.db).await.unwrap();

    let current_taken_done = taken_items::Model::get_current(&boot.app_context.db).await.unwrap();


    insta::with_settings!({
        filters => {
            let mut fvec = testing::CLEANUP_DATE.to_vec();
            fvec.extend(vec![
                (r"rounds_left: \d", "rounds_left: ROUNDS_LEFT"),
                (r"rounds_total: \d", "rounds_total: ROUNDS_TOTAL")
            ]);
            fvec
        }
    }, {
        assert_debug_snapshot!((
            current_taken_none,
            current_random,
            current_taken_done
        ))
    });
}

#[tokio::test]
#[serial]
async fn test_decrement() {
    configure_insta!();

    let boot = testing::boot_test::<App>().await.unwrap();
    testing::seed::<App>(&boot.app_context.db).await.unwrap();

    let create = interface::CreateUpdateItem {
        name: "Test item".to_string(),
        description: None,
        quantity: 2,
        size: interface::ItemSize::Small,
        infinite: false
    };

    let _model = items::Model::create(&boot.app_context.db, create).await;
    let current_random = taken_items::Model::get_random(&boot.app_context.db).await.unwrap();

    let decr = taken_items::Model::decrement_rounds(&boot.app_context.db).await.unwrap().unwrap();

    assert_eq!(current_random.rounds_left - 1, decr.rounds_left)
}

