create table user (
    email           text    not null    primary key,
    username        text    not null    unique,
    password_hash   text    not null,
    first_name      text    not null,
    last_name       text    not null,
    profile_pic     blob,
    login_session   text
);

create table interest (
    name            text    not null    primary key
);

create table country (
    name            text    not null    primary key
);

create table country_city (
    country         text    not null,
    city            text    not null,

    primary key (country, city),
    foreign key (country) references country(name) on delete restrict
);

create table accommodation_service (
    name            text    not null    primary key,
    description     text    not null,
    image           blob    not null    unique
);

create table season (
    name            text    not null    primary key
);

create table experience (
    author          text    not null,
    name            text    not null,
    description     text    not null,
    country         text    not null,
    city            text    not null,
    main_image      blob    not null,
    season          text    not null,
    what_to_know    text,

    /* Documentation */
    visa            integer,
    pp_validity     text,
    pp_pages        integer,
    vaccination     text,
    currency_entry  text,
    currency_exit   text,

    budget          integer,
    transport       text,
    additional_info text,

    primary key (country, city),
    foreign key (author) references user(email) on delete cascade,
    foreign key (season) references season(name) on delete cascade,
    foreign key (country, city) references country_city(country, city) on delete restrict,
    check (visa in (0, 1))
);

create table experience_contributor (
    country         text    not null,
    city            text    not null,
    user            text    not null,

    primary key (country, city, user),
    foreign key (country, city) references experience(country, city) on delete cascade
);

create table experience_image (
    id              integer not null    primary key,
    country         text    not null,
    city            text    not null,
    image           blob    not null,

    foreign key (country, city) references experience(country, city) on delete cascade
);

create table experience_video (
    id              integer not null    primary key,
    country         text    not null,
    city            text    not null,
    video_url       text    not null,

    foreign key (country, city) references experience(country, city) on delete cascade
);

create table experience_interest (
    country         text    not null,
    city            text    not null,
    interest        text    not null,

    primary key (country, city, interest),
    foreign key (interest) references interest(name) on delete restrict,
    foreign key (country, city) references experience(country, city) on delete cascade
);

create table experience_comment (
    author          text    not null,
    country         text    not null,
    city            text    not null,
    text            text    not null,
    timestamp       bigint  not null    default (strftime('%s', 'now')),

    primary key (author, timestamp),
    foreign key (author) references user(email) on delete cascade,
    foreign key (country, city) references experience(country, city) on delete cascade
);

create table experience_comment_reply (
    comment_author  text    not null,
    comment_time    bigint  not null,
    reply_author    text    not null,
    reply_time      bigint  not null    default (strftime('%s', 'now')),
    reply_text      text    not null,

    primary key (reply_author, reply_time),
    foreign key (reply_author) references user(email) on delete cascade,
    foreign key (comment_author, comment_time)
        references experience_comment(author, timestamp) on delete cascade
);

create table experience_like (
    user            text    not null,
    country         text    not null,
    city            text    not null,

    primary key (user, country, city),
    foreign key (user) references user(email) on delete cascade,
    foreign key (country, city) references experience(country, city) on delete cascade
);

create table collection (
    author          text    not null,
    name            text    not null    primary key,
    description     text    not null,
    image           blob    not null,
    season          text    not null,

    foreign key (author) references user(email) on delete cascade
);

create table collection_experience (
    name            text    not null,
    country         text    not null,
    city            text    not null,

    primary key (name, city, country),
    foreign key (name) references collection(name) on delete cascade,
    foreign key (country, city) references experience(country, city) on delete cascade
);

create table chat_message (
    sender          text    not null,
    recipient       text    not null,
    timestamp       integer default (strftime('%s', 'now')),
    content         text    not null,

    primary key (sender, recipient, timestamp),
    foreign key (sender) references user(email) on delete cascade,
    foreign key (recipient) references user(email) on delete cascade
);
