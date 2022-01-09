table! {
    accommodation_service (name) {
        name -> Text,
        description -> Text,
        image -> Binary,
    }
}

table! {
    chat_message (sender, recipient, timestamp) {
        sender -> Text,
        recipient -> Text,
        timestamp -> Nullable<Integer>,
        content -> Text,
    }
}

table! {
    collection (name) {
        author -> Text,
        name -> Text,
        description -> Text,
        image -> Binary,
        season -> Text,
    }
}

table! {
    collection_experience (name, country, city) {
        name -> Text,
        country -> Text,
        city -> Text,
    }
}

table! {
    country (name) {
        name -> Text,
    }
}

table! {
    country_city (country, city) {
        country -> Text,
        city -> Text,
    }
}

table! {
    experience (country, city) {
        author -> Text,
        name -> Text,
        description -> Text,
        country -> Text,
        city -> Text,
        main_image -> Binary,
        season -> Text,
        what_to_know -> Nullable<Text>,
        visa -> Nullable<Integer>,
        pp_validity -> Nullable<Text>,
        pp_pages -> Nullable<Integer>,
        vaccination -> Nullable<Text>,
        currency_entry -> Nullable<Text>,
        currency_exit -> Nullable<Text>,
        budget -> Nullable<Integer>,
        transport -> Nullable<Text>,
        additional_info -> Nullable<Text>,
    }
}

table! {
    experience_comment (author, timestamp) {
        author -> Text,
        country -> Text,
        city -> Text,
        text -> Text,
        timestamp -> BigInt,
    }
}

table! {
    experience_comment_reply (reply_author, reply_time) {
        comment_author -> Text,
        comment_time -> BigInt,
        reply_author -> Text,
        reply_time -> BigInt,
        reply_text -> Text,
    }
}

table! {
    experience_contributor (country, city, user) {
        country -> Text,
        city -> Text,
        user -> Text,
    }
}

table! {
    experience_image (id) {
        id -> Integer,
        country -> Text,
        city -> Text,
        image -> Binary,
    }
}

table! {
    experience_interest (country, city, interest) {
        country -> Text,
        city -> Text,
        interest -> Text,
    }
}

table! {
    experience_like (user, country, city) {
        user -> Text,
        country -> Text,
        city -> Text,
    }
}

table! {
    experience_video (id) {
        id -> Integer,
        country -> Text,
        city -> Text,
        video_url -> Text,
    }
}

table! {
    interest (name) {
        name -> Text,
    }
}

table! {
    season (name) {
        name -> Text,
    }
}

table! {
    user (email) {
        email -> Text,
        username -> Text,
        password_hash -> Text,
        first_name -> Text,
        last_name -> Text,
        profile_pic -> Nullable<Binary>,
        login_session -> Nullable<Text>,
    }
}

joinable!(collection -> user (author));
joinable!(collection_experience -> collection (name));
joinable!(country_city -> country (country));
joinable!(experience -> season (season));
joinable!(experience -> user (author));
joinable!(experience_comment -> user (author));
joinable!(experience_comment_reply -> user (reply_author));
joinable!(experience_interest -> interest (interest));
joinable!(experience_like -> user (user));

allow_tables_to_appear_in_same_query!(
    accommodation_service,
    chat_message,
    collection,
    collection_experience,
    country,
    country_city,
    experience,
    experience_comment,
    experience_comment_reply,
    experience_contributor,
    experience_image,
    experience_interest,
    experience_like,
    experience_video,
    interest,
    season,
    user,
);
