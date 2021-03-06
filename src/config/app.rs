use crate::api::*;
use actix_web::web;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("Configuring routes...");
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .service(
                        web::resource("/signup").route(web::post().to(account_controller::signup)),
                    )
                    .service(
                        web::resource("/login").route(web::post().to(account_controller::login)),
                    )
                    .service(
                        web::resource("/logout").route(web::post().to(account_controller::logout)),
                    ),
            )
            .service(
                web::scope("/user")
                    .service(
                        web::resource("/{email}")
                            .route(web::get().to(account_controller::find_by_id)),
                    )
            )
            .service(
                web::scope("/experience")
                    .service(
                        web::resource("")
                            .route(web::get().to(experience_controller::find_all))
                            .route(web::post().to(experience_controller::insert)),
                    )
                    .service(
                        web::resource("/{country}/{city}")
                            .route(web::get().to(experience_controller::find_by_id))
                            .route(web::put().to(experience_controller::update))
                            .route(web::delete().to(experience_controller::delete)),
                    )
            )
            .service(
                web::scope("/experience_image")
                    .service(
                        web::resource("/{country}/{city}")
                            .route(web::get().to(experience_image_controller::find_all_by_experience))
                    )
                    .service(
                        web::resource("")
                            .route(web::post().to(experience_image_controller::insert)),
                    )
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(experience_image_controller::delete))
                    )
            )
            .service(
                web::scope("/experience_video")
                    .service(
                        web::resource("/{country}/{city}")
                            .route(web::get().to(experience_video_controller::find_all_by_experience))
                    )
                    .service(
                        web::resource("")
                            .route(web::post().to(experience_video_controller::insert)),
                    )
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(experience_video_controller::delete))
                    )
            )
            .service(
                web::scope("/experience_like")
                    .service(
                        web::resource("/{country}/{city}")
                            .route(web::get().to(experience_like_controller::count_all_by_experience))
                            .route(web::delete().to(experience_like_controller::delete))
                            .route(web::post().to(experience_like_controller::insert)),
                    )
                    .service(
                        web::resource("/top")
                            .route(web::get().to(experience_like_controller::get_top_users))
                    )
                    .service(
                        web::resource("/{email}")
                            .route(web::get().to(experience_like_controller::get_user_likes))
                    )
                    .service(
                        web::resource("/{email}/{country}/{city}")
                            .route(web::get().to(experience_like_controller::has_user_liked_experience))
                    )
            )
            .service(
                web::scope("/experience_comment")
                    .service(
                        web::resource("/{country}/{city}")
                            .route(web::get().to(experience_comment_controller::find_all_by_experience))
                            .route(web::post().to(experience_comment_controller::insert)),
                    )
            )
            .service(
                web::scope("/experience_comment_reply")
                    .service(
                        web::resource("/experience/{country}/{city}")
                            .route(web::get().to(experience_comment_reply_controller::find_all_by_experience))
                    )
                    .service(
                        web::resource("/comment/{email}/{timestamp}")
                            .route(web::post().to(experience_comment_reply_controller::insert))
                    )
            )
            .service(
                web::scope("/collection")
                    .service(
                        web::resource("")
                            .route(web::get().to(collection_controller::find_all))
                            .route(web::post().to(collection_controller::insert))
                    )
                    .service(
                        web::resource("/id/{name}")
                            .route(web::get().to(collection_controller::find_by_id))
                            .route(web::delete().to(collection_controller::delete)),
                    )
                    .service(
                        web::resource("/author/{email}")
                            .route(web::get().to(collection_controller::find_by_author))
                    )
            )
            .service(
                web::scope("/collection_experience")
                    .service(
                        web::resource("")
                            .route(web::post().to(collection_experience_controller::insert))
                    )
                    .service(
                        web::resource("/{collection}")
                            .route(web::get().to(collection_experience_controller::find_by_collection))
                    )
                    .service(
                        web::resource("/{collection}/{country}/{city}")
                            .route(web::post().to(collection_experience_controller::delete))
                    )
            )
            .service(
                web::scope("/experience_interest")
                    .service(
                        web::resource("")
                            .route(web::post().to(experience_interest_controller::insert))
                    )
                    .service(
                        web::resource("/{country}/{city}")
                            .route(web::get().to(experience_interest_controller::find_by_experience))
                    )
            )
            .service(
                web::scope("/interest")
                    .service(
                        web::resource("")
                            .route(web::get().to(interest_controller::find_all))
                    )
            )
            .service(
                web::scope("/country")
                    .service(
                        web::resource("")
                            .route(web::get().to(country_controller::find_all))
                    )
            ),
    );
}
