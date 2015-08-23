//! the `life` module of the Leafline oppositional strategy game engine

use space::{Locale, Pinfield};
use identity::{Team, JobDescription, Agent};
use motion::{PONY_MOVEMENT_TABLE, FIGUREHEAD_MOVEMENT_TABLE};


/// represents the movement of a figurine
#[derive(Eq,PartialEq,Debug,Copy,Clone)]
pub struct Patch {
    pub star: Agent,
    pub whence: Locale,
    pub whither: Locale
}


/// represents the outcome of a team's turn with a `patch` governing
/// the figurine moved, the state of the world after the turn (`tree`),
/// and whether an opposing figurine was stunned and put in the hospital,
/// and if so, which one
#[derive(Eq,PartialEq,Debug,Copy,Clone)]
pub struct Commit {
    pub patch: Patch,
    pub tree: WorldState,
    pub hospitalization: Option<Agent>
}


#[derive(Eq,PartialEq,Debug,Copy,Clone)]
pub struct WorldState {
    to_move: Team,

    orange_servants: Pinfield,
    orange_ponies: Pinfield,
    orange_scholars: Pinfield,
    orange_cops: Pinfield,
    orange_princesses: Pinfield,
    orange_figurehead: Pinfield,

    blue_servants: Pinfield,
    blue_ponies: Pinfield,
    blue_scholars: Pinfield,
    blue_cops: Pinfield,
    blue_princesses: Pinfield,
    blue_figurehead: Pinfield,
}

impl WorldState {
    pub fn new() -> Self {
        let mut orange_servant_locales = Vec::new();
        let mut blue_servant_locales = Vec::new();
        for f in 0..8 {
            orange_servant_locales.push(Locale { rank: 1, file: f });
            blue_servant_locales.push(Locale { rank: 6, file: f });
        }
        WorldState {
            to_move: Team::Orange,

            orange_servants: Pinfield::init(&orange_servant_locales),
            orange_ponies: Pinfield::init(
                &vec![Locale { rank: 0, file: 1 },
                      Locale { rank: 0, file: 6 }]
            ),
            orange_scholars: Pinfield::init(
                &vec![Locale { rank: 0, file: 2 },
                      Locale { rank: 0, file: 5 }]
            ),
            orange_cops: Pinfield::init(
                &vec![Locale { rank: 0, file: 0 },
                      Locale { rank: 0, file: 7 }]
            ),
            orange_princesses: Pinfield::init(
                &vec![Locale { rank: 0, file: 3 }]),
            orange_figurehead: Pinfield::init(
                &vec![Locale { rank: 0, file: 4 }]),
            blue_servants: Pinfield::init(&blue_servant_locales),
            blue_ponies: Pinfield::init(
                &vec![Locale { rank: 7, file: 1 },
                      Locale { rank: 7, file: 6 }]
            ),
            blue_scholars: Pinfield::init(
                &vec![Locale { rank: 7, file: 2 },
                      Locale { rank: 7, file: 5 }]
            ),
            blue_cops: Pinfield::init(
                &vec![Locale { rank: 7, file: 0 },
                      Locale { rank: 7, file: 7 }]
            ),
            blue_princesses: Pinfield::init(
                &vec![Locale { rank: 7, file: 3 }]),
            blue_figurehead: Pinfield::init(
                &vec![Locale { rank: 7, file: 4 }]),
        }
    }

    pub fn agent_to_pinfield_ref(&self, agent: Agent) -> &Pinfield {
        match agent {
            Agent{ team: Team::Orange,
                   job_description: JobDescription::Servant } =>
                &self.orange_servants,
            Agent{ team: Team::Orange,
                   job_description: JobDescription::Pony } =>
                &self.orange_ponies,
            Agent{ team: Team::Orange,
                   job_description: JobDescription::Scholar } =>
                &self.orange_scholars,
            Agent{ team: Team::Orange,
                   job_description: JobDescription::Cop } =>
                &self.orange_cops,
            Agent{ team: Team::Orange,
                   job_description: JobDescription::Princess } =>
                &self.orange_princesses,
            Agent{ team: Team::Orange,
                   job_description: JobDescription::Figurehead } =>
                &self.orange_figurehead,
            Agent{ team: Team::Blue,
                   job_description: JobDescription::Servant } =>
                &self.blue_servants,
            Agent{ team: Team::Blue,
                   job_description: JobDescription::Pony } =>
                &self.blue_ponies,
            Agent{ team: Team::Blue,
                   job_description: JobDescription::Scholar } =>
                &self.blue_scholars,
            Agent{ team: Team::Blue,
                   job_description: JobDescription::Cop } =>
                &self.blue_cops,
            Agent{ team: Team::Blue,
                   job_description: JobDescription::Princess } =>
                &self.blue_princesses,
            Agent{ team: Team::Blue,
                   job_description: JobDescription::Figurehead } =>
                &self.blue_figurehead,
        }
    }

    // XXX this code-duplication is hideous, but what can you do in
    // this language? My problem is exactly that I don't know
    pub fn agent_to_pinfield_mutref(&mut self, agent: Agent) -> &mut Pinfield {
        match agent {
            Agent{ team: Team::Orange,
                   job_description: JobDescription::Servant } =>
                &mut self.orange_servants,
            Agent{ team: Team::Orange,
                   job_description: JobDescription::Pony } =>
                &mut self.orange_ponies,
            Agent{ team: Team::Orange,
                   job_description: JobDescription::Scholar } =>
                &mut self.orange_scholars,
            Agent{ team: Team::Orange,
                   job_description: JobDescription::Cop } =>
                &mut self.orange_cops,
            Agent{ team: Team::Orange,
                   job_description: JobDescription::Princess } =>
                &mut self.orange_princesses,
            Agent{ team: Team::Orange,
                   job_description: JobDescription::Figurehead } =>
                &mut self.orange_figurehead,
            Agent{ team: Team::Blue,
                   job_description: JobDescription::Servant } =>
                &mut self.blue_servants,
            Agent{ team: Team::Blue,
                   job_description: JobDescription::Pony } =>
                &mut self.blue_ponies,
            Agent{ team: Team::Blue,
                   job_description: JobDescription::Scholar } =>
                &mut self.blue_scholars,
            Agent{ team: Team::Blue,
                   job_description: JobDescription::Cop } =>
                &mut self.blue_cops,
            Agent{ team: Team::Blue,
                   job_description: JobDescription::Princess } =>
                &mut self.blue_princesses,
            Agent{ team: Team::Blue,
                   job_description: JobDescription::Figurehead } =>
                &mut self.blue_figurehead,
        }
    }

    pub fn except_replaced_subboard(&self, for_whom: Agent,
                                    subboard: Pinfield) -> Self {
        let mut resultant_state = self.clone();
        resultant_state.agent_to_pinfield_mutref(for_whom).0 = subboard.0;
        resultant_state
    }

    pub fn occupied_by(&self, team: Team) -> Pinfield {
        match team {
            Team::Orange => self.orange_servants.union(
                self.orange_ponies).union(
                    self.orange_scholars).union(self.orange_cops).union(
                        self.orange_princesses).union(self.orange_figurehead),
            Team::Blue => self.blue_servants.union(self.blue_ponies).union(
                    self.blue_scholars).union(self.blue_cops).union(
                            self.blue_princesses).union(self.blue_figurehead)
        }
    }

    pub fn occupied(&self) -> Pinfield {
        self.occupied_by(Team::Orange).union(self.occupied_by(Team::Blue))
    }

    pub fn unoccupied(&self) -> Pinfield {
        self.occupied().invert()
    }

    pub fn occupying_agent(&self, at: Locale) -> Option<Agent> {
        for team in Team::league().into_iter() {
            for agent in Agent::dramatis_personae(team).into_iter() {
                if self.agent_to_pinfield_ref(agent).query(at) {
                    return Some(agent)
                }
            }
        }
        None
    }

    pub fn apply(&self, patch: Patch) -> Option<Commit> {
        // subboard of moving figurine
        let backstory = self.agent_to_pinfield_ref(patch.star);
        // subboard of moving figurine after move
        let derived_subboard = backstory.transit(patch.whence, patch.whither);
        // insert subboard into post-patch world-model
        let mut tree = self.except_replaced_subboard(
            patch.star, derived_subboard
        );

        // was anyone stunned?
        let hospitalization = self.occupying_agent(patch.whither);
        if let Some(stunned) = hospitalization {
            if stunned.team == patch.star.team {
                panic!("{:?} tried to stun friendly figurine \
                        {:?} at {:?}.\
                        This shouldn't happen!",
                       patch.star, hospitalization, patch.whither);
            }

            // if someone was stunned, put her or him in the hospital
            let further_derived_subboard = tree.agent_to_pinfield_ref(
                stunned).quench(patch.whither);
            tree = tree.except_replaced_subboard(
                stunned, further_derived_subboard
            );
        }
        // XXX TODO: actually return None if this would result in
        // moving team being in "check"
        Some(Commit { patch: patch, tree: tree,
                      hospitalization: hospitalization })
    }

    /// generate possible commits for servants of the given team
    pub fn servant_lookahead(&self, team: Team) -> Vec<Commit> {
        let initial_rank;
        let standard_offset;
        let boost_offset;
        let stun_offsets;
        match team {
            Team::Orange => {
                initial_rank = 1;
                standard_offset = (1, 0);
                boost_offset = (2, 0);
                stun_offsets = [(1, -1), (1, 1)];
            },
            Team::Blue => {
                initial_rank = 6;
                standard_offset = (-1, 0);
                boost_offset = (-2, 0);
                stun_offsets = [(-1, -1), (-1, 1)];
            }
        }
        let mut premonitions = Vec::new();
        let servant_agent = Agent {
            team: team, job_description: JobDescription::Servant };
        let positional_chart: &Pinfield = self.agent_to_pinfield_ref(
            servant_agent);
        for start_locale in positional_chart.to_locales().into_iter() {
            // can move one locale if he's not blocked
            let std_destination_maybe = start_locale.displace(standard_offset);
            if let Some(destination_locale) = std_destination_maybe {
                if self.unoccupied().query(destination_locale) {
                    let premonition_maybe = self.apply(
                        Patch {
                            star: servant_agent,
                            whence: start_locale,
                            whither: destination_locale
                        }
                    );
                    if let Some(premonition) = premonition_maybe {
                        premonitions.push(premonition);
                    }
                }
            }

            // can move two locales if he hasn't previously moved
            if start_locale.rank == initial_rank {
                // safe to unwrap because we know that we're at the
                // initial rank
                let boost_destination = start_locale.displace(
                    boost_offset).unwrap();
                let standard_destination = start_locale.displace(
                    standard_offset).unwrap();
                if (self.unoccupied().query(boost_destination) &&
                    self.unoccupied().query(standard_destination)) {
                    let premonition_maybe = self.apply(
                        Patch {
                            star: servant_agent,
                            whence: start_locale,
                            whither: boost_destination
                        }
                    );
                    if let Some(premonition) = premonition_maybe {
                        premonitions.push(premonition);
                    }
                }
            }
            // TODO can stun diagonally
        }
        premonitions
    }

    /// generate possible commits for ponies of the given team
    pub fn pony_lookahead(&self, team: Team) -> Vec<Commit> {
        let mut premonitions = Vec::new();
        let pony_agent = Agent {
            team: team, job_description: JobDescription::Pony };
        let positional_chart: &Pinfield = self.agent_to_pinfield_ref(
            pony_agent);
        for start_locale in positional_chart.to_locales().into_iter() {
            let destinations = self.occupied_by(team).invert().intersection(
                Pinfield(PONY_MOVEMENT_TABLE[
                    start_locale.pindex() as usize])).to_locales();
            for destination in destinations.into_iter() {
                let premonition_maybe = self.apply(
                    Patch {
                        star: pony_agent,
                        whence: start_locale,
                        whither: destination
                    }
                );
                if let Some(premonition) = premonition_maybe {
                    premonitions.push(premonition);
                }
            }
        }
        premonitions
    }

    pub fn lookahead(&self) -> Vec<Self> {
        let premonitions = Vec::new();
        let moving_team = self.to_move;


        // TODO work in progress
        premonitions
    }

    pub fn display(&self) {
        println!("  a b c d e f g h");
        for rank in 0..8 {
            print!("{} ", rank+1);
            for file in 0..8 {
                let locale = Locale { rank: rank, file: file };
                if self.occupied().invert().query(locale) {
                    print!("_ ");
                } else {
                    for &team in [Team::Orange, Team::Blue].iter() {
                        for &figurine_class in
                            Agent::dramatis_personae(team).iter() {
                                if self.agent_to_pinfield_ref(
                                    figurine_class).query(locale) {
                                        figurine_class.render_caricature();
                                        print!(" ");
                                }
                        }
                    }
                }
            }
            println!("");
        }
    }
}


fn main() {
    let arena = WorldState::new();
    arena.display();
    println!("");
}


#[cfg(test)]
mod test {
    use super::{WorldState, Patch, Commit};
    use space::{Locale, Pinfield};
    use identity::{Team, JobDescription, Agent};

    #[test]
    fn test_agent_to_pinfield_ref_on_new_gamestate() {
        let state = WorldState::new();
        let agent = Agent { team: Team::Blue,
                            job_description: JobDescription::Princess };
        let blue_princess_realm = state.agent_to_pinfield_ref(agent);
        assert!(blue_princess_realm.query(Locale { rank: 7, file: 3 }));
    }

    #[test]
    fn test_orange_servants_to_locales_from_new_gamestate() {
        let state = WorldState::new();
        let mut expected = Vec::new();
        for file in 0..8 {
            expected.push(Locale { rank: 1, file: file });
        }
        assert_eq!(expected, state.orange_servants.to_locales());
    }

    #[test]
    fn test_orange_servant_lookahead_from_original_position() {
        let state = WorldState::new();
        let premonitions = state.servant_lookahead(Team::Orange);
        assert_eq!(16, premonitions.len());
        // although granted that a more thorough test would actually
        // say something about the nature of the positions, rather than
        // just how many there are
    }

    #[test]
    fn test_orange_pony_lookahead_from_original_position() {
        let state = WorldState::new();
        let premonitions = state.pony_lookahead(Team::Orange);
        assert_eq!(4, premonitions.len());
        let collected = premonitions.iter().map(
            |p| p.tree.orange_ponies.to_locales()).collect::<Vec<_>>();
        assert_eq!(
            vec![vec![Locale { rank: 0, file: 6 },
                      Locale { rank: 2, file: 0 }],
                 vec![Locale { rank: 0, file: 6 },
                      Locale { rank: 2, file: 2 }],
                 vec![Locale { rank: 0, file: 1 },
                      Locale { rank: 2, file: 5 }],
                 vec![Locale { rank: 0, file: 1 },
                      Locale { rank: 2, file: 7 }]],
                 collected
        );
    }

    #[test]
    fn concerning_occupying_agents() {
        let state = WorldState::new();
        let b8 = Locale { rank: 7, file: 1 };
        assert_eq!(Agent { team: Team::Blue,
                           job_description: JobDescription::Pony },
                   state.occupying_agent(b8).unwrap());
        let c4 = Locale { rank: 3, file: 2 };
        assert_eq!(None, state.occupying_agent(c4));
    }

    #[test]
    fn concerning_peaceful_patch_application() {
        let state = WorldState::new();
        let e2 = Locale { rank: 4, file: 1 };
        let e4 = Locale { rank: 4, file: 3 };
        let patch = Patch {
            star: Agent {
                team: Team::Orange,
                job_description: JobDescription::Servant
            },
            whence: e2,
            whither: e4
        };
        let new_state = state.apply(patch).unwrap().tree;
        assert_eq!(Agent { team: Team::Orange,
                           job_description: JobDescription::Servant },
                   new_state.occupying_agent(e4).unwrap());
        assert_eq!(None, new_state.occupying_agent(e2));
    }

    #[test]
    fn concerning_stunning_patch_application_in_natural_setting() {
        let state = WorldState::new();
        let orange_begins = Patch {
            star: Agent {
                team: Team::Orange,
                job_description: JobDescription::Servant
            },
            whence: Locale::from_algebraic("e2".to_string()),
            whither: Locale::from_algebraic("e4".to_string())
        };
        let blue_replies = Patch {
            star: Agent {
                team: Team::Blue,
                job_description: JobDescription::Servant
            },
            whence: Locale::from_algebraic("d7".to_string()),
            whither: Locale::from_algebraic("d5".to_string())
        };
        let orange_counterreplies = Patch {
            star: Agent {
                team: Team::Orange,
                job_description: JobDescription::Servant
            },
            whence: Locale::from_algebraic("e4".to_string()),
            whither: Locale::from_algebraic("d5".to_string())
        };
        let first_commit = state.apply(orange_begins).unwrap();
        assert_eq!(None, first_commit.hospitalization);
        let second_commit = first_commit.tree.apply(blue_replies).unwrap();
        assert_eq!(None, second_commit.hospitalization);
        let crucial_commit = second_commit.tree.apply(
            orange_counterreplies).unwrap();
        let new_state = crucial_commit.tree;
        assert_eq!(Agent { team: Team::Orange,
                           job_description: JobDescription::Servant },
                   new_state.occupying_agent(
                       Locale::from_algebraic("d5".to_string())).unwrap());
        let stunned = crucial_commit.hospitalization.unwrap();
        assert_eq!(Agent { team: Team::Blue,
                           job_description: JobDescription::Servant },
                   stunned);
    }
}