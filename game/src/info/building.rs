use crate::app::App;
use crate::colors;
use crate::helpers::ID;
use crate::info::{make_table, make_tabs, person, InfoTab};
use ezgui::{hotkey, Btn, EventCtx, GeomBatch, Key, Line, Text, TextExt, Widget};
use map_model::BuildingID;
use sim::PersonID;
use std::collections::HashMap;

#[derive(Clone)]
pub enum Tab {
    // If we're live updating, the people inside could change! We're choosing to freeze the list
    // here.
    People(Vec<PersonID>, usize),
    OSM,
}
impl std::cmp::PartialEq for Tab {
    fn eq(&self, other: &Tab) -> bool {
        match (self, other) {
            (Tab::People(_, _), Tab::People(_, _)) => true,
            (Tab::OSM, Tab::OSM) => true,
            _ => false,
        }
    }
}

pub fn info(
    ctx: &EventCtx,
    app: &App,
    id: BuildingID,
    tab: InfoTab,
    header_btns: Widget,
    action_btns: Vec<Widget>,
    batch: &mut GeomBatch,
    hyperlinks: &mut HashMap<String, (ID, InfoTab)>,
) -> Vec<Widget> {
    let mut rows = vec![];

    let b = app.primary.map.get_b(id);

    rows.push(Widget::row(vec![
        Line(format!("Building #{}", id.0)).roboto_bold().draw(ctx),
        header_btns,
    ]));

    rows.push(make_tabs(ctx, hyperlinks, ID::Building(id), tab.clone(), {
        let mut tabs = vec![
            ("Main", InfoTab::Nil),
            // TODO Has to be different than lane's OSM :(
            ("OSM", InfoTab::Bldg(Tab::OSM)),
        ];

        let ppl = app.primary.sim.bldg_to_people(id);
        if !ppl.is_empty() {
            tabs.push(("People", InfoTab::Bldg(Tab::People(ppl, 0))));
        }

        tabs
    }));

    match tab {
        InfoTab::Nil => {
            rows.extend(action_btns);

            let mut kv = Vec::new();

            kv.push(("Address", b.just_address(&app.primary.map)));
            if let Some(name) = b.just_name() {
                kv.push(("Name", name.to_string()));
            }

            if let Some(ref p) = b.parking {
                kv.push(("Parking", format!("{} spots via {}", p.num_stalls, p.name)));
            } else {
                kv.push(("Parking", "None".to_string()));
            }

            rows.extend(make_table(ctx, kv));

            let mut txt = Text::new();

            if !b.amenities.is_empty() {
                txt.add(Line(""));
                if b.amenities.len() > 1 {
                    txt.add(Line(format!("{} amenities:", b.amenities.len())));
                }
                for (name, amenity) in &b.amenities {
                    txt.add(Line(format!("- {} (a {})", name, amenity)));
                }
            }

            let trip_lines = app.primary.sim.count_trips_involving_bldg(id).describe();
            if !trip_lines.is_empty() {
                txt.add(Line(""));
                for line in trip_lines {
                    txt.add(Line(line));
                }
            }

            let cars = app.primary.sim.get_parked_cars_by_owner(id);
            if !cars.is_empty() {
                txt.add(Line(""));
                txt.add(Line(format!(
                    "{} parked cars owned by this building",
                    cars.len()
                )));
                // TODO Jump to it or see status
                for p in cars {
                    txt.add(Line(format!("- {}", p.vehicle.id)));
                }
            }

            if !txt.is_empty() {
                rows.push(txt.draw(ctx))
            }
        }
        InfoTab::Bldg(Tab::OSM) => {
            let mut kv = Vec::new();

            // TODO Not OSM, but separate debug panel seems weird
            kv.push((
                "Dist along sidewalk".to_string(),
                b.front_path.sidewalk.dist_along().to_string(),
            ));

            for (k, v) in &b.osm_tags {
                kv.push((k.to_string(), v.to_string()));
            }

            rows.extend(make_table(ctx, kv));
        }
        InfoTab::Bldg(Tab::People(ppl, idx)) => {
            let mut inner = vec![
                // TODO Keys are weird! But left/right for speed
                Widget::row(vec![
                    if idx != 0 {
                        hyperlinks.insert(
                            "previous".to_string(),
                            (
                                ID::Building(id),
                                InfoTab::Bldg(Tab::People(ppl.clone(), idx - 1)),
                            ),
                        );
                        Btn::text_fg("<").build(ctx, "previous", hotkey(Key::UpArrow))
                    } else {
                        Btn::text_fg("<").inactive(ctx)
                    }
                    .margin(5),
                    format!("Occupant {}/{}", idx + 1, ppl.len()).draw_text(ctx),
                    if idx != ppl.len() - 1 {
                        hyperlinks.insert(
                            "next".to_string(),
                            (
                                ID::Building(id),
                                InfoTab::Bldg(Tab::People(ppl.clone(), idx + 1)),
                            ),
                        );
                        Btn::text_fg(">").build(ctx, "next", hotkey(Key::DownArrow))
                    } else {
                        Btn::text_fg(">").inactive(ctx)
                    }
                    .margin(5),
                ])
                .centered(),
            ];
            inner.extend(person::info(
                ctx,
                app,
                ppl[idx],
                None,
                Vec::new(),
                hyperlinks,
            ));
            rows.push(Widget::col(inner).bg(colors::INNER_PANEL_BG));
        }
        _ => unreachable!(),
    }

    for p in app.primary.sim.get_parked_cars_by_owner(id) {
        batch.push(
            app.cs.get("something associated with something else"),
            app.primary
                .draw_map
                .get_obj(
                    ID::Car(p.vehicle.id),
                    app,
                    &mut app.primary.draw_map.agents.borrow_mut(),
                    ctx.prerender,
                )
                .unwrap()
                .get_outline(&app.primary.map),
        );
    }

    rows
}
