#![allow(dead_code)]
pub struct Party {
    members: Vec<Member>,
}

pub struct Member {
    level: i32,
    xp: i32,
}

pub struct Encounter {
    monsters: Vec<Monster>,
}

pub struct Monster {
    xp: i32,
    challenge_rating: f32,
}

pub fn calculate_difficulty(party: &Party, encounter: &Encounter, by_challenge_rating: bool) -> (String, i32, i32) {
    let mut total_xp = 0;
    let mut total_monsters_xp = 0;
    let mut total_challenge_rating = 0.0;
    let mut total_monsters_challenge_rating = 0.0;
    let mut num_monsters = 0;

    for member in &party.members {
        total_xp += member.xp;
    }

    for monster in &encounter.monsters {
        num_monsters += 1;
        total_monsters_xp += monster.xp;
        total_monsters_challenge_rating += monster.challenge_rating;
    }

    if by_challenge_rating {
        total_challenge_rating = total_monsters_challenge_rating / num_monsters as f32;

        if total_challenge_rating <= 0.5 {
            return ("Easy".to_string(), 1, total_monsters_xp);
        } else if total_challenge_rating <= 1.5 {
            return ("Medium".to_string(), 2, total_monsters_xp);
        } else if total_challenge_rating <= 2.5 {
            return ("Hard".to_string(), 2, total_monsters_xp);
        } else if total_challenge_rating <= 3.5 {
            return ("Deadly".to_string(), 3, total_monsters_xp);
        } else {
            return ("Impossible".to_string(), 4, total_monsters_xp);
        }
    } else {
        let xp_thresholds = [25, 50, 75, 100,
                             150, 200, 300, 400,
                             500, 600, 800, 1000,
                             1100, 1250, 1400, 1600,
                             2000, 2100, 2400, 2800];

        let mut xp_threshold = xp_thresholds[(party.members[0].level -1) as usize];

        for member in &party.members[1..] {
            xp_threshold += xp_thresholds[(member.level -1) as usize];
        }

        let multiplier = match num_monsters {
            n if n >=15 => n as f32 /4.0,
            n if n >=11 => n as f32 /3.0,
            n if n >=6 => n as f32 /2.0,
            _ => num_monsters as f32,
        };

        let adjusted_xp = (total_monsters_xp as f32 * multiplier) as i32;

        if adjusted_xp < xp_threshold /4 {
            return ("Easy".to_string(), 1, adjusted_xp);
        } else if adjusted_xp < xp_threshold /2 {
            return ("Medium".to_string(), 2, adjusted_xp);
        } else if adjusted_xp < xp_threshold *2 {
            return ("Hard".to_string(), 2, adjusted_xp);
        } else if adjusted_xp < xp_threshold *4 {
            return ("Deadly".to_string(), 3, adjusted_xp);
        } else {
            return ("Impossible".to_string(), 4, adjusted_xp);
        }
    }
}

//(1) Basic Rules for Dungeons and Dragons (D&D) Fifth Edition (5e) - D&D Beyond. https://www.dndbeyond.com/sources/basic-rules/building-combat-encounters.
//(2) dnd 5e - How to determine CR of a group of enemies? - Role-playing .... https://rpg.stackexchange.com/questions/161797/how-to-determine-cr-of-a-group-of-enemies.
// (3) Encounter Builder for Dungeons & Dragons (D&D) Fifth Edition (5e). https://www.dndbeyond.com/encounter-builder.