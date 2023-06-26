#[derive(Default)]
pub struct Goon_AutoCables; // Cable spawners, aka auto cables
pub struct Goon_AutoDP; // disposal pipe spawners
pub struct Goon_AutoLattice; // i think you get the idea
pub struct Goon_AutoCatwalk;

impl RenderPass for Goon_AutoCables {
    fn neighborhood_appearance<'a>(&self,
        atom: &Atom<'a>,
        objtree: &'a ObjectTree,
        neighborhood: &Neighborhood<'a, '_>,
        output: &mut Vec<Sprite<'a>>,
        bump: &'a bumpalo::Bump,
    ) -> bool {
        if !atom.istype("/obj/cable/auto") {
            return true;
        }

        let cable_layer = atom.get_var("cable_layer", objtree).as_str().unwrap_or("l2");

        let mut under_smes = false;
        let mut under_terminal = false;
        for atom in neighborhood.center() {
            if atom.istype("/obj/machinery/power/terminal/") {
                under_terminal = true;
            } else if atom.istype("/obj/machinery/power/smes/") {
                under_smes = true;
            }
        }

        // calculate linked dirs
        let mut linked_dirs = 0;
        'dir: for &check_dir in Dir::CARDINALS {
            let turf = neighborhood.offset(check_dir);
            // Don't link between SMES and terminal
            if under_smes {
                for atom in turf {
                    if atom.istype("/obj/machinery/power/terminal/") {
                        continue 'dir;
                    }
                }
            } else if under_terminal {
                for atom in turf {
                    if atom.istype("/obj/machinery/power/smes/") {
                        continue 'dir;
                    }
                }
            }

            for atom in turf {
                if atom.istype("/obj/cable")
                    && atom
                        .get_var("cable_layer", objtree)
                        .as_str()
                        .unwrap_or("l2")
                        == cable_layer
                {
                    if atom.istype("/obj/cable/auto"){
                        linked_dirs |= check_dir.to_int();
                        break;
                    }
                    else if atom.icon_state == check_dir {
                        linked_dirs |= check_dir.to_int();
                        break;
                    }

                }
            }
        }

        // calculate icon state
        let mut icon_state;
        if linked_dirs == 0 {
            icon_state = bumpalo::format!(in bump, "{}-noconnection", cable_layer);
        } else {
            icon_state = bumpalo::format!(in bump, "{}", cable_layer);
            let mut count = 0;
            for &check_dir in Dir::CARDINALS {
                if linked_dirs & check_dir.to_int() != 0 {
                    let _ = write!(icon_state, "-{}", check_dir.to_int());
                    count += 1;
                }
            }
            if count > 1 && should_have_node(neighborhood.center()) {
                let _ = write!(icon_state, "-node");
            }
        };

        output.push(Sprite {
            icon_state: icon_state.into_bump_str(),
            .. atom.sprite
        });
        false
    }
}
