use insta::{assert_debug_snapshot, with_settings};
use roadiebag2::app::App;
use loco_rs::testing;
use serial_test::serial;
use super::prepare_data;

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("taken_request");
        let _guard = settings.bind_to_scope();
    };
}

#[tokio::test]
#[serial]
async fn taken_crud() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;

        let create = interface::CreateUpdateItem {
            name: "Test item".to_string(),
            description: None,
            quantity: 1,
            size: interface::ItemSize::Small,
            infinite: false
        };

        let (auth_key, auth_value) = prepare_data::auth_header(&user.token);
        let create_response = request
            .post("/api/items")
            .json(&create)
            .add_header(auth_key.clone(), auth_value.clone())
            .await;
        create_response.assert_status_ok();

        let current_response = request
            .get("/api/taken")
            .add_header(auth_key.clone(), auth_value.clone())
            .await;
        current_response.assert_status_ok();

        let get_random = request
            .post("/api/taken")
            .add_header(auth_key.clone(), auth_value.clone())
            .await;
        get_random.assert_status_ok();

        let decr = request
            .post("/api/taken/decrement")
            .add_header(auth_key.clone(), auth_value.clone())
            .await;
        decr.assert_status_ok();

        let done_request = request
            .post("/api/taken/done")
            .add_header(auth_key.clone(), auth_value.clone())
            .await;
        done_request.assert_status_ok();

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
                (current_response.status_code(), current_response.text()),
                (get_random.status_code(), get_random.text()),
                (decr.status_code(), decr.text()),
                (done_request.status_code(), done_request.text())
            ))
        });

    }).await;
}