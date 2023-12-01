use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

// impl Related<super::fruit::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::Fruit.def()
//     }
// }

impl ActiveModelBehavior for ActiveModel {}
