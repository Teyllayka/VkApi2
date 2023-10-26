use crate::notes::types::{EditOptions, NewOptionsList, Options, Privacy};
use crate::{error::VkApiError, error::VkError, param_grid::ParamGrid, send_request, VkApi};

const API: &str = "https://api.vk.com/method/notes.";

pub async fn delete(api: &VkApi, note_id: String) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("note_id", note_id);
    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}delete", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn edit(
    api: &VkApi,
    note_id: String,
    title: String,
    text: String,
    options: Option<EditOptions>,
) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("note_id", note_id);
    params.insert_if_not_exists("title", title);
    params.insert_if_not_exists("text", text);

    match options {
        Some(option) => {
            match option.options {
                Options::NewOptions(privacy_view, vec_view, privacy_comment, vec_comment) => {
                    if api.v < 5.30 {
                        //throw internal error that version is too low to use this options
                        return Err(VkApiError::InternalError(
                            "Version is too low to use this options".to_string(),
                        ));
                    } else {
                        let mut privacy_view_string = String::new();
                        let mut privacy_comment_string = String::new();

                        match privacy_view {
                            Privacy::All => privacy_view_string.push_str("all"),
                            Privacy::Friends => privacy_view_string.push_str("friends"),
                            Privacy::FriendsOfFriends => {
                                privacy_view_string.push_str("friends_of_friends")
                            }
                            Privacy::OnlyMe => privacy_view_string.push_str("only_me"),
                        }

                        for i in vec_view {
                            match i {
                                NewOptionsList::List(id) => {
                                    privacy_view_string.push_str(&format!(",list{}", id))
                                }
                                NewOptionsList::User(id) => {
                                    privacy_view_string.push_str(&format!(",{}", id))
                                }
                                NewOptionsList::ListRestricted(id) => {
                                    privacy_view_string.push_str(&format!(",-list{}", id))
                                }
                                NewOptionsList::UserRestricted(id) => {
                                    privacy_view_string.push_str(&format!(",-{}", id))
                                }
                            }
                        }

                        match privacy_comment {
                            Privacy::All => privacy_comment_string.push_str("all"),
                            Privacy::Friends => privacy_comment_string.push_str("friends"),
                            Privacy::FriendsOfFriends => {
                                privacy_comment_string.push_str("friends_of_friends ")
                            }
                            Privacy::OnlyMe => privacy_comment_string.push_str("only_me"),
                        }

                        for i in vec_comment {
                            match i {
                                NewOptionsList::List(id) => {
                                    privacy_view_string.push_str(&format!(",list{}", id))
                                }
                                NewOptionsList::User(id) => {
                                    privacy_view_string.push_str(&format!(",{}", id))
                                }
                                NewOptionsList::ListRestricted(id) => {
                                    privacy_view_string.push_str(&format!(",-list{}", id))
                                }
                                NewOptionsList::UserRestricted(id) => {
                                    privacy_view_string.push_str(&format!(",-{}", id))
                                }
                            }
                        }
                    }
                }
                Options::OldOptions(privacy, comment_privacy) => {
                    if api.v > 5.30 {
                        //throw internal error that version is too low to use this options
                        return Err(VkApiError::InternalError(
                            "Version is too high to use this options".to_string(),
                        ));
                    } else {
                        match privacy {
                            Privacy::All => params.insert_if_not_exists("privacy", "0"),
                            Privacy::Friends => params.insert_if_not_exists("privacy", "1"),
                            Privacy::FriendsOfFriends => {
                                params.insert_if_not_exists("privacy", "2")
                            }
                            Privacy::OnlyMe => params.insert_if_not_exists("privacy", "3"),
                        }

                        match comment_privacy {
                            Privacy::All => params.insert_if_not_exists("comment_privacy", "0"),
                            Privacy::Friends => params.insert_if_not_exists("comment_privacy", "1"),
                            Privacy::FriendsOfFriends => {
                                params.insert_if_not_exists("comment_privacy", "2")
                            }
                            Privacy::OnlyMe => params.insert_if_not_exists("comment_privacy", "3"),
                        }
                    }
                }
            }
        }
        None => (),
    }

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}edit", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn delete_comment(
    api: &VkApi,
    comment_id: usize,
    owner_id: i64,
) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("comment_id", comment_id.to_string());

    params.insert_if_not_exists("owner_id", owner_id);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}deleteComment", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn restore_comment(
    api: &VkApi,
    comment_id: usize,
    owner_id: i64,
) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("comment_id", comment_id.to_string());

    params.insert_if_not_exists("owner_id", owner_id);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}restoreComment", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}
