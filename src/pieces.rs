use bevy::prelude::*;

use crate::engine::*;

pub struct PiecesPlugin;
impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_pieces.system())
            .init_resource::<PieceMetadata>()
            .add_system(move_pieces.system());
    }
}

pub struct PieceMetadata {
    pub piece_is_animating: bool
}

impl Default for PieceMetadata {
    fn default() -> Self {
        PieceMetadata {
            piece_is_animating: false
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
    // Current position
    pub x: u8,
    pub y: u8,
}

impl Piece {
    pub fn is_move_valid(&self, new_position: (u8, u8), pieces: Vec<Piece>) -> bool {
        // If there's a piece of the same color in the same square, it can't move
        if color_of_square(new_position, &pieces) == Some(self.color) {
            return false;
        }

        match self.piece_type {
            PieceType::King => {
                // Horizontal Move
                return ((self.x as i8 - new_position.0 as i8).abs() == 1 && (self.y == new_position.1))
                // Vertical Move
                || ((self.y as i8 - new_position.1 as i8).abs() == 1 && (self.x == new_position.0))
                // Diagonal
                || ((self.x as i8 - new_position.0 as i8).abs() == 1 && (self.y as i8 - new_position.1 as i8).abs() == 1);
            }
            PieceType::Queen => {
                return is_path_empty((self.x, self.y), new_position, &pieces)
                    && ((self.x as i8 - new_position.0 as i8).abs()
                        == (self.y as i8 - new_position.1 as i8).abs())
                    || ((self.x == new_position.0 && self.y != new_position.1)
                        || (self.x != new_position.0 && self.y == new_position.1));
            }
            PieceType::Bishop => {
                return is_path_empty((self.x, self.y), new_position, &pieces)
                    && (self.x as i8 - new_position.0 as i8).abs()
                        == (self.y as i8 - new_position.1 as i8).abs()
            }
            PieceType::Knight => {
                return ((self.x as i8 - new_position.0 as i8).abs() == 2
                    && (self.y as i8 - new_position.1 as i8).abs() == 1)
                    || ((self.x as i8 - new_position.0 as i8).abs() == 1
                        && (self.y as i8 - new_position.1 as i8).abs() == 2)
            }
            PieceType::Rook => {
                return is_path_empty((self.x, self.y), new_position, &pieces)
                    && ((self.x == new_position.0 && self.y != new_position.1)
                        || (self.y == new_position.1 && self.x != new_position.0))
            }
            PieceType::Pawn => {
                if self.color == PieceColor::White {    // For White Pieces
                    // Normal move
                    if new_position.0 as i8 - self.x as i8 == 1 && (self.y == new_position.1) {
                        if color_of_square(new_position, &pieces).is_none() {
                            return true;
                        }
                    }
                    // Two moves
                    if self.x == 1
                        && new_position.0 as i8 - self.x as i8 == 2
                        && (self.y == new_position.1)
                        && is_path_empty((self.x, self.y), new_position, &pieces)
                    {
                        if color_of_square(new_position, &pieces).is_none() {
                            return true;
                        }
                    }
                    // Take a piece
                    if new_position.0 as i8 - self.x as i8 == 1
                        && (self.y as i8 - new_position.1 as i8).abs() == 1
                    {
                        if color_of_square(new_position, &pieces) == Some(PieceColor::Black) {
                            return true;
                        }
                    }
                } else {    // For Black Pieces
                    // Normal move
                    if new_position.0 as i8 - self.x as i8 == -1 && (self.y == new_position.1) {
                        if color_of_square(new_position, &pieces).is_none() {
                            return true;
                        }
                    }
                    // Two moves
                    if self.x == 6
                        && new_position.0 as i8 - self.x as i8 == -2
                        && (self.y == new_position.1)
                        && is_path_empty((self.x, self.y), new_position, &pieces)
                    {
                        if color_of_square(new_position, &pieces).is_none() {
                            return true;
                        }
                    }
                    // Take a piece
                    if new_position.0 as i8 - self.x as i8 == -1
                        && (self.y as i8 - new_position.1 as i8).abs() == 1
                    {
                        if color_of_square(new_position, &pieces) == Some(PieceColor::White) {
                            return true;
                        }
                    }
                }
            }
        }
        println!("Invalid move.");
        return false;
    }
}

fn move_pieces(time: Res<Time>, mut piece_metadata: ResMut<PieceMetadata>, mut query: Query<(&mut Transform, &Piece)>) {
    for (mut transform, piece) in query.iter_mut() {
        // Get direction to move in
        let direction = Vec3::new(piece.x as f32, 0.0, piece.y as f32) - transform.translation;

        // Only move if the piece isn't already there (distance is big)
        if direction.length() > 0.1 {
            piece_metadata.piece_is_animating = true;
            println!("Piece is animating: {}", piece_metadata.piece_is_animating);
            transform.translation += direction.normalize() * time.delta_seconds() * 3.0;
        } else {
            piece_metadata.piece_is_animating = false;
            println!("Piece is animating: {}", piece_metadata.piece_is_animating);
        }
    }
}

fn color_of_square(pos: (u8, u8), pieces: &Vec<Piece>) -> Option<PieceColor> {
    for piece in pieces {
        if piece.x == pos.0 && piece.y == pos.1 {
            return Some(piece.color);
        }
    }
    None
}

fn is_path_empty(begin: (u8, u8), end: (u8, u8), pieces: &Vec<Piece>) -> bool {
    // Same column
    if begin.0 == end.0 {
        for piece in pieces {
            if piece.x == begin.0
                && ((piece.y > begin.1 && piece.y < end.1)
                    || (piece.y > end.1 && piece.y < begin.1))
            {
                return false;
            }
        }
    }
    // Same row
    if begin.1 == end.1 {
        for piece in pieces {
            if piece.y == begin.1
                && ((piece.x > begin.0 && piece.x < end.0)
                    || (piece.x > end.0 && piece.x < begin.0))
            {
                return false;
            }
        }
    }

    // Diagonals
    let x_diff = (begin.0 as i8 - end.0 as i8).abs();
    let y_diff = (begin.1 as i8 - end.1 as i8).abs();
    if x_diff == y_diff {
        for i in 1..x_diff {
            let pos = if begin.0 < end.0 && begin.1 < end.1 {
                // left bottom - right top
                (begin.0 + i as u8, begin.1 + i as u8)
            } else if begin.0 < end.0 && begin.1 > end.1 {
                // left top - right bottom
                (begin.0 + i as u8, begin.1 - i as u8)
            } else if begin.0 > end.0 && begin.1 < end.1 {
                // right bottom - left top
                (begin.0 - i as u8, begin.1 + i as u8)
            } else {
                // begin.0 > end.0 && begin.1 > end.1
                // right top - left bottom
                (begin.0 - i as u8, begin.1 - i as u8)
            };

            if color_of_square(pos, pieces).is_some() {
                return false;
            }
        }
    }

    true
}

// TODO: Implement premptively finding all valid moves for a piece and then highlighting them for the player.

fn create_pieces(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load all the meshes
    let king_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh0/Primitive0");
    let king_cross_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh1/Primitive0");
    let pawn_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh2/Primitive0");
    let knight_1_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh3/Primitive0");
    let knight_2_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh4/Primitive0");
    let rook_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh5/Primitive0");
    let bishop_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh6/Primitive0");
    let queen_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh7/Primitive0");

    let white_material = materials.add(Color::rgb(1.0, 0.8, 0.8).into());
    let black_material = materials.add(Color::rgb(0.0, 0.2, 0.2).into());

    // NOTE for Vec3 Positions: X is the rank, Z is the file.

    let mut board: Board = Board {
        squares: [Space {piece: None}; 64],
    };

    parse_fen_string_to_board(
        OTHER_OTHER_TEST_FEN,
        &mut board,
    );
    
    for (index, square) in board.squares.iter().enumerate() {
        let position = index_to_rank_and_file(index);
        if square.piece.is_none(){
            continue;
        }
        let piece = square.piece.unwrap();

        let piece_color = piece.piece_color;
        let material = if piece.piece_color == PieceColor::White {white_material.clone()} else {black_material.clone()};
        match piece.piece_type {
            PieceType::Pawn => 
                spawn_pawn(commands, material, piece.piece_color, pawn_handle.clone(), position),
            PieceType::Bishop => spawn_bishop(commands, material, piece_color, bishop_handle.clone(), position),
            PieceType::King => spawn_king(commands, material, piece_color, king_handle.clone(), king_cross_handle.clone(), position),
            PieceType::Knight => spawn_knight(commands, material, piece_color, knight_1_handle.clone(), knight_2_handle.clone(), position),
            PieceType::Queen => spawn_queen(commands, material, piece_color, queen_handle.clone(), position),
            PieceType::Rook => spawn_rook(commands, material, piece_color, rook_handle.clone(), position)
        }
    }
}

fn spawn_king(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    mesh_cross: Handle<Mesh>,
    position: (u8, u8),
) {
    commands
        // Spawn parent entity
        .spawn(PbrBundle {
            transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.0,
                position.1 as f32,
            )),
            ..Default::default()
        })
        .with(Piece {
            color: piece_color,
            piece_type: PieceType::King,
            x: position.0,
            y: position.1,
        })
        // Add children to the parent
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material: material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
            parent.spawn(PbrBundle {
                mesh: mesh_cross,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}

fn spawn_knight(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh_1: Handle<Mesh>,
    mesh_2: Handle<Mesh>,
    position: (u8, u8),
) {
    commands
        // Spawn parent entity
        .spawn(PbrBundle {
            transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.0,
                position.1 as f32,
            )),
            ..Default::default()
        })
        .with(Piece {
            color: piece_color,
            piece_type: PieceType::Knight,
            x: position.0,
            y: position.1,
        })
        // Add children to the parent
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: mesh_1,
                material: material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 0.9));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
            parent.spawn(PbrBundle {
                mesh: mesh_2,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 0.9));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}

fn spawn_queen(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    position: (u8, u8),
) {
    commands
        .spawn(PbrBundle {
            transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.0,
                position.1 as f32,
            )),
            ..Default::default()
        })
        .with(Piece {
            color: piece_color,
            piece_type: PieceType::Queen,
            x: position.0,
            y: position.1,
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -0.95));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}

fn spawn_bishop(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    position: (u8, u8),
) {
    commands
        .spawn(PbrBundle {
            transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.0,
                position.1 as f32,
            )),
            ..Default::default()
        })
        .with(Piece {
            color: piece_color,
            piece_type: PieceType::Bishop,
            x: position.0,
            y: position.1,
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.1, 0., 0.));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}

fn spawn_rook(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    position: (u8, u8),
) {
    commands
        .spawn(PbrBundle {
            transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.0,
                position.1 as f32,
            )),
            ..Default::default()
        })
        .with(Piece {
            color: piece_color,
            piece_type: PieceType::Rook,
            x: position.0,
            y: position.1,
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.1, 0., 1.8));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}

fn spawn_pawn(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    position: (u8, u8),
) {
    commands
        .spawn(PbrBundle {
            transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.0,
                position.1 as f32,
            )),
            ..Default::default()
        })
        .with(Piece {
            color: piece_color,
            piece_type: PieceType::Pawn,
            x: position.0,
            y: position.1,
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 2.6));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}
