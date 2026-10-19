use super::*;

pub(super) fn intro(root: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    root.spawn((
        Item::Intro,
        Node {
            position_type: PositionType::Absolute,
            left: percent(4),
            top: percent(12),
            width: px(455),
            padding: UiRect::all(px(30)),
            flex_direction: FlexDirection::Column,
            row_gap: px(18),
            border: UiRect::left(px(2)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.025, 0.075, 0.085, 0.84)),
        BorderColor::all(Color::srgba(0.9, 0.7, 0.4, 0.7)),
    )).with_children(|card| {
        card.spawn(text("O D D S E E D   /   Э К С П Е Д И Ц И Я  0 5", font, 11.0, GOLD));
        card.spawn(text("КРОМКА\nБУРИ", font, 66.0, CREAM));
        card.spawn(text("Три сердца. Один катер.\nНебо, которое не прощает спешки.", font, 15.0, CREAM));
        card.spawn(text(
            "Пройди сквозь облачный архипелаг.\nПодцепи древние ядра и верни их в гавань.\nБереги груз — и оставь энергию на побег.",
            font, 12.0, MUTED,
        ));
        button(card, font, Action::Start, "В ПОЛЁТ  [ENTER]", false);
        card.spawn(text(
            "W / S  тяга и тормоз    A / D  поворот\nПРОБЕЛ / C  высота     E  подцепить груз\nSHIFT  форсаж         H  управление",
            font, 11.0, MUTED,
        ));
    });
}

fn button(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    action: Action,
    label: &str,
    dynamic: bool,
) {
    parent
        .spawn((
            action,
            Button,
            Node {
                width: percent(100),
                padding: UiRect::all(px(16)),
                margin: UiRect::top(px(8)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(CREAM),
        ))
        .with_children(|button| {
            let mut label = button.spawn(text(label, font, 15.0, Color::srgb(0.05, 0.12, 0.13)));
            if dynamic {
                label.insert(Item::OverlayButton);
            }
        });
}

pub(super) fn overlay(root: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    root.spawn((
        Item::Overlay,
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.01, 0.035, 0.05, 0.7)),
    ))
    .with_children(|overlay| {
        overlay
            .spawn((
                Node {
                    width: px(650),
                    padding: UiRect::all(px(36)),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(22),
                    border: UiRect::top(px(2)),
                    ..default()
                },
                BorderColor::all(GOLD),
                BackgroundColor(Color::srgba(0.03, 0.09, 0.1, 0.96)),
            ))
            .with_children(|card| {
                card.spawn((Item::OverlayTitle, text("", font, 34.0, CREAM)));
                card.spawn((Item::OverlayBody, text("", font, 13.0, MUTED)));
                button(card, font, Action::Continue, "ПРОДОЛЖИТЬ [ENTER]", true);
                card.spawn(text(
                    "R  начать заново     M  звук     F2  качество графики",
                    font,
                    11.0,
                    MUTED,
                ));
            });
    });
}

pub(super) fn heading(mission: &Mission) -> String {
    match mission.phase {
        Phase::Won => "ГОРОД ВСТРЕТИТ РАССВЕТ",
        Phase::Lost => "ПОЛЁТ ПРЕРВАН",
        _ if mission.show_help => "УПРАВЛЕНИЕ КАТЕРОМ",
        _ => "ТИШИНА ПЕРЕД БУРЕЙ",
    }
    .into()
}

pub(super) fn body(mission: &Mission, pilot: &Pilot, threats: &Threats) -> String {
    match mission.phase {
        Phase::Won => format!(
            "Все три древних сердца доставлены в гавань.\n\nВремя: {:02}:{:02}    Стражей сбито: {}\nКорпус: {:.0}%\n\nСпасибо за полёт.",
            mission.elapsed as u32 / 60, mission.elapsed as u32 % 60, threats.downed, pilot.body.hull,
        ),
        Phase::Lost => "Катер потерян в облаках.\n\nТормози перед скалами и подхватом груза.\nБереги энергию для щита и форсажа.\nПовреждения чинятся после доставки в гавань.".into(),
        _ => concat!(
            "W — полная тяга       S — воздушный тормоз\n",
            "A / D — поворот       ПРОБЕЛ / C — вверх / вниз\n",
            "SHIFT — форсаж        CTRL — защитное поле\n",
            "E — подцепить / сдать F / левая мышь — импульс\n",
            "Q / Z — короче / длиннее трос\n",
            "X — оставить груз на аварийном поплавке\n\n",
            "Без W катер держит крейсерскую тягу.\n",
            "Груз раскачивается и тянет катер на поворотах.\n",
            "Форсаж нагревает двигатели и привлекает стражей.\n",
            "Щит и пушка расходуют общую энергию.\n",
            "Для подхвата и доставки затормози клавишей S.",
        ).into(),
    }
}
