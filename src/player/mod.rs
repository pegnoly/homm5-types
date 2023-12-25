use serde::{Serialize, Deserialize};
use crate::{common::{Trigger, FileRef}, Homm5Type};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TavernFilter {
    #[serde(rename = "BannedHeroesRaces")]
    pub banned_heroes_races: Vec<String>,
    #[serde(rename = "BannedHeroes")]
    pub banned_heroes: Vec<String>,
    #[serde(rename = "AllowedHeroes")]
    pub allowed_heroes: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "Item")]
pub struct Player {
    #[serde(rename = "MainTown")]
    pub main_town: String,
    #[serde(rename = "MainHero")]
    pub main_hero: String,
    #[serde(rename = "ActivePlayer")]
    pub active_player: bool,
    #[serde(rename = "Team")]
    pub team: usize,
    #[serde(rename = "CanBeHumanPlayer")]
    pub can_be_human_player: bool,
    #[serde(rename = "CanBeComputerPlayer")]
    pub can_be_computer_player: bool,
    #[serde(rename = "Behaviour")]
    pub behaviour: String,
    #[serde(rename = "CaptureAbility")]
    pub capture_ability: u8,
    #[serde(rename = "StartHero")]
    pub start_hero: String,
    #[serde(rename = "HeroInTown")]
    pub hero_in_town: bool,
    #[serde(rename = "ReserveHeroes")]
    pub reserve_heroes: Vec<String>,
    #[serde(rename = "AddHeroTrigger")]
    pub add_hero_trigger: Trigger,
    #[serde(rename = "RemoveHeroTrigger")]
    pub remove_hero_trigger: Trigger,
    #[serde(rename = "VictoryMessageRef")]
    pub victory_message_ref: FileRef,
    #[serde(rename = "DefeatMessageRef")]
    pub defeat_message_ref: FileRef,
    #[serde(rename = "Race")]
    pub race: String,
    #[serde(rename = "Colour")]
    pub color: String,
    #[serde(rename = "CanBeDisabled")]
    pub can_be_disabled: bool,
    #[serde(rename = "Attractors")]
    pub attractors: Vec<String>,
    #[serde(rename = "TavernFilter")]
    pub tavern_filter: TavernFilter,
    #[serde(rename = "DenyFogOfWarForAllies")]
    pub deny_fog_of_war_for_allies: Vec<String>
}

impl Homm5Type for Player {
}