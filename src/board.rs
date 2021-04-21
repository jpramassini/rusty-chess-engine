use bevy::{app::AppExit, prelude::*};
use bevy_mod_picking::*;

use crate::pieces::*;

pub struct PlayerTurn(pub PieceColor);
impl Default for PlayerTurn {
    fn default() -> Self {
        Self(PieceColor::White)
    }
}

impl PlayerTurn {
    fn change(&mut self) {
        self.0 = match self.0 {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White
        }
    }
}

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .init_resource::<PlayerTurn>()
            .add_event::<ResetSelectedEvent>()
            .add_startup_system(create_board.system())
            .add_system(color_squares.system())
            .add_system(select_square.system())
            .add_system(move_piece.system())
            .add_system(select_piece.system())
            .add_system(despawn_taken_pieces.system())
            .add_system(reset_selected.system());
    }
}

pub struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}
#[derive(Default)]
struct SelectedSquare {
    entity: Option<Entity>,
}
#[derive(Default)]
struct SelectedPiece {
    entity: Option<Entity>,
}

fn create_board(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1.0 }));

    for i in 0..8 {
        for j in 0..8 {
            commands
                .spawn(PbrBundle {
                    mesh: mesh.clone(),
                    material: if (i + j + 1) % 2 == 0 {
                        materials.add(Color::rgb(1.0, 0.9, 0.9).into())
                    } else {
                        materials.add(Color::rgb(0.0, 0.1, 0.1).into())
                    },
                    transform: Transform::from_translation(Vec3::new(i as f32, 0.0, j as f32)),
                    ..Default::default()
                })
                .with(PickableMesh::default())
                .with(Square { x: i, y: j });
        }
    }
}

/*
NOTE: Query is a bevy ECS thing which gives you access to all of the entities which have the components you specify.
In this case, all of the entities that make up the board have Square and Handle for Standard material on
them, so this gets those for us. In queries, components have to be references, hence the & in the signature.
 */

// Change color of squares according to selected squares
fn color_squares(
    pick_state: Res<PickState>,
    selected_square: Res<SelectedSquare>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Square, &Handle<StandardMaterial>)>,
) {
    // Get the entity under the cursor, if it exists.
    let top_entity = if let Some((entity, _intersection)) = pick_state.top(Group::default()) {
        Some(*entity)
    } else {
        None
    };

    for (entity, square, material_handle) in query.iter() {
        // Grab the actual material
        let material = materials.get_mut(material_handle).unwrap();

        // Change the material color
        material.albedo = if Some(entity) == top_entity {
            // If hovered
            Color::rgb(0.8, 0.3, 0.3)
        } else if Some(entity) == selected_square.entity {
            // If selected
            Color::rgb(0.9, 0.1, 0.1)
        } else if square.is_white() {
            // If white square
            Color::rgb(1.0, 0.9, 0.9)
        } else {
            // If black square
            Color::rgb(0.0, 0.1, 0.1)
        };
    }
}

fn select_square(
    pick_state: Res<PickState>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    squares_query: Query<&Square>
) {
    // Only run if left is pressed
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    // Get square under cursor, set as selected
    if let Some((square_entity, _intersection)) = pick_state.top(Group::default()) {
        // Get ref to actual square, making sure it exists and is a square.
        if let Ok(_square) = squares_query.get(*square_entity) {
            // Mark square as selected
            selected_square.entity = Some(*square_entity);
        }
    } else {
        // Player clicked outside the board, deselect everything
        selected_square.entity = None;
        selected_piece.entity = None;
    }
}

fn select_piece(
    selected_square: ChangedRes<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    turn: Res<PlayerTurn>,
    squares_query: Query<&Square>,
    pieces_query: Query<(Entity, &Piece)>
) {
    let square_entity = if let Some(entity) = selected_square.entity {
        entity
    } else {
        return;
    };

    let square = if let Ok(square) = squares_query.get(square_entity) {
        square
    } else {
        return;
    };

    if selected_piece.entity.is_none() {
        // Select the piece in the currently selected square
        for (piece_entity, piece) in pieces_query.iter() {
            if piece.x == square.x && piece.y == square.y && piece.color == turn.0 {
                selected_piece.entity = Some(piece_entity);
                break;
            }
        }
    }
}

fn move_piece(
    commands: &mut Commands,
    selected_square: ChangedRes<SelectedSquare>,
    selected_piece: Res<SelectedPiece>,
    mut turn: ResMut<PlayerTurn>,
    squares_query: Query<&Square>,
    mut pieces_query: Query<(Entity, &mut Piece)>,
    mut reset_selected_event: ResMut<Events<ResetSelectedEvent>>
) {
    let square_entity = if let Some(entity) = selected_square.entity {
        entity
    } else {
        return;
    };

    let square = if let Ok(square) = squares_query.get(square_entity) {
        square
    } else {
        return;
    };

    if let Some(selected_piece_entity) = selected_piece.entity {
        let pieces_vec = pieces_query.iter_mut().map(|(_, piece)| *piece).collect();
        let pieces_entity_vec = pieces_query.iter_mut()
        .map(|(entity, piece)| (entity, *piece))
        .collect::<Vec<(Entity, Piece)>>();

        // Move the selected piece to the selected square
        let mut piece =
            if let Ok((_piece_entity, piece)) = pieces_query.get_mut(selected_piece_entity) {
                piece
            } else {
                return;
            };

        if piece.is_move_valid((square.x, square.y), pieces_vec) {
            // Check if a piece of the opposite color exists in this square and despawn it
            for (other_entity, other_piece) in pieces_entity_vec {
                if other_piece.x == square.x
                    && other_piece.y == square.y
                    && other_piece.color != piece.color {
                        commands.insert_one(other_entity, Taken);
                    }
            }

            // Move piece
            piece.x = square.x;
            piece.y = square.y;

            // Change turn
            turn.change();
        }

        reset_selected_event.send(ResetSelectedEvent);
    }
}

struct ResetSelectedEvent;

fn reset_selected(
    mut event_reader: Local<EventReader<ResetSelectedEvent>>,
    events: Res<Events<ResetSelectedEvent>>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>
) {
    for _event in event_reader.iter(&events) {
        selected_square.entity = None;
        selected_piece.entity = None;
    }
}

struct Taken;
fn despawn_taken_pieces(
    commands: &mut Commands,
    mut app_exit_events: ResMut<Events<AppExit>>,
    query: Query<(Entity, &Piece, &Taken)>,
) {
    for (entity, piece, _taken) in query.iter() {
        // If the kind is taken, we should exit
        if piece.piece_type == PieceType::King {
            println!(
                "{} won! Thanks for playing!",
                match piece.color {
                    PieceColor::White => "Black",
                    PieceColor::Black => "White"
                }
            );
            app_exit_events.send(AppExit);
        }

        commands.despawn_recursive(entity);
    }
}
