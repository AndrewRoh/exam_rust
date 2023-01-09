use tiberius;

pub struct CharacterTable {
    pub char_guid: tiberius::Uuid,
    pub player_guid: tiberius::Uuid,
    pub char_name: ::prost::alloc::string::String,
    pub gender: u8,
    pub char_level: i16,
    pub gold: i32,
    pub silver: i32,
    pub diamond: i32,
    pub free_currency: f64,
    pub premium_currency: f64,
    pub max_health: f32,
    pub health: f32,
    pub health_regen_rate: f32,
    pub max_mana: f32,
    pub mana: f32,
    pub mana_regen_rate: f32,
    pub description: ::prost::alloc::string::String,
    pub is_admin: bool,
    pub create_dt: tiberius::time::DateTime2,
}
