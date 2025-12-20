use sonettobuf;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct UserSimpleProperty {
    pub user_id: i64,
    pub property_id: i32,
    pub property_value: String,
}

impl From<UserSimpleProperty> for sonettobuf::SimpleProperty {
    fn from(p: UserSimpleProperty) -> Self {
        sonettobuf::SimpleProperty {
            id: Some(p.property_id),
            property: Some(p.property_value),
        }
    }
}
