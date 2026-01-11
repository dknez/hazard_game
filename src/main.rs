// Implementation of a Risk-like turn-based strategy game in Rust.

use std::io;
use std::io::{Write}; // Import the Write trait for flushing stdout
use std::collections::HashMap;
use petgraph::graph::UnGraph; // For use in graph representation of the world map
use petgraph::visit::IntoNodeReferences;
use rand::Rng;
use rand::prelude::SliceRandom;

#[derive(Clone, Debug)]
enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    Indigo,
}

#[derive(Debug)]
struct Player {
    name: String,
    color: Color,
    army_per_territory: HashMap<u32,u32>, // Mapping of territory index to number of armies
}

impl Player {
    fn new(name: String, color: Color) -> Self {
        Player {
            name,
            color,
            army_per_territory: HashMap::new(),
        }
    }
}

fn setup_players(names: Vec<String>) -> Vec<Player> {
    let colors = vec![
        Color::Red,
        Color::Blue,
        Color::Green,
        Color::Yellow,
        Color::Indigo,
    ];

    let mut players = Vec::new();
    for (i, name) in names.into_iter().enumerate() {
        let color = colors.get(i).unwrap_or(&Color::Red).clone(); // Default to Red if out of colors

        // We use player.color in the print statement to avoid a compiler warning
        // and the color field never being read.
        let player = Player::new(name, color);
        println!("{} has been assigned color {:?}", player.name, player.color);
        players.push(player);
    }

    players
}

fn assign_territories_and_armies_to_players(
    territories: &UnGraph<&'static str, ()>,
    players: &mut Vec<Player>) {
    let mut territory_indices: Vec<u32> = territories
        .node_indices()
        .map(|index| index.index() as u32)
        .collect();

    // Randomly permute territory_indices so that we assign territories to players in
    // a random manner.
    let mut rng = rand::thread_rng();
    territory_indices.shuffle(&mut rng);

    let mut player_index = 0;
    for territory_index in territory_indices {
        players[player_index].army_per_territory.insert(territory_index, 0); // Start with 0 armies
        player_index = (player_index + 1) % players.len();
    }

    // Now assign armies to each territory
    let armies_per_player =
        match players.len() {
            1 => 45,
            2 => 40,
            3 => 35,
            4 => 30,
            5 => 25,
            _ => 0, // This case should not occur due to earlier checks
        };

    println!("Do you want to manually assign troops, or automatically assign toops to all territories evenly?");
    print!("Type 1 for manual, or 2 for automatic even assignment: ");

    io::stdout().flush().expect("Failed to flush stdout");

    let mut manual_or_even_assignment = String::new();
    io::stdin()
        .read_line(&mut manual_or_even_assignment)
        .expect("Failed to read line");
    let manual_or_even_assignment = manual_or_even_assignment.trim().parse().expect("Please type a number!");

    let mut is_manual_assignment = false;
    match manual_or_even_assignment {
        1 => {
            is_manual_assignment = true;
            println!("Manual assignment mode selected.");
        },
        2 => {
            println!("Automatic even assignment mode selected.");
        },
        _ => {
            println!("Invalid input. Defaulting to automatic even assignment.");
        }
    }

    if is_manual_assignment {
        let mut army_count_per_player = vec![0; players.len()];
        'outer_loop: loop {
            let mut player_index = 0;
            for player in players.iter_mut() {
                println!("Player: {}, choose a territory index to add an army:", player.name);

                let mut sorted_territory_indices = Vec::new();
                for territory_index in player.army_per_territory.keys() {
                    sorted_territory_indices.push(*territory_index);
                }
                sorted_territory_indices.sort();

                for territory_index in sorted_territory_indices {
                    println!("Territory index: {}, territory name: {}",
                        territory_index,
                        territories.node_weight(petgraph::graph::NodeIndex::new(territory_index as usize)).unwrap());
                }

                let mut selected_index = String::new();
                io::stdin()
                    .read_line(&mut selected_index)
                    .expect("Failed to read line");
                // Reuse the selected_index variable name (requires us to re-declare with "let")
                let selected_index = selected_index.trim().parse().expect("Please type a number!");

                if let Some(armies) = player.army_per_territory.get_mut(&selected_index) {
                    *armies += 1;
                    army_count_per_player[player_index] += 1;

                    if army_count_per_player[player_index] >= armies_per_player {
                        println!("Player {} has assigned all their armies.", player.name);
                        break 'outer_loop;
                    }
                } else {
                    println!("You do not own this territory.");
                    continue;
                }

                player_index += 1;
            }
        }
        for player in players.iter_mut() {
            let mut total_armies = 0;
            'player_loop: loop {
                for (_territory_index, armies) in player.army_per_territory.iter_mut() {
                    // We need to check if we've already assigned enough armies since
                    // we iterate over all territories
                    if total_armies >= armies_per_player {
                        break 'player_loop;
                    }

                    *armies += 1;
                    total_armies += 1;
                }
            }
        }
        // for player in players.iter_mut() {
        //     println!("Player: {}", player.name);
        //     for (territory_index, _) in player.army_per_territory.iter() {
        //         let territory_name = territories.node_weight(petgraph::graph::NodeIndex::new(*territory_index as usize)).unwrap();
        //         print!("Enter number of armies to place in {} (between 1 and 5): ", territory_name);
        //         io::stdout().flush().expect("Failed to flush stdout");
        //         let mut n_armies_input = String::new();
        //         io::stdin()
        //             .read_line(&mut n_armies_input)
        //             .expect("Failed to read line");
        //         let n_armies = n_armies_input.trim().parse().expect("Please type a number!");
        //         player.army_per_territory.insert(*territory_index, n_armies);
        //     }
        // }
    }
    else {
        for player in players.iter_mut() {
            let mut total_armies = 0;
            'player_loop: loop {
                for (_territory_index, armies) in player.army_per_territory.iter_mut() {
                    // We need to check if we've already assigned enough armies since
                    // we iterate over all territories
                    if total_armies >= armies_per_player {
                        break 'player_loop;
                    }

                    *armies += 1;
                    total_armies += 1;
                }
            }
        }
    }

    println!("\nTerritories and armies have been assigned to players as follows:");
    print_players(&territories, players);
}

fn print_players(territories: &UnGraph<&'static str, ()>, players: &Vec<Player>) {
    for player in players.iter() {
        print_player(territories, player);
    }
}

fn print_player(territories: &UnGraph<&'static str, ()>, player: &Player) {
    println!("Player: {}", player.name);
    for (territory_index, armies) in player.army_per_territory.iter() {
        let territory_name = territories.node_weight(petgraph::graph::NodeIndex::new(*territory_index as usize)).unwrap();
        println!("  Territory: {}, Armies: {}", territory_name, armies);
    }
    println!("");
}

fn setup_territories() -> UnGraph<&'static str, ()> {
    let mut territories = UnGraph::<&str, ()>::new_undirected();

    let aus_wa = territories.add_node("Western Australia");
    let aus_ea = territories.add_node("Eastern Australia");
    let aus_ng = territories.add_node("New Guinea");
    let aus_id = territories.add_node("Indonesia");

    territories.add_edge(aus_wa, aus_ea, ());
    territories.add_edge(aus_wa, aus_id, ());
    territories.add_edge(aus_ea, aus_ng, ());
    territories.add_edge(aus_ng, aus_id, ());

    let asia_in = territories.add_node("India");
    let asia_ch = territories.add_node("China");
    let asia_si = territories.add_node("Siberia");
    let asia_mo = territories.add_node("Mongolia");
    let asia_ja = territories.add_node("Japan");
    let asia_ya = territories.add_node("Yakutsk");
    let asia_ir = territories.add_node("Irkutsk");
    let asia_af = territories.add_node("Afghanistan");
    let asia_me = territories.add_node("Middle East");
    let asia_se = territories.add_node("Southeast Asia");
    let asia_ka = territories.add_node("Kamchatka");
    let asia_ur = territories.add_node("Ural");

    territories.add_edge(aus_id, asia_se, ());
    territories.add_edge(asia_se, asia_ch, ());
    territories.add_edge(asia_se, asia_in, ());
    territories.add_edge(asia_in, asia_af, ());
    territories.add_edge(asia_in, asia_me, ());
    territories.add_edge(asia_in, asia_ch, ());
    territories.add_edge(asia_me, asia_af, ());
    territories.add_edge(asia_ch, asia_af, ());
    territories.add_edge(asia_ch, asia_ur, ());
    territories.add_edge(asia_ch, asia_si, ());
    territories.add_edge(asia_ch, asia_mo, ());
    territories.add_edge(asia_af, asia_ur, ());
    territories.add_edge(asia_ja, asia_mo, ());
    territories.add_edge(asia_ja, asia_ka, ());
    territories.add_edge(asia_mo, asia_si, ());
    territories.add_edge(asia_mo, asia_ir, ());
    territories.add_edge(asia_mo, asia_ka, ());
    territories.add_edge(asia_si, asia_ya, ());
    territories.add_edge(asia_si, asia_ur, ());
    territories.add_edge(asia_si, asia_ir, ());
    territories.add_edge(asia_ka, asia_ir, ());
    territories.add_edge(asia_ka, asia_ya, ());
    territories.add_edge(asia_ya, asia_ir, ());

    territories
}

fn print_all_territories(territories: &UnGraph<&'static str, ()>) {
    println!("World with {} territories has been set up. Territories:\n", territories.node_count());

    for (node_index, weight) in territories.node_references() {
        println!("Territory: {}", weight);

        for neighbor in territories.neighbors(node_index) {
            let neighbor_weight = territories.node_weight(neighbor).unwrap();
            println!("  Neighbor: {}", neighbor_weight);
        }
        println!("");
    }
}

fn add_armies_to_player(
    player: &mut Player,) {
    let total_territories: u32 = player.army_per_territory.len() as u32;
    let additional_armies = std::cmp::max(3, total_territories / 3);

    println!(
        "Player {} receives {} additional armies to deploy.",
        player.name, additional_armies);

    let mut additional_armies_count = 0;
    'outer_loop: loop {
        for (_territory_index, armies) in player.army_per_territory.iter_mut() {
            // We need to check if we've already assigned enough armies since
            // we iterate over all territories
            if additional_armies_count >= additional_armies {
                break 'outer_loop;
            }

            *armies += 1;
            additional_armies_count += 1;
        }
    }
}

fn perform_attack(
    territories: &UnGraph<&'static str, ()>,
    players: &mut Vec<Player>,
    attacker_idx: usize,
    defender_idx: usize,
    attacking_territory_index: u32,
    target_territory_index: u32,) -> bool {

    // We currently hard-code to using the maximum number of armies rather
    // than asking every time.
    let use_max_armies = true;

    let attacking_territory_name = territories.node_weight(petgraph::graph::NodeIndex::new(attacking_territory_index as usize)).unwrap();
    let target_territory_name = territories.node_weight(petgraph::graph::NodeIndex::new(target_territory_index as usize)).unwrap();

    println!(
        "Player {} is attacking from {} to {}",
        players[attacker_idx].name, attacking_territory_name, target_territory_name);

    let n_attack_armies = *players[attacker_idx].army_per_territory.get(&attacking_territory_index).unwrap();
    println!("Player {} has {} armies in {}",
        players[attacker_idx].name,
        n_attack_armies,
        attacking_territory_name);
    let max_attack_armies = std::cmp::min(n_attack_armies - 1, 3);

    let mut n_attacking_armies;
    if use_max_armies {
        n_attacking_armies = max_attack_armies;
        println!("Player {} is attacking with {} armies", players[attacker_idx].name, n_attacking_armies);
    }
    else {
        print!("Choose number of armies to attack with (between 1 and {}): ", max_attack_armies);

        // Need to flush stdout to ensure the prompt appears before reading input
        io::stdout().flush().expect("Failed to flush stdout");

        let mut n_attacking_armies_input = String::new();
        io::stdin()
            .read_line(&mut n_attacking_armies_input)
            .expect("Failed to read line");
        n_attacking_armies = n_attacking_armies_input.trim().parse().expect("Please type a number!");
        if n_attacking_armies > max_attack_armies {
            n_attacking_armies = max_attack_armies;
            println!("Requested too many attacking armies, reducing to {}", n_attacking_armies);
        }
        if n_attacking_armies == 0 {
            n_attacking_armies = 1;
            println!("Cannot attack with zero armies, increasing to 1.");
        }
    }

    let n_defend_armies = *players[defender_idx].army_per_territory.get(&target_territory_index).unwrap();
    println!("Player {} has {} armies in {}",
        players[defender_idx].name,
        n_defend_armies,
        target_territory_name);
    let max_defend_armies = std::cmp::min(n_defend_armies,2);

    let mut n_defending_armies;
    if use_max_armies {
        n_defending_armies = max_defend_armies;
        println!("Player {} is defending with {} armies", players[defender_idx].name, n_defending_armies);
    }
    else {
        print!("Choose number of armies to defend with (between 1 and {}): ", max_defend_armies);

        io::stdout().flush().expect("Failed to flush stdout");

        let mut n_defending_armies_input = String::new();
        io::stdin()
            .read_line(&mut n_defending_armies_input)
            .expect("Failed to read line");
        n_defending_armies = n_defending_armies_input.trim().parse().expect("Please type a number!");
        if n_defending_armies > max_defend_armies {
            n_defending_armies = max_defend_armies;
            println!("Requested too many defending armies, reducing to {}", n_defending_armies);
        }
        if n_defending_armies == 0 {
            n_defending_armies = 1;
            println!("Cannot defend with zero armies, increasing to 1.");
        }
    }

    let mut rng = rand::thread_rng();

    let mut attacking_dice_rolls = Vec::<u8>::new(); // Placeholder for dice rolls
    for _ in 0..n_attacking_armies {
        let dice_roll = rng.gen_range(1..=6);
        println!("Attacker rolled: {}", dice_roll);
        attacking_dice_rolls.push(dice_roll);
    }
    let mut defending_dice_rolls = Vec::<u8>::new(); // Placeholder for dice rolls
    for _ in 0..n_defending_armies {
        let dice_roll = rng.gen_range(1..=6);
        println!("Defender rolled: {}", dice_roll);
        defending_dice_rolls.push(dice_roll);
    }

    attacking_dice_rolls.sort_by(|a, b| b.cmp(a)); // Sort descending
    defending_dice_rolls.sort_by(|a, b| b.cmp(a)); // Sort descending

    let n_comparisons = std::cmp::min(attacking_dice_rolls.len(), defending_dice_rolls.len());
    for i in 0..n_comparisons {
        if attacking_dice_rolls[i] > defending_dice_rolls[i] {
            println!("Attacker wins comparison {}: {} vs {}", i + 1, attacking_dice_rolls[i], defending_dice_rolls[i]);
            // Defender loses one army
            let defender_armies = players[defender_idx].army_per_territory.get_mut(&target_territory_index).unwrap();
            *defender_armies -= 1;
        } else {
            println!("Defender wins comparison {}: {} vs {}", i + 1, defending_dice_rolls[i], attacking_dice_rolls[i]);
            // Attacker loses one army
            let attacker_armies = players[attacker_idx].army_per_territory.get_mut(&attacking_territory_index).unwrap();
            *attacker_armies -= 1;
        }
    }

    let new_n_attack_armies = *players[attacker_idx].army_per_territory.get(&attacking_territory_index).unwrap();
    println!("Player {} now has {} armies in {}",
        players[attacker_idx].name,
        new_n_attack_armies,
        attacking_territory_name);

    let new_n_defend_armies = *players[defender_idx].army_per_territory.get(&target_territory_index).unwrap();
    println!("Player {} now has {} armies in {}",
        players[defender_idx].name,
        new_n_defend_armies,
        target_territory_name);

    if new_n_defend_armies == 0 {
        players[defender_idx].army_per_territory.remove(&target_territory_index);

        println!("Player {} conquered territory {}!",
            players[attacker_idx].name,
            target_territory_name);

        // We move at least the number of attacking armies used in the attack,
        // up to the maximum number of armies minus one left behind in the
        // attacking territory.
        let max_movable_armies = new_n_attack_armies - 1;
        let min_movable_armies = n_attacking_armies;
        print!("Choose number of armies to move into conquered territory (between {} and {}): ",
            min_movable_armies,
            max_movable_armies);

        io::stdout().flush().expect("Failed to flush stdout");

        let mut n_movable_armies_input = String::new();
        io::stdin()
            .read_line(&mut n_movable_armies_input)
            .expect("Failed to read line");
        let mut n_movable_armies = n_movable_armies_input.trim().parse().expect("Please type a number!");
        if n_movable_armies > max_movable_armies {
            n_movable_armies = max_movable_armies;
            println!("Requested too many movable armies, reducing to {}", n_movable_armies);
        }
        if n_movable_armies < min_movable_armies {
            n_movable_armies = min_movable_armies;
            println!("Requested too few movable armies, increasing to {}", n_movable_armies);
        }

        players[attacker_idx].army_per_territory.insert(target_territory_index, n_movable_armies);
        let attacker_armies = players[attacker_idx].army_per_territory.get_mut(&attacking_territory_index).unwrap();
        *attacker_armies -= n_movable_armies;
    }

    if new_n_attack_armies == 1 {
        println!("Player {} only has one army left, attack on {} cannot continue",
            players[attacker_idx].name,
            target_territory_name);
    }

    (new_n_defend_armies == 0) || (new_n_attack_armies == 1)
}

fn check_game_over(players: &Vec<Player>, territories: &UnGraph<&'static str, ()>) -> bool {
    let total_territories = territories.node_count();
    for player in players {
        let n_territories = player.army_per_territory.len();
        if n_territories == total_territories {
            println!("Game Over! Player {} has conquered all territories.", player.name);
            return true;
        }
    }
    false
}

fn main() {
    println!("\n==== Welcome to Hazard, the Risk-like strategy game! ====");

    let territories = setup_territories();
    print_all_territories(&territories);

    print!("Please enter the number of players between 1 and 5: ");

    // Need to flush stdout to ensure the prompt appears before reading input
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let number_of_players: i32 = input.trim().parse().expect("Please type a number!");
    assert!(
        number_of_players >= 1 && number_of_players <= 5,
        "Number of players must be between 1 and 5"
    );
    println!("==== Setting up game for {} players ====", number_of_players);

    let mut player_names = Vec::new();
    for i in 0..number_of_players {
        print!("Enter name for Player {}: ", i + 1);
        io::stdout().flush().expect("Failed to flush stdout");

        let mut name_input = String::new();
        io::stdin()
            .read_line(&mut name_input)
            .expect("Failed to read line");

        player_names.push(name_input.trim().to_string());
    }
    println!("");

    let mut players = setup_players(player_names);

    // Assign territories and initial armies here
    assign_territories_and_armies_to_players(&territories, &mut players);

    // Now we start the game
    'game_loop: loop {
        for player_idx in 0..players.len() {

            {
                let mut_player = &mut players[player_idx];
                println!("\n==== Player {}'s turn ====", mut_player.name);
                println!("\n==== Reinforcement phase ====");

                add_armies_to_player(mut_player);
                println!();
            }

            let mut attack_count = 0;

            let mut defender_idx_option: Option<usize> = None;
            let mut attacking_territory_index: u32 = 0;
            let mut target_territory_index: u32 = 0;

            let mut attack_finished = false;

            println!("==== Attack phase ====");
            loop {

                {
                    println!("==== Attack phase round {} ====", attack_count + 1);

                    let player = &players[player_idx];
                    print_player(&territories, player);

                    let mut choose_new_attack = true;

                    if attack_count > 0 && !attack_finished {
                        print!("Do you want to attack the territory again? (y/n): ");
                        io::stdout().flush().expect("Failed to flush stdout");

                        let mut repeat_attack = String::new();
                        io::stdin()
                            .read_line(&mut repeat_attack)
                            .expect("Failed to read line");
                        match repeat_attack.trim() {
                            "y" | "Y" => {
                                choose_new_attack = false;
                            }
                            "n" | "N" => {
                                // In this case we leave choose_new_attack as true, which will
                                // keep attacking_territory_index and target_territory_index unchanged
                            }
                            _ => {
                                println!("Invalid input, skipping attack phase.");
                            }
                        }
                    }

                    if choose_new_attack {
                        print!("Do you want to attack any territory? (y/n): ");
                        io::stdout().flush().expect("Failed to flush stdout");

                        let mut input = String::new();
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read line");
                        match input.trim() {
                            "y" | "Y" => {
                                // Get sorted list of territory indices, since it's easier for the player
                                // to read when it is ordered.
                                let mut sorted_attacking_territory_indices = Vec::new();
                                for territory_index in player.army_per_territory.keys() {
                                    sorted_attacking_territory_indices.push(*territory_index);
                                }
                                sorted_attacking_territory_indices.sort();
                                println!("Select territory index to attack from:");
                                for territory_index in sorted_attacking_territory_indices {
                                    println!("Territory index: {}, territory name: {}",
                                        territory_index,
                                        territories.node_weight(petgraph::graph::NodeIndex::new(territory_index as usize)).unwrap());
                                }

                                print!("Attacking from territory index: ");
                                io::stdout().flush().expect("Failed to flush stdout");

                                let mut selected_index = String::new();
                                io::stdin()
                                    .read_line(&mut selected_index)
                                    .expect("Failed to read line");
                                attacking_territory_index = selected_index.trim().parse().expect("Please type a number!");

                                if let Some(armies) = player.army_per_territory.get(&attacking_territory_index) {
                                    if *armies < 2 {
                                        println!("Not enough armies to attack from this territory.");
                                        continue;
                                    }
                                } else {
                                    println!("You do not own this territory.");
                                    continue;
                                }

                                let mut sorted_target_territory_indices = Vec::new();
                                sorted_target_territory_indices.sort();
                                for neighbor in territories.neighbors(petgraph::graph::NodeIndex::new(attacking_territory_index as usize)) {
                                    if let Some(_) = player.army_per_territory.get(&neighbor.index().try_into().unwrap()) {
                                        // Skip territories owned by the player
                                        continue;
                                    }
                                    sorted_target_territory_indices.push(neighbor.index());
                                }

                                if sorted_target_territory_indices.is_empty() {
                                    println!("No target territories available to attack from {}!",
                                      territories.node_weight(petgraph::graph::NodeIndex::new(attacking_territory_index as usize)).unwrap());
                                    continue;
                                }

                                sorted_target_territory_indices.sort();

                                println!("\nSelect target territory index:");
                                for territory_index in sorted_target_territory_indices {
                                    println!("Territory index: {}, territory name: {}",
                                        territory_index,
                                        territories.node_weight(petgraph::graph::NodeIndex::new(territory_index as usize)).unwrap());
                                }

                                print!("Targeting territory index: ");
                                io::stdout().flush().expect("Failed to flush stdout");

                                let mut target_index = String::new();
                                io::stdin()
                                    .read_line(&mut target_index)
                                    .expect("Failed to read line");
                                target_territory_index = target_index.trim().parse().expect("Please type a number!");

                                // defender is the player who owns the target territory
                                for other_player in players.iter() {
                                    if other_player.name != player.name {
                                        if other_player.army_per_territory.contains_key(&target_territory_index) {
                                            defender_idx_option = Some(players.iter().position(|p| p.name == other_player.name).unwrap() as usize);
                                            break;
                                        }
                                    }
                                }
                            }
                            "n" | "N" => {
                                println!("==== Attack phase has ended, player {}'s turn is over ====", player.name);
                                break;
                            }
                            _ => {
                                println!("Invalid input, skipping attack phase.");
                                break;
                            }
                        }
                    }
                }

                if let Some(defender_idx) = defender_idx_option
                {
                    // attack_finished is true if the territory was conquered, or
                    // if the attacker only has one army left on the attacking territory
                    attack_finished =
                        perform_attack(
                            &territories,
                            &mut players,
                            player_idx,
                            defender_idx,
                            attacking_territory_index,
                            target_territory_index);

                    attack_count += 1;

                    // Check if one player now has all the territories. If so, we can exit
                    // the game.
                    if check_game_over(&players, &territories) {
                        break 'game_loop;
                    }
                }

                println!();
            }
        }
    }
}
