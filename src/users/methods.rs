use crate::error::{VkApiError, VkError};
use crate::users::types::{
    Fields, FollowersResponse, NameCase, ReportType, User, UserGetFollowersOptions, UserGetOptions,
    UserResponse, UsersReportOptions,
};
use crate::{send_request, ParamGrid, VkApi};
use serde_json::Value;

const API: &str = "https://api.vk.com/method/users.";

pub async fn report(
    api: &VkApi,
    user_id: usize,
    report_type: ReportType,
    options: Option<UsersReportOptions>,
) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("user_id", user_id.to_string());

    match report_type {
        ReportType::Porn => params.insert_if_not_exists("type", "porn"),
        ReportType::Spam => params.insert_if_not_exists("type", "spam"),
        ReportType::Insult => params.insert_if_not_exists("type", "insult"),
        ReportType::Advertisement => params.insert_if_not_exists("type", "advert"),
    }

    match options {
        Some(options) => params.insert_if_not_exists("comment", options.comment),
        None => {}
    }

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}report", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn get(api: &VkApi, options: Option<UserGetOptions>) -> Result<UserResponse, VkApiError> {
    let api_key = if !api.flow_key.is_empty() {
        &api.flow_key
    } else if !api.group_key.is_empty() {
        &api.group_key
    } else if !api.service_key.is_empty() {
        &api.service_key
    } else {
        return Err(VkApiError::InternalError("There is no existing keys in your api object. Please set one of the keys before calling this function.".to_string()));
    };

    let mut params = ParamGrid::new();

    match options {
        Some(options) => {
            let mut user_ids = String::new();
            for user_id in &options.user_ids {
                match user_id {
                    User::Id(user_id) => user_ids.push_str(&user_id.to_string()),
                    User::ScreenName(screen_name) => user_ids.push_str(&screen_name),
                }
                user_ids.push_str(",");
            }

            if !options.user_ids.is_empty() {
                params.insert_if_not_exists("user_ids", user_ids);
            }

            let mut fields = String::new();
            for field in options.fields {
                match field {
                    //match all enums
                    Fields::Activities => fields.push_str("activities"),
                    Fields::About => fields.push_str("about"),
                    Fields::Blacklisted => fields.push_str("blacklisted"),
                    Fields::BlacklistedByMe => fields.push_str("blacklisted_by_me"),
                    Fields::Books => fields.push_str("books"),
                    Fields::Bdate => fields.push_str("bdate"),
                    Fields::CanBeInvitedGroup => fields.push_str("can_be_invited_group"),
                    Fields::CanPost => fields.push_str("can_post"),
                    Fields::CanSeeAllPosts => fields.push_str("can_see_all_posts"),
                    Fields::CanSeeAudio => fields.push_str("can_see_audio"),
                    Fields::CanSendFriendRequest => fields.push_str("can_send_friend_request"),
                    Fields::CanWritePrivateMessage => fields.push_str("can_write_private_message"),
                    Fields::Career => fields.push_str("career"),
                    Fields::CommonCount => fields.push_str("common_count"),
                    Fields::Connections => fields.push_str("connections"),
                    Fields::Contacts => fields.push_str("contacts"),
                    Fields::City => fields.push_str("city"),
                    Fields::Country => fields.push_str("country"),
                    Fields::CropPhoto => fields.push_str("crop_photo"),
                    Fields::Domain => fields.push_str("domain"),
                    Fields::Education => fields.push_str("education"),
                    Fields::Exports => fields.push_str("exports"),
                    Fields::FollowersCount => fields.push_str("followers_count"),
                    Fields::FriendStatus => fields.push_str("friend_status"),
                    Fields::HasPhoto => fields.push_str("has_photo"),
                    Fields::HasMobile => fields.push_str("has_mobile"),
                    Fields::HomeTown => fields.push_str("home_town"),
                    Fields::Photo100 => fields.push_str("photo_100"),
                    Fields::Photo200 => fields.push_str("photo_200"),
                    Fields::Photo200Orig => fields.push_str("photo_200_orig"),
                    Fields::Photo4000rig => fields.push_str("photo_400_orig"),
                    Fields::Photo50 => fields.push_str("photo_50"),
                    Fields::Sex => fields.push_str("sex"),
                    Fields::Site => fields.push_str("site"),
                    Fields::Schools => fields.push_str("schools"),
                    Fields::ScreenName => fields.push_str("screen_name"),
                    Fields::Status => fields.push_str("status"),
                    Fields::Verified => fields.push_str("verified"),
                    Fields::Games => fields.push_str("games"),
                    Fields::Interests => fields.push_str("interests"),
                    Fields::IsFavorite => fields.push_str("is_favorite"),
                    Fields::IsFriend => fields.push_str("is_friend"),
                    Fields::IsHiddenFromFeed => fields.push_str("is_hidden_from_feed"),
                    Fields::LastSeen => fields.push_str("last_seen"),
                    Fields::MaidenName => fields.push_str("maiden_name"),
                    Fields::Military => fields.push_str("military"),
                    Fields::Movies => fields.push_str("movies"),
                    Fields::Music => fields.push_str("music"),
                    Fields::Nickname => fields.push_str("nickname"),
                    Fields::Occupation => fields.push_str("occupation"),
                    Fields::Online => fields.push_str("online"),
                    Fields::Personal => fields.push_str("personal"),
                    Fields::PhotoId => fields.push_str("photo_id"),
                    Fields::PhotoMax => fields.push_str("photo_max"),
                    Fields::PhotoMaxOrig => fields.push_str("photo_max_orig"),
                    Fields::Quotes => fields.push_str("quotes"),
                    Fields::Relation => fields.push_str("relation"),
                    Fields::Relatives => fields.push_str("relatives"),
                    Fields::Timezone => fields.push_str("timezone"),
                    Fields::Tv => fields.push_str("tv"),
                    Fields::Universities => fields.push_str("universities"),
                }
                fields.push_str(",");
            }
            params.insert_if_not_exists("fields", fields);

            match options.name_case {
                NameCase::Nom => params.insert_if_not_exists("name_case", "nom"),
                NameCase::Gen => params.insert_if_not_exists("name_case", "gen"),
                NameCase::Dat => params.insert_if_not_exists("name_case", "dat"),
                NameCase::Acc => params.insert_if_not_exists("name_case", "acc"),
                NameCase::Ins => params.insert_if_not_exists("name_case", "ins"),
                NameCase::Abl => params.insert_if_not_exists("name_case", "abl"),
            }
        }
        None => {}
    }

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}get", API),
        api_key,
        api.v,
    )
    .await?;

    return if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        Err(VkApiError::VkError(error))
    } else {
        let json: Value = serde_json::from_str(&response_text)?;
        let data: UserResponse = serde_json::from_value(json.clone())?;
        Ok(data)
    };
}

pub async fn get_followers(
    api: &VkApi,
    user_id: usize,
    offset: usize,
    count: usize,
    options: Option<UserGetFollowersOptions>,
) -> Result<FollowersResponse, VkApiError> {
    let api_key = if !api.flow_key.is_empty() {
        &api.flow_key
    } else if !api.service_key.is_empty() {
        &api.service_key
    } else {
        return Err(VkApiError::InternalError("There is no existing keys in your api object. Please set one of the keys before calling this function.".to_string()));
    };

    let mut params = ParamGrid::new();

    params.insert_if_not_exists("user_id", user_id);
    params.insert_if_not_exists("offset", offset);
    params.insert_if_not_exists("count", count);

    match options {
        Some(options) => {
            let mut fields = String::new();
            for field in options.fields {
                match field {
                    //match all enums
                    Fields::Activities => fields.push_str("activities"),
                    Fields::About => fields.push_str("about"),
                    Fields::Blacklisted => fields.push_str("blacklisted"),
                    Fields::BlacklistedByMe => fields.push_str("blacklisted_by_me"),
                    Fields::Books => fields.push_str("books"),
                    Fields::Bdate => fields.push_str("bdate"),
                    Fields::CanBeInvitedGroup => fields.push_str("can_be_invited_group"),
                    Fields::CanPost => fields.push_str("can_post"),
                    Fields::CanSeeAllPosts => fields.push_str("can_see_all_posts"),
                    Fields::CanSeeAudio => fields.push_str("can_see_audio"),
                    Fields::CanSendFriendRequest => fields.push_str("can_send_friend_request"),
                    Fields::CanWritePrivateMessage => fields.push_str("can_write_private_message"),
                    Fields::Career => fields.push_str("career"),
                    Fields::CommonCount => fields.push_str("common_count"),
                    Fields::Connections => fields.push_str("connections"),
                    Fields::Contacts => fields.push_str("contacts"),
                    Fields::City => fields.push_str("city"),
                    Fields::Country => fields.push_str("country"),
                    Fields::CropPhoto => fields.push_str("crop_photo"),
                    Fields::Domain => fields.push_str("domain"),
                    Fields::Education => fields.push_str("education"),
                    Fields::Exports => fields.push_str("exports"),
                    Fields::FollowersCount => fields.push_str("followers_count"),
                    Fields::FriendStatus => fields.push_str("friend_status"),
                    Fields::HasPhoto => fields.push_str("has_photo"),
                    Fields::HasMobile => fields.push_str("has_mobile"),
                    Fields::HomeTown => fields.push_str("home_town"),
                    Fields::Photo100 => fields.push_str("photo_100"),
                    Fields::Photo200 => fields.push_str("photo_200"),
                    Fields::Photo200Orig => fields.push_str("photo_200_orig"),
                    Fields::Photo4000rig => fields.push_str("photo_400_orig"),
                    Fields::Photo50 => fields.push_str("photo_50"),
                    Fields::Sex => fields.push_str("sex"),
                    Fields::Site => fields.push_str("site"),
                    Fields::Schools => fields.push_str("schools"),
                    Fields::ScreenName => fields.push_str("screen_name"),
                    Fields::Status => fields.push_str("status"),
                    Fields::Verified => fields.push_str("verified"),
                    Fields::Games => fields.push_str("games"),
                    Fields::Interests => fields.push_str("interests"),
                    Fields::IsFavorite => fields.push_str("is_favorite"),
                    Fields::IsFriend => fields.push_str("is_friend"),
                    Fields::IsHiddenFromFeed => fields.push_str("is_hidden_from_feed"),
                    Fields::LastSeen => fields.push_str("last_seen"),
                    Fields::MaidenName => fields.push_str("maiden_name"),
                    Fields::Military => fields.push_str("military"),
                    Fields::Movies => fields.push_str("movies"),
                    Fields::Music => fields.push_str("music"),
                    Fields::Nickname => fields.push_str("nickname"),
                    Fields::Occupation => fields.push_str("occupation"),
                    Fields::Online => fields.push_str("online"),
                    Fields::Personal => fields.push_str("personal"),
                    Fields::PhotoId => fields.push_str("photo_id"),
                    Fields::PhotoMax => fields.push_str("photo_max"),
                    Fields::PhotoMaxOrig => fields.push_str("photo_max_orig"),
                    Fields::Quotes => fields.push_str("quotes"),
                    Fields::Relation => fields.push_str("relation"),
                    Fields::Relatives => fields.push_str("relatives"),
                    Fields::Timezone => fields.push_str("timezone"),
                    Fields::Tv => fields.push_str("tv"),
                    Fields::Universities => fields.push_str("universities"),
                }
                fields.push_str(",");
            }
            params.insert_if_not_exists("fields", fields);

            match options.name_case {
                NameCase::Nom => params.insert_if_not_exists("name_case", "nom"),
                NameCase::Gen => params.insert_if_not_exists("name_case", "gen"),
                NameCase::Dat => params.insert_if_not_exists("name_case", "dat"),
                NameCase::Acc => params.insert_if_not_exists("name_case", "acc"),
                NameCase::Ins => params.insert_if_not_exists("name_case", "ins"),
                NameCase::Abl => params.insert_if_not_exists("name_case", "abl"),
            }
        }
        None => {}
    }

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}getFollowers", API),
        api_key,
        api.v,
    )
    .await?;

    return if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        Err(VkApiError::VkError(error))
    } else {
        let json: Value = serde_json::from_str(&response_text)?;
        let data: FollowersResponse = serde_json::from_value(json["response"].clone())?;
        Ok(data)
    };
}
