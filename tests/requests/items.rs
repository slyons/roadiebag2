use insta::{assert_debug_snapshot, with_settings};
use loco_rs::testing;
use roadiebag2::app::App;
use serial_test::serial;

use super::prepare_data;

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("item_request");
        let _guard = settings.bind_to_scope();
    };
}

#[tokio::test]
#[serial]
async fn item_crud() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;

        let create = interface::CreateUpdateItem {
            name: "Test item".to_string(),
            description: None,
            quantity: 2,
            size: interface::ItemSize::Small,
            infinite: false,
        };

        let (auth_key, auth_value) = prepare_data::auth_header(&user.token);
        let create_response = request
            .post("/api/items")
            .json(&create)
            .add_header(auth_key.clone(), auth_value.clone())
            .await;
        create_response.assert_status_ok();
        let model: interface::Item = create_response.clone().json();

        let read_response = request
            .get(&format!("/api/items/{}", model.id))
            .add_header(auth_key.clone(), auth_value.clone())
            .await;
        read_response.assert_status_ok();

        let update = interface::CreateUpdateItem {
            quantity: 4,
            ..create
        };

        let update_response = request
            .post(&format!("/api/items/{}", model.id))
            .json(&update)
            .add_header(auth_key.clone(), auth_value.clone())
            .await;
        update_response.assert_status_ok();

        let delete_response = request
            .delete(&format!("/api/items/{}", model.id))
            .add_header(auth_key, auth_value)
            .await;
        delete_response.assert_status_ok();

        insta::with_settings!({
            filters => testing::CLEANUP_DATE.to_vec()
            }, {
            assert_debug_snapshot!((
                //Create
                (create_response.status_code(), create_response.text()),
                //Read
                (read_response.status_code(), read_response.text()),
                //Update
                (update_response.status_code(), update_response.text()),
                //Delete
                (delete_response.status_code(), delete_response.status_code())
            ))
        });
    })
    .await;
}
