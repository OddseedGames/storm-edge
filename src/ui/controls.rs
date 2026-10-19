use super::*;

pub(super) fn buttons(
    mut buttons: Query<
        (&Interaction, &Action, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut mission: ResMut<Mission>,
    mut pilot: ResMut<Pilot>,
    mut threats: ResMut<Threats>,
    mut trails: ResMut<Trails>,
) {
    for (interaction, action, mut color) in &mut buttons {
        color.0 = if *interaction == Interaction::Hovered {
            GOLD
        } else {
            CREAM
        };
        if *interaction != Interaction::Pressed {
            continue;
        }
        match action {
            Action::Start if mission.phase == Phase::Intro => {
                mission.phase = Phase::Flying;
                mission.announce("Первый маяк слева по курсу. Пробел / C — высота. E — подцепить.");
            }
            Action::Continue if mission.phase == Phase::Paused => {
                mission.phase = Phase::Flying;
                mission.show_help = false;
            }
            Action::Continue if matches!(mission.phase, Phase::Won | Phase::Lost) => {
                let muted = mission.muted;
                *mission = Mission::default();
                mission.muted = muted;
                *pilot = Pilot::default();
                *threats = Threats::default();
                *trails = Trails::default();
            }
            _ => {}
        }
    }
}
