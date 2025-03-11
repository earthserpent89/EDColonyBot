struct ConstructionSite {
    id: String,
    name: String,
    location: String,
    status: String,
    created_at: String,
    updated_at: String,
}

struct User {
    id: String,
    username: String,
    joined_at: String,
}

struct ConstructionUpdate {
    site_id: String,
    user_id: String,
    update_message: String,
    timestamp: String,
}