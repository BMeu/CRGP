//! Representations of Twitter users.

/// Users can be anyone or anything.
///
/// Users tweet, follow, create lists, have a home timelin, can be mentioned, and can be looked up
/// in bulk.
///
/// This struct's field correspond directly to the fields of the same name in the Twitter API.
///
/// # See Also
/// https://dev.twitter.com/overview/api/users
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    /// Integer representation of the unique identifier for this user.
    pub id: u64,

    /// The screen name, handle, or alias that this user identifies themselves with. Screen names
    /// are unique, but subject to change.
    pub screen_name: String
}
