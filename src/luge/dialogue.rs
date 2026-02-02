use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{actions::GameAction, player::Player};

#[derive(Component)]
pub(crate) struct RickDialogue;

#[derive(Resource)]
pub(crate) struct DialogueState {
    pub(crate) current_scene: SceneId,
    pub(crate) line_index: usize,
    pub(crate) waiting_for_input: bool,
}

impl Default for DialogueState {
    fn default() -> Self {
        Self {
            current_scene: SceneId::Intro,
            line_index: 0,
            waiting_for_input: true,
        }
    }
}

#[derive(Resource)]
pub(crate) struct RickLines(Vec<Scene>);

impl RickLines {
    pub(crate) fn init() -> Self {
        Self(vec![
            Scene {
                id: SceneId::Intro,
                lines: vec![
                    "Heya chump--errr, champ. Heh heh. \
                    Welcome ta Slick Rick's Luge Lounge. \
                    Da numbah one luge lounge in da lesser tri-state region."
                        .to_string(),
                    "Take a seat. Bob a sled. Spend some moolah. \
                    Su money es mi money, amigo. Capice?"
                        .to_string(),
                    "See dem stats der on da right? You can raise em by \
                    spendin Slick Coins, see?"
                        .to_string(),
                    "Oh, ya ain't got none? No problemo, sonny. Just head \
                    on down da luge. You'll find plenty along da way."
                        .to_string(),
                    "Just watch out for.... erm... OBSTACLES let's say. \
                        Roadblocks, if you will. Especially da ones wit da claws..."
                        .to_string(),
                ],
                completed: false,
            },
            Scene {
                id: SceneId::Shop,
                lines: vec![],
                completed: false,
            },
        ])
    }

    fn get_scene(&self, id: SceneId) -> Option<&Scene> {
        self.0.iter().find(|s| s.id == id)
    }

    fn get_scene_mut(&mut self, id: SceneId) -> Option<&mut Scene> {
        self.0.iter_mut().find(|s| s.id == id)
    }

    pub(crate) fn get_line(&self, id: SceneId, index: usize) -> Option<&str> {
        self.get_scene(id)?.lines.get(index).map(|s| s.as_str())
    }
}

struct Scene {
    id: SceneId,
    lines: Vec<String>,
    completed: bool,
}

#[derive(PartialEq, Copy, Clone)]
pub(crate) enum SceneId {
    Intro,
    Shop,
}

impl SceneId {
    fn next(self) -> Option<Self> {
        match self {
            Self::Intro => Some(Self::Shop),
            Self::Shop => None,
        }
    }
}

pub(super) fn advance_dialogue(
    mut dialogue_state: ResMut<DialogueState>,
    mut rick_lines: ResMut<RickLines>,
    mut rick_text: Single<&mut Text, With<RickDialogue>>,
    action_state: Single<&ActionState<GameAction>, With<Player>>,
) {
    if !dialogue_state.waiting_for_input {
        return;
    }

    if action_state.just_pressed(&GameAction::Continue) {
        dialogue_state.line_index += 1;

        if let Some(line) =
            rick_lines.get_line(dialogue_state.current_scene, dialogue_state.line_index)
        {
            rick_text.0 = line.to_string();
        } else {
            // Mark current scene completed
            if let Some(scene) = rick_lines.get_scene_mut(dialogue_state.current_scene) {
                scene.completed = true;
            }

            // Advance to next scene or finish dialogue
            if let Some(next_scene) = dialogue_state.current_scene.next() {
                dialogue_state.current_scene = next_scene;
                dialogue_state.line_index = 0;
                if let Some(line) = rick_lines.get_line(next_scene, 0) {
                    rick_text.0 = line.to_string();
                    return;
                }
                if let Some(scene) = rick_lines.get_scene_mut(next_scene) {
                    scene.completed = true;
                }
            }
            dialogue_state.waiting_for_input = false;
            rick_text.0 = String::new();
        }
    }
}
