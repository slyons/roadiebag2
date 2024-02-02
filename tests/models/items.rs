use insta::assert_debug_snapshot;
use loco_rs::testing;
use roadiebag2::{app::App, models::items};
use serial_test::serial;

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
    };
}

#[tokio::test]
#[serial]
async fn test_create() {
    configure_insta!();

    let boot = testing::boot_test::<App>().await.unwrap();

    let create = interface::CreateUpdateItem {
        name: "Test item".to_string(),
        description: None,
        quantity: 2,
        size: interface::ItemSize::Small,
        infinite: false,
    };

    let model = items::Model::create(&boot.app_context.db, create).await;
    insta::with_settings!({
        filters => testing::CLEANUP_DATE.to_vec()
    }, {
        assert_debug_snapshot!(model)
    });
}

#[tokio::test]
#[serial]
async fn test_update() {
    configure_insta!();

    let boot = testing::boot_test::<App>().await.unwrap();

    let create = interface::CreateUpdateItem {
        name: "Test item".to_string(),
        description: None,
        quantity: 2,
        size: interface::ItemSize::Small,
        infinite: false,
    };

    let model = items::Model::create(&boot.app_context.db, create.clone()).await;
    assert_eq!(model.is_ok(), true);
    let model = model.unwrap();

    let update = interface::CreateUpdateItem {
        quantity: 4,
        ..create
    };

    let model2 = items::Model::update(&boot.app_context.db, model.id, update).await;
    assert_eq!(model2.is_ok(), true);

    insta::with_settings!({
        filters => testing::CLEANUP_DATE.to_vec()
    }, {
        assert_debug_snapshot!((model, model2))
    });
}

#[tokio::test]
#[serial]
async fn can_delete() {
    configure_insta!();

    let boot = testing::boot_test::<App>().await.unwrap();

    let starting_list = items::Model::list(&boot.app_context.db, None)
        .await
        .unwrap();

    let create = interface::CreateUpdateItem {
        name: "Test item".to_string(),
        description: None,
        quantity: 2,
        size: interface::ItemSize::Small,
        infinite: false,
    };

    let item = items::Model::create(&boot.app_context.db, create)
        .await
        .unwrap();
    let after_create_list = items::Model::list(&boot.app_context.db, None)
        .await
        .unwrap();

    items::Model::delete(&boot.app_context.db, item.id)
        .await
        .unwrap();
    let after_delete_list = items::Model::list(&boot.app_context.db, None)
        .await
        .unwrap();

    insta::with_settings!({
        filters => testing::CLEANUP_DATE.to_vec()
    }, {
        assert_debug_snapshot!((
            starting_list,
            after_create_list,
            after_delete_list
        ))
    });
}

#[tokio::test]
#[serial]
async fn can_filter() {
    configure_insta!();

    let boot = testing::boot_test::<App>().await.unwrap();

    let starting_list = items::Model::list(&boot.app_context.db, None)
        .await
        .unwrap();

    let create = interface::CreateUpdateItem {
        name: "Test item".to_string(),
        description: None,
        quantity: 2,
        size: interface::ItemSize::Small,
        infinite: false,
    };
    let create2 = interface::CreateUpdateItem {
        name: "Test item2".to_string(),
        description: None,
        quantity: 1,
        size: interface::ItemSize::Medium,
        infinite: true,
    };
    let item1 = items::Model::create(&boot.app_context.db, create)
        .await
        .unwrap();
    let item2 = items::Model::create(&boot.app_context.db, create2)
        .await
        .unwrap();
    let all_items = items::Model::list(&boot.app_context.db, None)
        .await
        .unwrap();

    let small_filter = interface::ItemFilter {
        size: Some(interface::ItemSize::Small),
        ..Default::default()
    };
    let small_items = items::Model::list(&boot.app_context.db, Some(small_filter))
        .await
        .unwrap();

    let inf_filter = interface::ItemFilter {
        infinite: Some(true),
        ..Default::default()
    };
    let inf_items = items::Model::list(&boot.app_context.db, Some(inf_filter))
        .await
        .unwrap();

    insta::with_settings!({
        filters => testing::CLEANUP_DATE.to_vec()
    }, {
        assert_debug_snapshot!((
            starting_list,
            all_items,
            small_items,
            inf_items
        ))
    });
}
