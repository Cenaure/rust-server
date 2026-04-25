use utoipa::OpenApi;
use crate::{handlers, models, jikan_integration};

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::auth_handler::sign_in,
        handlers::auth_handler::sign_up,
        handlers::auth_handler::logout,
        handlers::users_handler::list_users,
        handlers::users_handler::add_user,
        handlers::users_handler::get_user,
        handlers::users_handler::patch_user,
        handlers::users_handler::delete_user,
        handlers::groups_handler::list_groups,
        handlers::groups_handler::add_group,
        handlers::groups_handler::get_group,
        handlers::groups_handler::patch_group,
        handlers::groups_handler::delete_group,
        handlers::anime_handler::get_top,
        handlers::anime_handler::get_random,
    ),
    components(schemas(
        models::UserDTO,
        models::UserSignIn,
        models::UserSignUp,
        models::UserCreate,
        models::UserUpdate,
        models::Group,
        models::GroupDTO,
        models::GroupCreate,
        models::GroupUpdate,
        models::TopAnimeParams,
        jikan_integration::common::enums::anime::AnimeType,
        jikan_integration::common::enums::anime::AnimeFilter,
        jikan_integration::common::enums::anime::AnimeRating,
        jikan_integration::common::structs::common::Pagination,
        jikan_integration::common::structs::common::PaginationItems,
        jikan_integration::common::structs::common::CommonMalResponse,
        jikan_integration::common::structs::anime::AnimeStruct,
        jikan_integration::common::structs::anime::AnimeImages,
        jikan_integration::common::structs::anime::WebpImage,
        jikan_integration::common::structs::anime::AnimeTrailer,
        jikan_integration::common::structs::anime::AnimeTitles,
        jikan_integration::common::structs::top::AnimeTopJikanResponse,
        jikan_integration::common::structs::random::AnimeRandomJikanResponse,
    )),
    tags(
            (name = "Anidream Server", description = "Anidream Server for Angular project.")
    ),
)]
pub struct ApiDoc;