use deathframe::physics::CollisionTag as CTag;

#[derive(PartialEq, Eq, Hash, Clone, Deserialize)]
pub enum CollisionTag {
    Player,
    Solid,
    Enemy,
}

impl CTag for CollisionTag {
    fn collides_with(&self, other: &Self) -> bool {
        match (self, other) {
            (CollisionTag::Player, CollisionTag::Solid) => true,
            (CollisionTag::Player, CollisionTag::Enemy) => true,
            (CollisionTag::Enemy, CollisionTag::Player) => true,
            (CollisionTag::Enemy, CollisionTag::Solid) => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Deserialize)]
pub enum SolidTag {
    Player,
    Solid,
    Enemy,
}

impl CTag for SolidTag {
    fn collides_with(&self, other: &Self) -> bool {
        match (self, other) {
            (SolidTag::Player, SolidTag::Solid) => true,
            (SolidTag::Enemy, SolidTag::Solid) => true,
            (SolidTag::Player, SolidTag::Enemy) => false,
            (SolidTag::Enemy, SolidTag::Player) => false,
            _ => false,
        }
    }
}
