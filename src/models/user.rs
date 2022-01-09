use crate::{
    config::db::Connection,
    constants,
    models::{user_token::UserToken},
    schema::user::{self, dsl::*},
};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "user"]
pub struct User {
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub profile_pic: Option<Vec<u8>>,
    pub login_session: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserDTO {
    pub email: String,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub profile_pic_base64: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "user"]
pub struct LoginInfoDTO {
    pub email: String,
    pub login_session: String,
}

impl From<UserDTO> for User {
    fn from(u: UserDTO) -> Self {
        Self {
            email: u.email,
            username: u.username,
            password_hash: hash(&u.password, DEFAULT_COST).unwrap(),
            first_name: u.first_name,
            last_name: u.last_name,
            login_session: None,
            profile_pic: if let Some(pic) = u.profile_pic_base64 { base64::decode(pic).ok() } else { None },
        }
    }
}

impl From<User> for UserDTO {
    fn from(u: User) -> Self {
        Self {
            email: u.email,
            username: u.username,
            password: "".to_owned(),
            first_name: u.first_name,
            last_name: u.last_name,
            profile_pic_base64: if let Some(pic) = u.profile_pic { Some(base64::encode(pic)) } else { None },
        }
    }
}

impl User {
    pub fn signup(new_user: User, conn: &Connection) -> Result<String, String> {
        if Self::find_user_by_username(&new_user.username, conn).is_err()
        && Self::find_user_by_email(&new_user.email, conn).is_err() {
            diesel::insert_into(user).values(&new_user).execute(conn);
            Ok(constants::MESSAGE_SIGNUP_SUCCESS.to_string())
        } else {
            Err(format!("Email or username is already used"))
        }
    }

    pub fn login(login: LoginDTO, conn: &Connection) -> Option<LoginInfoDTO> {
        if let Ok(user_to_verify) = user
            .filter(username.eq(&login.username_or_email))
            .or_filter(email.eq(&login.username_or_email))
            .get_result::<User>(conn)
        {
            if !user_to_verify.password_hash.is_empty()
                && verify(&login.password, &user_to_verify.password_hash).unwrap()
            {
                let login_session_str = User::generate_login_session();
                if User::update_login_session_to_db(
                    &user_to_verify.email,
                    &login_session_str,
                    conn,
                ) {
                    return Some(LoginInfoDTO {
                        email: user_to_verify.email,
                        login_session: login_session_str,
                    });
                }
            } else {
                return Some(LoginInfoDTO {
                    email: user_to_verify.email,
                    login_session: String::new(),
                });
            }
        }

        None
    }

    pub fn logout(em: &str, conn: &Connection) {
        if let Ok(found) = user.find(em).get_result::<User>(conn) {
            Self::update_login_session_to_db(&found.email, "", conn);
        }
    }

    pub fn is_valid_login_session(user_token: &UserToken, conn: &Connection) -> bool {
        user
            .filter(email.eq(&user_token.email))
            .filter(login_session.eq(&user_token.login_session))
            .get_result::<User>(conn)
            .is_ok()
    }

    pub fn find_user_by_email_or_username(em_or_un: &str, conn: &Connection) -> QueryResult<User> {
        Self::find_user_by_username(em_or_un, conn).or(
            Self::find_user_by_email(em_or_un, conn)
        )
    }

    pub fn find_user_by_email(em: &str, conn: &Connection) -> QueryResult<User> {
        user.filter(email.eq(em)).get_result::<User>(conn)
    }

    pub fn find_user_by_username(un: &str, conn: &Connection) -> QueryResult<User> {
        user.filter(username.eq(un)).get_result::<User>(conn)
    }

    pub fn generate_login_session() -> String {
        Uuid::new_v4().to_simple().to_string()
    }

    pub fn update_login_session_to_db(
        un: &str,
        login_session_str: &str,
        conn: &Connection,
    ) -> bool {
        if let Ok(found) = User::find_user_by_email(un, conn) {
            diesel::update(user.find(found.email))
                .set(login_session.eq(login_session_str.to_string()))
                .execute(conn)
                .is_ok()
        } else {
            false
        }
    }
}
