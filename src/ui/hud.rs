use super::*;

fn positioned(left: Val, top: Val, width: Val) -> Node {
    Node {
        position_type: PositionType::Absolute,
        left,
        top,
        width,
        ..default()
    }
}

pub(super) fn spawn(root: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    root.spawn((
        Item::Hud,
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
    ))
    .with_children(|hud| {
        hud.spawn((
            text("К Р О М К А   Б У Р И", font, 12.0, CREAM),
            positioned(px(30), px(26), auto()),
        ));
        hud.spawn((
            Item::Objective,
            text("", font, 15.0, CREAM),
            positioned(percent(30), px(22), percent(40)),
            TextLayout::justify(Justify::Center),
        ));
        hud.spawn((
            Item::Compass,
            text("", font, 12.0, GOLD),
            positioned(percent(30), px(68), percent(40)),
            TextLayout::justify(Justify::Center),
        ));
        instruments(hud, font);
        hud.spawn((
            Item::Threat,
            text("", font, 12.0, GOLD),
            Node {
                position_type: PositionType::Absolute,
                right: px(30),
                bottom: px(55),
                width: px(320),
                ..default()
            },
            TextLayout::justify(Justify::Right),
        ));
        hud.spawn((
            text(
                "H  ПОМОЩЬ   ESC  ПАУЗА   P  ФОТО   F2  ГРАФИКА",
                font,
                10.0,
                MUTED,
            ),
            Node {
                position_type: PositionType::Absolute,
                right: px(30),
                bottom: px(28),
                ..default()
            },
        ));
        hud.spawn((
            Item::Notice,
            text("", font, 13.0, CREAM),
            Node {
                position_type: PositionType::Absolute,
                left: percent(26),
                bottom: px(42),
                width: percent(48),
                padding: UiRect::all(px(12)),
                ..default()
            },
            TextLayout::justify(Justify::Center),
            BackgroundColor(Color::srgba(0.025, 0.075, 0.085, 0.74)),
        ));
        hud.spawn((
            Item::Target,
            text("", font, 12.0, GOLD),
            positioned(px(0), px(0), px(220)),
            TextLayout::justify(Justify::Center),
        ));
        hud.spawn((
            Item::Crosshair,
            text("+", font, 22.0, Color::srgba(0.9, 0.95, 0.87, 0.7)),
            positioned(px(0), px(0), auto()),
        ));
    });
}

fn instruments(hud: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    hud.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: px(28),
            left: px(28),
            width: px(285),
            padding: UiRect::all(px(17)),
            flex_direction: FlexDirection::Column,
            row_gap: px(8),
            ..default()
        },
        BackgroundColor(Color::srgba(0.025, 0.075, 0.085, 0.75)),
    ))
    .with_children(|panel| {
        panel.spawn((Item::Stats, text("", font, 20.0, CREAM)));
        for (index, label, color) in [
            (0, "КОРПУС", Color::srgb(0.4, 0.8, 0.65)),
            (1, "ЭНЕРГИЯ", GOLD),
            (2, "НАГРЕВ", Color::srgb(0.86, 0.39, 0.23)),
        ] {
            panel.spawn(text(label, font, 9.0, MUTED));
            panel
                .spawn((
                    Node {
                        width: percent(100),
                        height: px(3),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.4, 0.5, 0.5, 0.3)),
                ))
                .with_children(|bar| {
                    bar.spawn((
                        Item::Bar(index),
                        Node {
                            width: percent(100),
                            height: percent(100),
                            ..default()
                        },
                        BackgroundColor(color),
                    ));
                });
        }
    });
}

fn display(visible: bool) -> Display {
    if visible {
        Display::Flex
    } else {
        Display::None
    }
}

fn project(
    point: Vec3,
    camera: &Camera,
    transform: &GlobalTransform,
    window: &Window,
) -> Option<Vec2> {
    let size = camera.logical_viewport_size()?;
    camera
        .world_to_viewport(transform, point)
        .ok()
        .map(|p| p / size * Vec2::new(window.width(), window.height()))
}

pub(super) fn update(
    pilot: Res<Pilot>,
    mission: Res<Mission>,
    threats: Res<Threats>,
    settings: Res<ViewSettings>,
    camera: Query<(&Camera, &GlobalTransform), With<FlightCamera>>,
    windows: Query<&Window>,
    mut items: Query<(&Item, Option<&mut Text>, &mut Node)>,
) {
    let body = &pilot.body;
    let target = mission.target();
    let delta = target - body.position;
    let desired = (-delta.x).atan2(-delta.z);
    let bearing = (desired - body.yaw + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU)
        - std::f32::consts::PI;
    for (item, label, mut node) in &mut items {
        let value = match item {
            Item::Root => {
                node.display = display(!settings.photo);
                None
            }
            Item::Intro => {
                node.display = display(mission.phase == Phase::Intro);
                None
            }
            Item::Hud => {
                node.display = display(mission.phase != Phase::Intro);
                None
            }
            Item::Overlay => {
                node.display = display(matches!(
                    mission.phase,
                    Phase::Paused | Phase::Won | Phase::Lost
                ));
                None
            }
            Item::Stats => Some(format!(
                "{:03.0} км/ч   {:03.0} м\n{} / 3  ДОСТАВЛЕНО",
                body.velocity.length() * 3.6,
                body.position.y + 62.0,
                mission.delivered
            )),
            Item::Objective => Some(objective(&mission)),
            Item::Compass => Some(format!(
                "{}  {:03.0}°    {:.0} м",
                if bearing.abs() < 0.1 {
                    "ПРЯМО"
                } else if bearing > 0.0 {
                    "< ЛЕВЕЕ"
                } else {
                    "ПРАВЕЕ >"
                },
                bearing.to_degrees().abs(),
                delta.length()
            )),
            Item::Notice => {
                node.display = display(mission.notice_left > 0.0);
                Some(mission.notice.clone())
            }
            Item::Threat => Some(threat_status(&mission, &pilot, &threats)),
            Item::Target => {
                if let (Ok((camera, global)), Ok(window)) = (camera.single(), windows.single()) {
                    let point = project(target, camera, global, window).unwrap_or(Vec2::new(
                        if bearing > 0.0 {
                            55.0
                        } else {
                            window.width() - 55.0
                        },
                        window.height() * 0.45,
                    ));
                    node.left = px(point.x.clamp(100.0, window.width() - 100.0) - 110.0);
                    node.top = px(point.y.clamp(120.0, window.height() - 180.0));
                }
                Some(if delta.length() < 19.0 {
                    if mission.payload.is_some() {
                        "[E]  СДАТЬ ГРУЗ"
                    } else {
                        "[E]  ПОДЦЕПИТЬ"
                    }
                    .into()
                } else {
                    format!(
                        "◇  {}\n{:.0} м",
                        if mission.payload.is_some() {
                            "ГАВАНЬ"
                        } else {
                            "МАЯК"
                        },
                        delta.length()
                    )
                })
            }
            Item::Crosshair => {
                if let (Ok((camera, global)), Ok(window)) = (camera.single(), windows.single()) {
                    if let Some(point) = project(
                        body.position + body.forward() * 180.0,
                        camera,
                        global,
                        window,
                    ) {
                        node.left = px(point.x - 7.0);
                        node.top = px(point.y - 14.0);
                    }
                }
                None
            }
            Item::Bar(index) => {
                node.width = percent(match index {
                    0 => body.hull,
                    1 => body.energy,
                    _ => body.heat,
                });
                None
            }
            Item::OverlayTitle => Some(menus::heading(&mission)),
            Item::OverlayBody => Some(menus::body(&mission, &pilot, &threats)),
            Item::OverlayButton => Some(
                if matches!(mission.phase, Phase::Won | Phase::Lost) {
                    "НОВАЯ ЭКСПЕДИЦИЯ [ENTER]"
                } else {
                    "ПРОДОЛЖИТЬ [ENTER]"
                }
                .into(),
            ),
        };
        if let (Some(value), Some(mut text)) = (value, label) {
            if text.0 != value {
                text.0 = value;
            }
        }
    }
}

fn objective(mission: &Mission) -> String {
    if let Some(payload) = &mission.payload {
        format!(
            "В ГАВАНЬ  /  {}\n{} кг  ·  трос {:.1} м",
            RELICS[payload.relic].name, payload.mass, payload.length
        )
    } else if mission.delivered < 3 {
        format!(
            "0{}  /  {}\nЗабери ядро у маяка",
            mission.delivered + 1,
            RELICS[mission.delivered].name
        )
    } else {
        "ЭКСПЕДИЦИЯ ЗАВЕРШЕНА".into()
    }
}

fn threat_status(mission: &Mission, pilot: &Pilot, threats: &Threats) -> String {
    if threats
        .hunters
        .iter()
        .any(|h| h.health > 0.0 && h.alert > 0.2)
    {
        "ТЕПЛОВОЙ СЛЕД ОБНАРУЖЕН\nF — импульс   CTRL — щит\nОстывай и уходи за скалы".into()
    } else if pilot.body.heat > 75.0 {
        "ПЕРЕГРЕВ\nОтпусти форсаж".into()
    } else {
        format!(
            "ВЕТЕР  ЗАПАДНЫЙ\nФРОНТ БУРИ  {:02.0}%",
            mission.storm * 100.0
        )
    }
}
